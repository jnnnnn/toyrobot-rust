// Here is our model

pub struct Table {
    xsize: usize,
    ysize: usize,
}

pub struct Robot {
    x: usize,
    y: usize,
    direction: Direction,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Table {
    pub fn new(xsize: usize, ysize: usize) -> Table {
        Table {
            xsize: xsize,
            ysize: ysize,
        }
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
