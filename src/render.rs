use camera::Camera;
use consts::{HEIGHT, WIDTH};

pub struct Render {
    pub camera: Camera,
    render: Vec<char>,
}

impl Render {
    pub fn new() -> Render {
        Render {
            camera: Camera::new(),
            render: vec![' '; (WIDTH + HEIGHT) as usize],
        }
    }
}
