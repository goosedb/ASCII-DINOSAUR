use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord_f {
    pub x: f32,
    pub y: f32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x: x, y: y }
    }
    pub fn from_coord_f(coord: Coord_f) -> Coord {
        Coord {
            x: coord.x as i32,
            y: coord.y as i32,
        }
    }
}

impl Coord_f {
    pub fn new(x: f32, y: f32) -> Coord_f {
        Coord_f { x: x, y: y }
    }
    pub fn from_coord(coord: Coord) -> Coord_f {
        Coord_f {
            x: coord.x as f32,
            y: coord.y as f32,
        }
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

impl Add for Coord_f {
    type Output = Coord_f;

    fn add(self, other: Coord_f) -> Coord_f {
        Coord_f {
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

impl Sub for Coord_f {
    type Output = Coord_f;

    fn sub(self, other: Coord_f) -> Coord_f {
        Coord_f {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
