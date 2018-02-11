use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub fn new(x: f64, y: f64) -> Coord {
        Coord { x: x, y: y }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
