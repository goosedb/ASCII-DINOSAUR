use coord::*;
use aabb::*;

pub struct Camera {
    size: Coord,
    position: Coord_f,
}

impl Camera {
    pub fn new(size: Coord, position: Coord_f) -> Camera {
        Camera {
            size: size,
            position: position,
        }
    }
    pub fn move_it(&mut self, step: Coord_f) {
        self.position = self.position + step;
    }
    pub fn get_border(&self) -> AABB {
        AABB::new(
            self.position,
            self.position + Coord_f::from_coord(self.size),
        )
    }
}
