use consts::*;
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
    pub fn move_it(&mut self, speed: i32) {
        self.position = self.position + Coord_f::new(speed as f32 / FPS as f32, 0_f32);
    }
    pub fn get_border(&self) -> AABB {
        AABB::new(
            self.position,
            self.position + Coord_f::from_coord(self.size),
        )
    }
}
