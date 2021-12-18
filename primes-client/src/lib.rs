pub mod chars_reader;
pub mod commands;
pub mod messages;

use std::sync::mpsc::{channel, Receiver};
use std::io::{Write, BufRead};
use std::net::{TcpStream};
use std::time::{Duration};

use crate::commands::{Command};
use crate::messages::{ClientMessage, ServerMessage};
use crate::chars_reader::{IntoCharsReader};

use common::{
    Result,
    with_error_report,
    is_would_block_io_result,
    is_would_block_result,
};

use common::parsing::{self, read_message};
use common::shared::{IntoShared, Shared};

pub const DEFAULT_PORT: u32 = 6969;

struct Session {
    stream: TcpStream,
    message_queue: Vec<ClientMessage>,
    requested_primes_count: u32,
    is_range_requested: bool,
    calculations_result: Shared<Option<Vec<u32>>>,
}

fn enqueue_packet(message: &ClientMessage, message_queue: &mut Vec<ClientMessage>) -> Result<()> {
    message_queue.push(message.clone());
    Ok(())
}

fn send_packet(message: &ClientMessage, session: &mut Session) -> Result<()> {
    enqueue_packet(message, &mut session.message_queue)
}

enum CommandProcessing {
    Ok,
    Stop,
    Connect(Session),
}

fn get_max_n_primes(count: u32, session: &mut Session) -> Result<CommandProcessing> {
    let it = ClientMessage::GetMaxNPrimes {
        count: count,
    };

    session.requested_primes_count = count;

    send_packet(&it, session)?;
    Ok(CommandProcessing::Ok)
}

fn connect(address: &str) -> Result<CommandProcessing> {
    let stream = TcpStream::connect(address)?;
    stream.set_nonblocking(true)?;

    let session = Session {
        stream: stream,
        message_queue: vec![],
        requested_primes_count: 0,
        is_range_requested: false,
        calculations_result: None.to_shared(),
    };

    Ok(CommandProcessing::Connect(session))
}

fn connect_with_checks(address: &str) -> Result<CommandProcessing> {
    let result = connect(address);

    match result {
        Ok(it) => Ok(it),
        Err(error) => {
            println!("(Console) Error while connecting > {:?}", error);
            Ok(CommandProcessing::Ok)
        }
    }
}

fn handle_command_with_session(command: &Command, session: &mut Session) -> Result<CommandProcessing> {
    match command {
        Command::GetMaxNPrimes { count } => get_max_n_primes(count.clone(), session),
        _ => Ok(CommandProcessing::Ok)
    }
}

fn notify_no_session() -> Result<CommandProcessing> {
    println!("(Console) You should first establish a connection");
    Ok(CommandProcessing::Ok)
}

fn handle_command(command: &Command, session: &mut Option<Session>) -> Result<CommandProcessing> {
    match command {
        Command::Nothing => Ok(CommandProcessing::Ok),
        Command::End => Ok(CommandProcessing::Stop),
        Command::Connect { address } => connect_with_checks(address),
        _ => match session {
            Some(it) => handle_command_with_session(command, it),
            None => notify_no_session()
        }
    }
}

fn process_message_queue(session: &mut Session) -> Result<()> {
    let queue = &mut session.message_queue;

    if queue.len() == 0 {
        return Ok(())
    }

    let first = &queue[0];
    let packet = parsing::to_bytes(first)?;

    let result = session.stream.write_all(&packet);

    if is_would_block_io_result(&result) {
        return Ok(())
    }

    result?;
    queue.remove(0);

    Ok(())
}

enum IncommingProcessing {
    Proceed,
    Stop,
}

fn handle_notification(message: &str) -> Result<()> {
    println!("(Server) {}", message);
    Ok(())
}

fn handle_max_n_primes(primes: &[u32], session: &mut Session) -> Result<()> {
    if primes.len() < session.requested_primes_count as usize {
        println!("(Server) Sorry, I only have {} numbers: {:?}", primes.len(), &primes);
    } else {
        println!("(Server) Here you are: {:?}", &primes);
    }

    Ok(())
}

