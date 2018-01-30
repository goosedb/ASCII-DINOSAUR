use coord::*;
use aabb::*;

pub struct Camera {
    size: Coord,
    position: Coord,
}

impl Camera {
    pub fn new(size: Coord, position: Coord) -> Camera {
        Camera {
            size: size,
            position: position,
        }
    }
    pub fn move_it(&mut self, step: Coord) {
        self.position.x += step.x;
        self.position.y += step.y;
    }
    pub fn get_border(&self) -> AABB {
        AABB::new(self.position, self.position + self.size)
    }
}
