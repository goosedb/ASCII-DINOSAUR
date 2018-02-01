use coord::*;

pub struct Sprite {
    pub sprite: Vec<char>,
    size: Coord,
}

impl Sprite {
    pub fn new(sprite: Vec<char>, size: Coord) -> Sprite {
        Sprite {
            sprite: sprite,
            size: size,
        }
    }
    pub fn get_pixel(&self, position: i32) -> char {
        self.sprite[position as usize]
    }
    pub fn get_size(&self) -> Coord {
        self.size
    }
    pub fn set_pixel(&mut self, pix: char, pos: Coord) {
        self.sprite[(pos.y * self.size.x + pos.x) as usize] = pix;
    }
}
