use coord::Coord;
use consts::FRAMES_PER_SECOND;

pub struct Background {
    clouds_position: f64,
    ground_position: f64,
}

impl Background {
    pub fn new() -> Background {
        Background {
            clouds_position: 0.0,
            ground_position: 0.0,
        }
    }
    pub fn move_it(&mut self, speed: Coord) {
        self.clouds_position += speed.x / 50.0 / FRAMES_PER_SECOND as f64;
        self.ground_position += speed.x / FRAMES_PER_SECOND as f64;
    }
}
