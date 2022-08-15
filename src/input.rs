use super::commands::*;
use super::model;
use std::io;

// Read a line of input and parse it, returning a runnable command closure to run
pub fn read_input() -> Result<Command, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut words = input.split_whitespace();
    let keyword = words.next().ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "No keyword found",
    ))?;
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
    let x = get_int(words.next())?;
    let y = get_int(words.next())?;
    let direction_input = match words.next() {
        Some(direction) => direction,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Missing direction",
            ))
        }
    };
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

fn get_int(word: Option<&str>) -> Result<i32, std::io::Error> {
    word.ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "Missing integer",
    ))?.parse::<i32>().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid integer",
        )
    })
}
