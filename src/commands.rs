use super::model::*;

use phf::phf_map;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Place {
        x: i32,
        y: i32,
        direction: Direction,
    },
    Move,
    Left,
    Right,
    Report,
}

pub static COMMANDS: phf::Map<&'static str, Command> = phf_map! {
    "PLACE" => Command::Place{x: 0, y: 0, direction: Direction {x: 0, y: 0}},
    "MOVE" => Command::Move,
    "LEFT" => Command::Left,
    "RIGHT" => Command::Right,
    "REPORT" => Command::Report,
};

fn turn_left(robot: &mut Robot) {
    let old = &robot.direction;
    robot.direction = Direction {
        x: -old.y,
        y: old.x,
    };
}

fn turn_right(robot: &mut Robot) {
    let old = &robot.direction;
    robot.direction = Direction {
        x: old.y,
        y: -old.x,
    };
}

fn move_robot(robot: &mut Robot, table: &Table) {
    let x = robot.x + robot.direction.x;
    let y = robot.y + robot.direction.y;
    if table.valid_position(x, y) {
        robot.x = x;
        robot.y = y;
    }
}

fn report(robot: &Robot) {
    let direction = match robot.direction {
        Direction { x: 0, y: 1, .. } => "NORTH",
        Direction { x: 1, y: 0, .. } => "EAST",
        Direction { x: 0, y: -1, .. } => "SOUTH",
        Direction { x: -1, y: 0, .. } => "WEST",
        _ => "UNKNOWN",
    };
    println!("{},{} {direction}", robot.x, robot.y);
}

pub fn execute_command(table: &Table, robot: &mut Option<Robot>, command: Command) {
    match command {
        Command::Place { x, y, direction } => {
            if table.valid_position(x, y) {
                *robot = Some(Robot::new(x, y, direction));
            }
        }
        Command::Move => {
            if let Some(ref mut robot) = robot {
                move_robot(robot, table);
            }
        }
        Command::Left => {
            if let Some(ref mut robot) = robot {
                turn_left(robot);
            }
        }
        Command::Right => {
            if let Some(ref mut robot) = robot {
                turn_right(robot);
            }
        }
        Command::Report => {
            if let Some(ref robot) = robot {
                report(robot);
            }
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_left() {
        let mut robot = Robot::new(0, 0, DIRECTIONS.get("NORTH").unwrap().clone());
        turn_left(&mut robot);
        assert_eq!(robot.direction, DIRECTIONS.get("WEST").unwrap().clone());
    }

    #[test]
    fn test_right() {
        let mut robot = Robot::new(0, 0, DIRECTIONS.get("NORTH").unwrap().clone());
        turn_right(&mut robot);
        assert_eq!(robot.direction, DIRECTIONS.get("EAST").unwrap().clone());
    }

    #[test]
    fn test_move() {
        let mut robot = Robot::new(0, 0, DIRECTIONS.get("NORTH").unwrap().clone());
        move_robot(&mut robot, &Table::new(5, 5));
        assert_eq!(robot.y, 1);
    }

    #[test]
    fn test_move_off_table() {
        let mut robot = Robot::new(0, 4, DIRECTIONS.get("NORTH").unwrap().clone());
        move_robot(&mut robot, &Table::new(5, 5));
        assert_eq!(robot.y, 4);
    }

    #[test]
    fn test_report() {
        let robot = Robot::new(0, 0, DIRECTIONS.get("NORTH").unwrap().clone());
        report(&robot);
        assert_eq!(robot.y, 0);
    }

    #[test]
    fn test_place() {
        let mut robot = None;
        execute_command(
            &Table::new(5, 5),
            &mut robot,
            Command::Place {
                x: 1,
                y: 2,
                direction: DIRECTIONS.get("EAST").unwrap().clone(),
            },
        );
        let robot = robot.unwrap();
        assert_eq!(robot.x, 1);
        assert_eq!(robot.y, 2);
        assert_eq!(robot.direction, *DIRECTIONS.get("EAST").unwrap());
    }

    #[test]
    fn test_place_off_table() {
        let mut robot = None;
        execute_command(
            &Table::new(5, 5),
            &mut robot,
            Command::Place {
                x: 6,
                y: 2,
                direction: DIRECTIONS.get("EAST").unwrap().clone(),
            },
        );
        assert_eq!(robot, None);
    }
}
