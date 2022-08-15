// Here is our model

use phf::phf_map;

#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    pub xsize: i32,
    pub ysize: i32,
}

#[derive(Debug, PartialEq, Eq)]

pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
        x >= 0 && y >= 0 && x < self.xsize && y < self.ysize
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bounds() {
        assert!(Table::new(5, 5).valid_position(0, 0));
        assert!(Table::new(5, 5).valid_position(4, 4));
        assert!(!Table::new(5, 5).valid_position(5, 5));
        assert!(!Table::new(5, 5).valid_position(4, 5));
        assert!(!Table::new(5, 5).valid_position(5, 4));
        assert!(!Table::new(5, 5).valid_position(0, -1));
        assert!(!Table::new(5, 5).valid_position(-1, 0));
    }
}