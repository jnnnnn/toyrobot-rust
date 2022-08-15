// Here is our model

use phf::phf_map;

pub struct Table {
    pub xsize: usize,
    pub ysize: usize,
}

pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

#[derive(Debug, Clone)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

pub static DIRECTIONS: phf::Map<&'static str, Direction> = phf_map! {
    "NORTH" => Direction { x: 0, y: 1 },
    "EAST" => Direction { x: 1, y: 0 },
    "SOUTH" => Direction { x: 0, y: -1 },
    "WEST" => Direction { x: -1, y: 0 },
};


impl Table {
    pub fn new(xsize: usize, ysize: usize) -> Table {
        Table {
            xsize: xsize,
            ysize: ysize,
        }
    }

    pub fn valid_position(&self, x: usize, y: usize) -> bool {
        x < self.xsize && y < self.ysize
    }
}

impl Robot {
    pub fn new(x: usize, y: usize, direction: Direction) -> Robot {
        Robot {
            x: x,
            y: y,
            direction: direction,
        }
    }
}
