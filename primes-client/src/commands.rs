use std::iter::{Peekable};

use crate::chars_reader::{CharsReader};
use crate::{DEFAULT_PORT};

#[derive(Clone)]
pub enum Command {
    Nothing,
    End,
    Connect { address: String },
    GetMaxNPrimes { count: u32 },
}

fn is_blank(symbol: char) -> bool {
    symbol == '\r' ||
    symbol == '\n' ||
    symbol == '\t' ||
    symbol == ' '
}

fn parse_connect(words: &[String]) -> Command {
    if words.len() >= 3 {
        Command::Connect {
            address: format!("{}:{}", words[1], words[2])
        }
    } else if words.len() >= 2 {
        Command::Connect {
            address: format!("{}:{}", words[1], DEFAULT_PORT),
        }
    } else {
        Command::Connect {
            address: format!("localhost:{}", DEFAULT_PORT),
        }
    }
}

fn parse_max(words: &[String]) -> Command {
    if words.len() >= 2 {
        match words[1].parse::<u32>() {
            Ok(it) => {
                Command::GetMaxNPrimes {
                    count: it
                }
            }
            Err(_) => {
                println!("(Console) Noooooo, you can't just put a non-u32 value here!!1!");
                Command::Nothing
            }
        }
    } else {
        println!("(Console) Max what? 1, 2, 100 - how many?");
        Command::Nothing
    }
}

fn parse_words<'a>(input: &mut Peekable<CharsReader<'a>>) -> Vec<String> {
    let mut words = vec!["".to_owned()];

    while let Some(it) = input.next() {
        if it == '\r' {
            // ignore
        } else if it == '\n' {
            break
        } else if is_blank(it) {
            if words[words.len() - 1].len() != 0 {
                words.push("".to_owned());
            }
        } else {
            let last_index = words.len() - 1;
            words[last_index].push(it);
        }
    }

    words
}

fn parse_command<'a>(input: &mut Peekable<CharsReader<'a>>) -> Command {
    let words: Vec<String> = parse_words(input)
        .into_iter()
        .filter(|it| it.len() > 0)
        .collect();

    if words.len() == 0 {
        Command::Nothing
    } else if words[0] == "q" || words[0] == "quit" || words[0] == "exit" {
        Command::End
    } else if words[0] == "connect" || words[0] == "c" {
        parse_connect(&words)
    } else if words[0] == "max" {
        parse_max(&words)
    } else {
        println!("(Console) No succh a command found...");
        Command::Nothing
    }
}

pub fn parse<'a>(input: &mut Peekable<CharsReader<'a>>) -> Command {
    if let Some(_) = input.peek() {
        parse_command(input)
    } else {
        Command::End
    }
}
