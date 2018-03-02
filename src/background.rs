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
    pub fn move_it(&mut self, move_x: f64) {
        self.clouds_position += move_x / 50.0 as f64;
        self.ground_position += move_x;
    }
}