fn is_even(number: u32) -> bool {
    number & 1 == 0
}

fn perform_calculations(
    lower_bound: u32,
    count: u32,
    primes: Shared<Option<Vec<u32>>>,
) -> Result<()> {
    std::thread::sleep(Duration::from_secs(3));

    let mut new_primes = vec![];

    let mut current = if is_even(lower_bound) {
        lower_bound + 1
    } else {
        lower_bound
    };

    while current < lower_bound + count {
        let root = (current as f64).sqrt().ceil() as u32;
        let mut is_prime = true;

        for that in 3..root {
            if current % that == 0 {
                is_prime = false;
                break
            }
        }

        if is_prime {
            new_primes.push(current.clone());
        }

        current += 2;
    }

    let _value = primes.write()?.insert(new_primes);

    Ok(())
}

const CALCULATION_RANGE_SIZE: u32 = 10;

fn handle_range(lower_bound: u32, session: &mut Session) -> Result<()> {
    let count = CALCULATION_RANGE_SIZE;
    let primes = session.calculations_result.clone();

    std::thread::spawn(move || {
        with_error_report(|| perform_calculations(lower_bound, count, primes));
    });

    Ok(())
}

fn process_incomming(session: &mut Session) -> Result<IncommingProcessing> {
    let result: Result<ServerMessage> = read_message(&mut session.stream);

    if is_would_block_result(&result) {
        return Ok(IncommingProcessing::Proceed)
    }

    let message = match result {
        Ok(message) => message,
        Err(_) => {
            println!("(Console) Disconnected from the server");
            return Ok(IncommingProcessing::Stop)
        }
    };

    match message {
        ServerMessage::Notification { message } => handle_notification(&message)?,
        ServerMessage::MaxNPrimes { primes } => handle_max_n_primes(&primes, session)?,
        ServerMessage::Range { lower_bound } => handle_range(lower_bound.clone(), session)?,
        ServerMessage::PublishingResult {} => {
            session.is_range_requested = false;
        },
    }

    Ok(IncommingProcessing::Proceed)
}

fn check_calculations(session: &mut Session) -> Result<()> {
    let result = session.calculations_result.write()?.take();

    if let Some(primes) = result {
        let it = ClientMessage::PublishResults { primes };
        return send_packet(&it, session)
    }

    if !session.is_range_requested {
        session.is_range_requested = true;
        let it = ClientMessage::GetRange { count: CALCULATION_RANGE_SIZE };
        return send_packet(&it, session)
    }

    Ok(())
}

fn handle_connection(receiver: Receiver<Command>) -> Result<()> {
    let mut session: Option<Session> = None;

    loop {
        let maybe_command = receiver.try_recv();

        let processing = if let Ok(command) = maybe_command {
            handle_command(&command, &mut session)?
        } else {
            CommandProcessing::Ok
        };

        if matches!(processing, CommandProcessing::Stop) {
            break;
        }

        if let CommandProcessing::Connect(it) = processing {
            session = Some(it);
        }

        let mut it = if let Some(thing) = session {
            thing
        } else {
            continue
        };

        process_message_queue(&mut it)?;
        check_calculations(&mut it)?;

        let processing = process_incomming(&mut it)?;

        if matches!(processing, IncommingProcessing::Stop) {
            session = None;
        } else {
            session = Some(it);
        }

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn handle_commands() -> Result<()> {
    let (
        send_command,
        read_command,
    ) = channel::<Command>();

    std::thread::spawn(|| {
        with_error_report(|| handle_connection(read_command))
    });

    let stdin = std::io::stdin();
    let lock: &mut dyn BufRead = &mut stdin.lock();
    let mut reader = lock.to_chars().peekable();

    loop {
        let command = commands::parse(&mut reader);
        send_command.send(command.clone())?;

        if matches!(command, Command::End) {
            break
        }
    }

    Ok(())
}

pub fn start() {
    with_error_report(handle_commands);
}
