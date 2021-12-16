use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::thread;
// use std::collections::{HashMap};
// use std::time::{Duration};
// use std::path::{Path};
// use std::fs::{File};

use tas_server::messages::{ClientMessage, ServerMessage};
use tas_server::parsing::{read_message};

use common::{Result, with_error_report};
use common::shared::{IntoShared, Shared};
use common::shared::vec::{SharedVec};

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

    Ok(())
}

pub fn main() {
    with_error_report(handle_connection);
}
