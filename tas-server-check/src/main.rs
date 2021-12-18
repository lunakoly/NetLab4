use std::net::{TcpStream};

use tas_server::messages::{ClientMessage, ServerMessage};
use tas_server::{DEFAULT_PORT};

use common::{Result, with_error_report};

fn read_incomming(stream: &mut TcpStream) -> Result<()> {
    let it: ServerMessage = common::parsing::read_message(stream)?;
    println!("{:?}\n", &it);
    Ok(())
}

fn probe(message: &ClientMessage, stream: &mut TcpStream) -> Result<()> {
    common::parsing::to_writer(&message, stream)?;
    read_incomming(stream)
}

fn handle_connection() -> Result<()> {
    let mut stream = TcpStream::connect(format!("localhost:{}", DEFAULT_PORT))?;

    macro_rules! probe {
        ( $it:expr ) => {
            probe(&$it, &mut stream)?;
        };
    }

    read_incomming(&mut stream)?;
    read_incomming(&mut stream)?;

    probe! {
        ClientMessage::Execute {
            command: vec!["login".into(), "john".into(), "qwer".into()],
        }
    }

    probe! {
        ClientMessage::Execute {
            command: vec!["ls".into()],
        }
    }

    probe! {
        ClientMessage::Execute {
            command: vec!["cd".into(), "src".into()],
        }
    }

    probe! {
        ClientMessage::Execute {
            command: vec!["cd".into(), "..".into()],
        }
    }

    probe! {
        ClientMessage::Execute {
            command: vec!["cd".into(), "..".into()],
        }
    }

    probe! {
        ClientMessage::Execute {
            command: vec!["cd".into(), "E:\\\\".into()],
        }
    }

    probe! {
        ClientMessage::Execute {
            command: vec!["ls".into()],
        }
    }

    probe! {
        ClientMessage::Execute {
            command: vec!["who".into()],
        }
    }

    probe! {
        ClientMessage::Execute {
            command: vec!["login".into(), "ron".into(), "4321".into()],
        }
    }

    probe! {
        ClientMessage::Execute {
            command: vec!["kill".into(), "ron".into()],
        }
    }

    loop {
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

pub fn main() {
    with_error_report(handle_connection);
}
