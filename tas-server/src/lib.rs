pub mod messages;
pub mod members;

use std::path::{Path, PathBuf};
use std::net::{TcpListener, TcpStream};
use std::time::{Duration};
use std::io::{Write};
use std::thread;

use crate::members::{load_members, Members};
use crate::messages::{ClientMessage, ServerMessage};

use common::{
    Result,
    with_error_report,
    is_would_block_io_result,
    is_would_block_result,
};

use common::parsing::{self, read_message};
use common::shared::{IntoShared, Shared};
use common::shared::vec::{SharedVec};

const DEFAULT_PORT: u32 = 6969;

struct UserData {
    name: String,
    location: PathBuf,
    is_alive: bool,
}

type User = Shared<UserData>;

fn empty_user() -> Result<User> {
    let data = UserData {
        name: format!("guest"),
        location: std::env::current_dir()?,
        is_alive: true,
    };

    Ok(data.to_shared())
}

#[derive(Clone)]
struct Context {
    users: SharedVec<User>,
    members: Shared<Members>,
}

struct Session {
    me: User,
    context: Context,
    stream: TcpStream,
    message_queue: Vec<ServerMessage>,
}

fn force_send_packet(message: &ServerMessage, session: &mut Session) {
    session.message_queue.push(message.clone());
}

fn notify(message: &str, session: &mut Session) -> Result<()> {
    let it = ServerMessage::Notification {
        message: message.to_owned()
    };

    force_send_packet(&it, session);
    Ok(())
}

fn handle_login(
    command: Vec<String>,
    session: &mut Session
) -> Result<()> {
    if command.len() < 3 {
        return notify("The command misses some parameters", session)
    }

    let name = &command[1];
    let pass = &command[2];

    if !session.context.members.read()?.has_user(name) {
        return notify(&format!("No such a user > {}", name), session)
    }

    let settings = session.context.members.read()?.settings_for(name)?;

    if pass != &settings.pass {
        return notify("Incorrect password", session)
    }

    session.me.write()?.name = name.clone();

    let role = session.context.members.read()?.role_for(name)?;

    let message = ServerMessage::Role {
        title: role.title,
        allowed_commands: role.allowed_commands,
    };

    force_send_packet(&message, session);
    Ok(())
}

fn location_to_string(location: &PathBuf) -> Result<String> {
    match location.to_str() {
        Some(thing) => Ok(thing.to_owned().replace("\\\\?\\", "")),
        None => Ok("Unknown".to_owned())
    }
}

fn handle_ls(
    session: &mut Session
) -> Result<()> {
    let location = session.me.read()?.location.clone();
    let mut files = vec![];

    for it in std::fs::read_dir(location)? {
        match it?.path().to_str() {
            Some(thing) => files.push(thing.to_owned()),
            None => {}
        }
    }

    let message = ServerMessage::FilesList { files };

    force_send_packet(&message, session);
    Ok(())
}

fn handle_cd(
    command: Vec<String>,
    session: &mut Session
) -> Result<()> {
    if command.len() < 2 {
        return notify("The command misses some parameters", session)
    }

    let target = &command[1];
    let target_path = Path::new(target);

    if !target_path.exists() {
        return notify("No such a path", session)
    }

    if !target_path.is_dir() {
        return notify("This is not a directory", session)
    }

    let mut new_location: PathBuf;

    if target_path.is_absolute() {
        new_location = target_path.to_path_buf();
    } else {
        new_location = session.me.read()?.location.clone();
        new_location.push(target);
    }

    let normalized = match new_location.canonicalize() {
        Ok(it) => it,
        Err(error) => {
            return notify(&format!("Error > {:?}", error), session)
        }
    };

    session.me.write()?.location = normalized.clone();

    let message = ServerMessage::MoveTo {
        location: location_to_string(&normalized)?
    };

    force_send_packet(&message, session);

    Ok(())
}

fn collect_users(session: &mut Session) -> Result<Vec<(String, String)>> {
    let mut users = vec![];

    let users_lock = session.context.users.read()?;

    for it in users_lock.iter() {
        let name = it.read()?.name.clone();
        let location = it.read()?.location.clone();
        let string = location_to_string(&location)?;
        users.push((name, string));
    }

    Ok(users)
}

fn handle_who(
    session: &mut Session
) -> Result<()> {
    let users = collect_users(session)?;
    let message = ServerMessage::UsersList { users };

    force_send_packet(&message, session);
    Ok(())
}

