mod commands;
mod input;
mod model;

// prepare the list of commands (PLACE, MOVE, and TURN) and then call them as lines of text input dictate.
fn main() {
    let table = model::Table::new(5, 5);
    let mut robot: Option<model::Robot> = None;

    loop {
        match input::read_input() {
            Ok(command) => {
                commands::execute_command(&table, &mut robot, command);
                
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}
