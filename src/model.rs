// Here is our model

use phf::phf_map;

pub struct Table {
    pub xsize: i32,
    pub ysize: i32,
}

pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

#[derive(Clone)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

pub static DIRECTIONS: phf::Map<&'static str, Direction> = phf_map! {
    "NORTH" => Direction { x: 0, y: 1, },
    "EAST" => Direction { x: 1, y: 0, },
    "SOUTH" => Direction { x: 0, y: -1, },
    "WEST" => Direction { x: -1, y: 0, },
};

impl Table {
    pub fn new(xsize: i32, ysize: i32) -> Table {
        Table {
            xsize: xsize,
            ysize: ysize,
        }
    }

    pub fn valid_position(&self, x: i32, y: i32) -> bool {
        x < self.xsize && y < self.ysize
    }
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Robot {
        Robot {
            x: x,
            y: y,
            direction: direction,
        }
    }
}
