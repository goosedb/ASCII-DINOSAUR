use vec2::Vec2;

pub struct Sprite {
    size: Vec2,
    pixels: Vec<char>,
    position: Vec2,
}

impl Sprite {
    pub fn new(position: Vec2, size: Vec2) -> Sprite {
        Sprite {
            size: size,
            pixels: vec![' '; (size.x * size.y) as usize],
            position: position,
        }
    }
    pub fn size(&self) -> Vec2 {
        self.size
    }
    pub fn position(&self) -> Vec2 {
        self.position
    }
    pub fn set_pixel(&mut self, position: Vec2, pixel: char) {
        self.pixels[(position.x + position.y * self.size.x) as usize] = pixel;
    }
    pub fn get_pixel(&self, position: Vec2) -> char {
        self.pixels[(position.x + position.y * self.size.x) as usize]
    }
}
