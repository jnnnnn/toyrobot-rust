use super::model::*;

use phf::phf_map;

#[derive(Clone)]
pub enum Command {
    Place {
        x: usize,
        y: usize,
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
    robot.x = (robot.x as i32 + robot.direction.x) as usize;
    robot.y = (robot.y as i32 + robot.direction.y) as usize;
    if robot.x >= table.xsize {
        robot.x = table.xsize - 1;
    }
    if robot.y >= table.ysize {
        robot.y = table.ysize - 1;
    }
}

fn report(robot: &Robot) {
    println!("{},{},{:?}", robot.x, robot.y, robot.direction);
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