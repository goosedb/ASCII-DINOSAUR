use aabb::AABB;
use coord::Coord;
use consts::{HEIGHT, WIDTH};

pub struct Camera {
    border: AABB,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            border: AABB::new(
                Coord::new(0.0, 0.0),
                Coord::new(WIDTH as f64, HEIGHT as f64),
            ),
        }
    }
    pub fn move_it(&mut self, step: Coord) {
        self.border.min = self.border.min + step;
        self.border.max = self.border.max + step;
    }
    pub fn get_min(&self) -> Coord {
        self.border.min
    }
    pub fn get_max(&self) -> Coord {
        self.border.max
    }
}
