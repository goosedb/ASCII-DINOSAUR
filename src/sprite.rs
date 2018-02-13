use coord::Coord;

pub struct Sprite {
    pub sprite: Vec<char>,
    pub size: Coord,
}

impl Sprite {
    pub fn new(sprite: Vec<char>, size: Coord) -> Sprite {
        Sprite {
            sprite: sprite,
            size: size,
        }
    }
}
