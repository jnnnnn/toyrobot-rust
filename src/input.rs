use super::commands::*;
use super::model;
use std::io;

// Read a line of input and parse it, returning a runnable command closure to run
pub fn read_input() -> Result<Command, &'static str> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read input")?;
    parse_input(input)
}

fn parse_input(input: String) -> Result<Command, &'static str> {
    // split input by comma or space
    let mut words = input.split(|c| c == ',' || c == ' ');
    let keyword = words.next().ok_or("No keyword found")?;
    match parse_keyword(&keyword) {
        Some(command) => parse_command(command, words),
        None => Err("Unknown Command"),
    }
}

fn parse_keyword(keyword: &str) -> Option<Command> {
    COMMANDS.get(keyword).cloned()
}

fn parse_command<'a>(
    command: Command,
    words: impl Iterator<Item = &'a str>,
) -> Result<Command, &'static str> {
    match command {
        Command::Place { .. } => parse_place(words),
        _ => Ok(command),
    }
}

fn parse_place<'a>(mut words: impl Iterator<Item = &'a str>) -> Result<Command, &'static str> {
    let x = get_int(words.next())?;
    let y = get_int(words.next())?;
    let direction_input = match words.next() {
        Some(direction) => direction,
        None => return Err("Missing direction"),
    };
    let direction = match model::DIRECTIONS.get(direction_input) {
        Some(direction) => direction,
        None => return Err("Invalid direction"),
    };
    Ok(Command::Place {
        x,
        y,
        direction: direction.clone(),
    })
}

fn get_int(word: Option<&str>) -> Result<i32, &'static str> {
    word.ok_or("Missing integer")?
        .parse::<i32>()
        .map_err(|_| "Invalid integer")
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_place() {
        let input = "PLACE 1,2,NORTH";
        let command = parse_input(input.to_string()).unwrap();
        assert_eq!(
            command,
            Command::Place {
                x: 1,
                y: 2,
                direction: model::DIRECTIONS.get("NORTH").unwrap().clone(),
            }
        );
    }

    #[test]
    fn test_parse_invalid_place() {
        let input = "PLACE 1,2,BAD";
        let result = parse_input(input.to_string());
        assert_eq!(result, Err("Invalid direction"));
    }

    #[test]
    fn test_garbage() {
        let input = "garbage";
        let result = parse_input(input.to_string());
        assert_eq!(result, Err("Unknown Command"));
    }

    #[test]
    fn test_blank() {
        let input = "";
        let result = parse_input(input.to_string());
        assert_eq!(result, Err("Unknown Command"));
    }
}
