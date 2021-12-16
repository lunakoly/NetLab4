use std::net::{TcpStream};

use tas_server::messages::{ClientMessage, ServerMessage};

use common::{Result, with_error_report};

const DEFAULT_PORT: u32 = 6969;

fn probe(message: &ClientMessage, stream: &mut TcpStream) -> Result<()> {
    tas_server::parsing::to_writer(&message, stream)?;

    let it: ServerMessage = tas_server::parsing::read_message(stream)?;
    println!("{:?}\n", &it);

    Ok(())
}

fn handle_connection() -> Result<()> {
    let mut stream = TcpStream::connect(format!("localhost:{}", DEFAULT_PORT))?;

    let it: ServerMessage = tas_server::parsing::read_message(&mut stream)?;
    println!("{:?}\n", &it);

    let it: ServerMessage = tas_server::parsing::read_message(&mut stream)?;
    println!("{:?}\n", &it);

    let it = ClientMessage::Execute {
        command: vec!["login".into(), "john".into(), "qwer".into()],
    };

    probe(&it, &mut stream)?;

    let it = ClientMessage::Execute {
        command: vec!["ls".into()],
    };

    probe(&it, &mut stream)?;

    let it = ClientMessage::Execute {
        command: vec!["cd".into(), "src".into()],
    };

    probe(&it, &mut stream)?;

    let it = ClientMessage::Execute {
        command: vec!["cd".into(), "..".into()],
    };

    probe(&it, &mut stream)?;

    let it = ClientMessage::Execute {
        command: vec!["cd".into(), "..".into()],
    };

    probe(&it, &mut stream)?;

    let it = ClientMessage::Execute {
        command: vec!["cd".into(), "E:\\\\".into()],
    };

    probe(&it, &mut stream)?;

    let it = ClientMessage::Execute {
        command: vec!["ls".into()],
    };

    probe(&it, &mut stream)?;

    let it = ClientMessage::Execute {
        command: vec!["who".into()],
    };

    probe(&it, &mut stream)?;

    let it = ClientMessage::Execute {
        command: vec!["login".into(), "ron".into(), "4321".into()],
    };

    probe(&it, &mut stream)?;

    let it = ClientMessage::Execute {
        command: vec!["kill".into(), "ron".into()],
    };

    probe(&it, &mut stream)?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

pub fn main() {
    with_error_report(handle_connection);
}
