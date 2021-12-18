use std::net::{TcpStream, TcpListener};

use primes_client::messages::{ClientMessage, ServerMessage};
use primes_client::{DEFAULT_PORT};

use common::{Result, with_error_report};

fn collect_primes(count: u32, numbers: &mut Vec<bool>) -> Vec<u32> {
    let mut primes = vec![];

    for (index, &is_prime) in numbers.iter().enumerate().rev() {
        if is_prime {
            primes.push(index as u32);
        }

        if primes.len() >= count as usize {
            break
        }
    }

    primes.reverse();
    primes
}

fn handle_incomming(
    message: ClientMessage,
    stream: &mut TcpStream,
    numbers: &mut Vec<bool>,
) -> Result<()> {
    macro_rules! send {
        ( $it:expr ) => {
            common::parsing::to_writer(&$it, stream)?;
        };
    }

    println!("{:?}\n", &message);

    match message {
        ClientMessage::GetMaxNPrimes { count } => {
            send! {
                ServerMessage::MaxNPrimes {
                    primes: collect_primes(count, numbers),
                }
            }
        }
        ClientMessage::GetRange { count } => {
            let length = numbers.len();
            numbers.resize(length + count as usize, false);

            send! {
                ServerMessage::Range {
                    lower_bound: length as u32,
                }
            }
        }
        ClientMessage::PublishResults { primes } => {
            for it in primes {
                numbers[it as usize] = true;
            }

            send! {
                ServerMessage::PublishingResult {  }
            }
        }
    }

    Ok(())
}

fn handle_connection() -> Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", DEFAULT_PORT))?;
    let mut numbers = vec![];

    for incomming in listener.incoming() {
        let mut stream = incomming?;

        loop {
            let it: ClientMessage = common::parsing::read_message(&mut stream)?;
            handle_incomming(it, &mut stream, &mut numbers)?;
        }
    }

    Ok(())
}

pub fn main() {
    with_error_report(handle_connection);
}
