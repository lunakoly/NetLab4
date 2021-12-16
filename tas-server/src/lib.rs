pub mod messages;
pub mod parsing;
pub mod members;

use std::fs::{DirEntry};
use std::path::{Path, PathBuf};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::thread;


// use std::collections::{HashMap};
// use std::time::{Duration};
// use std::path::{Path};
// use std::fs::{File};

use crate::members::{load_members, save_members, Members, Role};
use crate::messages::{ClientMessage, ServerMessage};
use crate::parsing::{read_message};

use common::{Result, with_error_report};
use common::shared::{IntoShared, Shared};
use common::shared::vec::{SharedVec};

const DEFAULT_PORT: u32 = 6969;

struct UserData {
    name: String,
    location: PathBuf,
    // address: SocketAddr,
}

type User = Shared<UserData>;

fn empty_user() -> Result<User> {
    let data = UserData {
        name: format!("guest"),
        location: std::env::current_dir()?,
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
}

fn notify(message: &str, stream: &mut TcpStream) -> Result<()> {
    let it = ServerMessage::Notification {
        message: message.to_owned()
    };

    let packet = parsing::to_bytes(&it)?;
    stream.write_all(&packet)?;
    Ok(())
}

fn handle_login(
    command: Vec<String>,
    session: &mut Session
) -> Result<()> {
    if command.len() < 3 {
        return notify("The command misses some parameters", &mut session.stream)
    }

    let name = &command[1];
    let pass = &command[2];

    if !session.context.members.read()?.has_user(name) {
        return notify(&format!("No such a user > {}", name), &mut session.stream)
    }

    let settings = session.context.members.read()?.settings_for(name)?;

    if pass != &settings.pass {
        return notify("Incorrect password", &mut session.stream)
    }

    session.me.write()?.name = name.clone();

    let role = session.context.members.read()?.role_for(name)?;

    let role_message = ServerMessage::Role {
        title: role.title,
        allowed_commands: role.allowed_commands,
    };

    let packet = parsing::to_bytes(&role_message)?;

    session.stream.write_all(&packet)?;

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
    let packet = parsing::to_bytes(&message)?;

    session.stream.write_all(&packet)?;

    Ok(())
}

fn handle_cd(
    command: Vec<String>,
    session: &mut Session
) -> Result<()> {
    if command.len() < 2 {
        return notify("The command misses some parameters", &mut session.stream)
    }

    let target = &command[1];
    let target_path = Path::new(target);

    if !target_path.exists() {
        return notify("No such a path", &mut session.stream)
    }

    if !target_path.is_dir() {
        return notify("This is not a directory", &mut session.stream)
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
            return notify(&format!("Error > {:?}", error), &mut session.stream)
        }
    };

    session.me.write()?.location = normalized.clone();

    let message = ServerMessage::MoveTo {
        location: location_to_string(&normalized)?
    };

    let packet = parsing::to_bytes(&message)?;

    session.stream.write_all(&packet)?;

    Ok(())
}

fn handle_who(
    session: &mut Session
) -> Result<()> {
    let mut users = vec![];

    let users_lock = session.context.users.read()?;

    for it in users_lock.iter() {
        let name = it.read()?.name.clone();
        let location = it.read()?.location.clone();
        let string = location_to_string(&location)?;
        users.push((name, string));
    }

    let message = ServerMessage::UsersList { users };
    let packet = parsing::to_bytes(&message)?;

    session.stream.write_all(&packet)?;

    Ok(())
}

fn handle_execute(command: Vec<String>, session: &mut Session) -> Result<()> {
    if command.len() == 0 {
        return notify("Empty command", &mut session.stream)
    }

    let name = session.me.read()?.name.clone();
    let role = session.context.members.read()?.role_for(&name)?;

    println!("Com > {:?}", &command);
    println!("> {:?}", &role);

    if !role.allowed_commands.contains(&command[0]) {
        return notify("No such an allowed command for you", &mut session.stream)
    }

    match command[0].as_ref() {
        "login" => handle_login(command, session),
        "ls" => handle_ls(session),
        "cd" => handle_cd(command, session),
        "who" => handle_who(session),
        it => notify(&format!("No such a command > {}", it), &mut session.stream)
    }
}

enum ClientHandling {
    Proceed,
    Stop,
}

fn handle_client(session: &mut Session) -> Result<ClientHandling> {
    let result: Result<ClientMessage> = read_message(&mut session.stream);

    let message = match result {
        Ok(message) => message,
        Err(error) => {
            println!("Disconnecting the client {:?} > {}", &mut session.stream.peer_addr(), &error);
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

    let packet = parsing::to_bytes(&message)?;

    session.stream.write_all(&packet)?;
    Ok(())
}

fn send_initial_location(session: &mut Session) -> Result<()> {
    let directory = session.me.read()?.location.clone();

    let location = match directory.to_str() {
        Some(thing) => thing.to_owned(),
        None => return Ok(())
    };

    let message = ServerMessage::MoveTo { location };
    let packet = parsing::to_bytes(&message)?;

    session.stream.write_all(&packet)?;
    Ok(())
}

fn handle_incomming(mut session: Session) -> Result<()> {
    send_initial_role(&mut session)?;
    send_initial_location(&mut session)?;

    session.context.users.write()?.push(session.me.clone());

    loop {
        let handling = handle_client(&mut session)?;

        if let ClientHandling::Stop = handling {
            break;
        }
    }

    Ok(())
}

fn handle_connection() -> Result<()> {
    let context = Context {
        users: vec![].to_shared(),
        members: load_members()?.to_shared(),
    };

    let listener = TcpListener::bind(format!("0.0.0.0:{}", DEFAULT_PORT))?;

    for incomming in listener.incoming() {
        let session = Session {
            me: empty_user()?,
            context: context.clone(),
            stream: incomming?,
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
