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
    pub fn move_it(&mut self, move_x: f64) {
        self.border.min.x += move_x;
        self.border.max.x += move_x;
    }
    pub fn get_min(&self) -> Coord {
        self.border.min
    }
    pub fn get_max(&self) -> Coord {
        self.border.max
    }
}