fn are_locations_same(a: &PathBuf, b: &PathBuf) -> Result<bool> {
    let string_a = location_to_string(a)?;
    let string_b = location_to_string(b)?;
    Ok(string_a == string_b)
}

fn handle_kill(
    command: Vec<String>,
    session: &mut Session
) -> Result<()> {
    if command.len() < 2 {
        return notify("The command misses some parameters", session)
    }

    let target = &command[1];
    let mut count = 0;

    for it in session.context.users.read()?.iter() {
        let has_same_name = &it.read()?.name == target;
        let is_nearby = are_locations_same(&it.read()?.location, &session.me.read()?.location)?;

        if has_same_name && is_nearby {
            it.write()?.is_alive = false;
            count += 1;
        }
    }

    let message = ServerMessage::KillResult {
        killed_users_count: count as u32,
    };

    force_send_packet(&message, session);

    Ok(())
}

fn handle_execute(command: Vec<String>, session: &mut Session) -> Result<()> {
    if command.len() == 0 {
        return notify("Empty command", session)
    }

    let name = session.me.read()?.name.clone();
    let role = session.context.members.read()?.role_for(&name)?;

    if !role.allowed_commands.contains(&command[0]) {
        return notify("No such an allowed command for you", session)
    }

    match command[0].as_ref() {
        "login" => handle_login(command, session),
        "ls" => handle_ls(session),
        "cd" => handle_cd(command, session),
        "who" => handle_who(session),
        "kill" => handle_kill(command, session),
        it => notify(&format!("No such a command > {}", it), session)
    }
}

enum ClientHandling {
    Proceed,
    Stop,
}

fn process_message_queue(session: &mut Session) -> Result<()> {
    if session.message_queue.len() == 0 {
        return Ok(())
    }

    let first = &session.message_queue[0];
    let packet = parsing::to_bytes(first)?;

    let result = session.stream.write_all(&packet);

    if is_would_block_io_result(&result) {
        return Ok(())
    }

    result?;
    session.message_queue.remove(0);

    Ok(())
}

fn handle_client(session: &mut Session) -> Result<ClientHandling> {
    process_message_queue(session)?;

    let result: Result<ClientMessage> = read_message(&mut session.stream);

    if is_would_block_result(&result) {
        return Ok(ClientHandling::Proceed)
    }

    let message = match result {
        Ok(message) => message,
        Err(error) => {
            println!("Disconnecting the client {:?} > {}", session.stream.peer_addr(), &error);
            return Ok(ClientHandling::Stop)
        }
    };

    match message {
        ClientMessage::Execute { command } => handle_execute(command, session)?
    }

    Ok(ClientHandling::Proceed)
}

fn send_initial_role(session: &mut Session) -> Result<()> {
    let guest_role = session.context.members.read()?.role_for("guest")?;

    let message = ServerMessage::Role {
        title: guest_role.title,
        allowed_commands: guest_role.allowed_commands,
    };

    force_send_packet(&message, session);
    Ok(())
}

fn send_initial_location(session: &mut Session) -> Result<()> {
    let directory = session.me.read()?.location.clone();

    let location = match directory.to_str() {
        Some(thing) => thing.to_owned(),
        None => return Ok(())
    };

    let message = ServerMessage::MoveTo { location };

    force_send_packet(&message, session);
    Ok(())
}

fn handle_incomming(mut session: Session) -> Result<()> {
    send_initial_role(&mut session)?;
    send_initial_location(&mut session)?;

    session.context.users.write()?.push(session.me.clone());

    loop {
        if !session.me.read()?.is_alive {
            break
        }

        let handling = handle_client(&mut session)?;

        if let ClientHandling::Stop = handling {
            break;
        }

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn handle_connection() -> Result<()> {
    let context = Context {
        users: vec![].to_shared(),
        members: load_members()?.to_shared(),
    };

    let listener = TcpListener::bind(format!("0.0.0.0:{}", DEFAULT_PORT))?;
    listener.set_nonblocking(true)?;

    for incomming in listener.incoming() {
        if is_would_block_io_result(&incomming) {
            std::thread::sleep(Duration::from_millis(16));
            continue
        }

        let session = Session {
            me: empty_user()?,
            context: context.clone(),
            stream: incomming?,
            message_queue: vec![],
        };

        thread::spawn(move || {
            with_error_report(|| handle_incomming(session));
        });
    }

    Ok(())
}

pub fn start() {
    with_error_report(handle_connection);
}
