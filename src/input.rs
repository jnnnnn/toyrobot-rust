use super::commands::*;
use super::model;
use std::io;

// Read a line of input and parse it, returning a runnable command closure to run
pub fn read_input() -> Result<Command, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut words = input.split_whitespace();
    let keyword = words.next().unwrap();
    match parse_keyword(&keyword) {
        Some(command) => parse_command(command, words),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unknown Command",
        )),
    }
}

fn parse_keyword(keyword: &str) -> Option<Command> {
    COMMANDS.get(keyword).cloned()
}

fn parse_command(
    command: Command,
    words: std::str::SplitWhitespace,
) -> Result<Command, std::io::Error> {
    match command {
        Command::Place { .. } => parse_place(words),
        _ => Ok(command),
    }
}

fn parse_place(mut words: std::str::SplitWhitespace) -> Result<Command, std::io::Error> {
    let x = words.next().unwrap().parse::<usize>().unwrap();
    let y = words.next().unwrap().parse::<usize>().unwrap();
    let direction_input = words.next().unwrap();
    let direction = match model::DIRECTIONS.get(direction_input) {
        Some(direction) => direction,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid direction",
            ))
        }
    };
    Ok(Command::Place {
        x,
        y,
        direction: direction.clone(),
    })
}
