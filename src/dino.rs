use coord::*;
use aabb::*;
use consts::*;
use sprite::*;

pub struct Dino {
    pub collision_model: AABB,
    eye: i32,
    pub legs: i32,
    pose: i32,
    speed: Coord,
    accel: Coord,
}

impl Dino {
    pub fn new() -> Dino {
        Dino {
            collision_model: AABB::new(Coord::new(3, HEIGHT - 11), Coord::new(3 + 14, HEIGHT - 4)),
            eye: 0,
            legs: 0,
            pose: 0,
            speed: Coord::new(1, 0),
            accel: Coord::new(0, -1),
        }
    }
    pub fn step(&mut self) {
        self.legs = (self.legs + 1) % 2;
    }
    pub fn jump(&mut self) {
        self.speed = self.speed + Coord::new(0, 3);
    }
    pub fn get_sprite(&self) -> Sprite {
        let mut sprite = Sprite::new(vec![' '], Coord::new(0,0));

        if self.pose == 0 {
            sprite = Sprite::new(
                vec![
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@', '@', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@', '@', '@', ' ', ' ', ' ', ' ', '@',
                    '@', '@', '@', '@', '@', ' ', ' ', '@', '@', ' ', ' ', '@', '@', '@', '@', '@',
                    '@', '@', '~', '~', ' ', '@', '@', '@', '@', '@', '@', '@', '@', '@', ' ', ' ',
                    ' ', ' ', ' ', '@', '@', '@', '@', '@', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ],
                Coord::new(13,8)
                //self.collision_model.max - self.collision_model.mi
            );
            if self.legs == 0 {
                sprite.set_pixel('|', Coord::new(3, 6));
                sprite.set_pixel('L', Coord::new(3, 7));
                sprite.set_pixel('L', Coord::new(6, 6));
            }
            if self.legs == 1 {
                sprite.set_pixel('|', Coord::new(6, 6));
                sprite.set_pixel('L', Coord::new(6, 7));
                sprite.set_pixel('L', Coord::new(3, 6));
            }
            if self.eye == 0 {
                sprite.set_pixel('o', Coord::new(9, 0));
            }
            if self.eye == 1 {
                sprite.set_pixel('O', Coord::new(9, 0));
            }
        }
        if self.pose == 1 {
            sprite = Sprite::new(
                vec![
                '@', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@', '@', ' ', '@', '@', '@', '@',
                '@', '@', '@', '@', ' ', ' ', '@', '@', '@', '@', '@', '@', '@', ' ', '@', '@',
                '@', '@', '@', '@', ' ', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@',
                '@', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@', '@', ' ',
                ' ', 'S', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ],
                self.collision_model.max - self.collision_model.min);
            if self.legs == 0 {
                sprite.set_pixel('I', Coord::new(4, 4));
                sprite.set_pixel('L', Coord::new(4, 5));
                sprite.set_pixel('L', Coord::new(6, 4));
            }
            if self.legs == 1 {
                sprite.set_pixel('I', Coord::new(6, 4));
                sprite.set_pixel('L', Coord::new(6, 5));
                sprite.set_pixel('L', Coord::new(4, 4));
            }
            if self.eye == 0 {
                sprite.set_pixel('o', Coord::new(13, 0));
            }
            if self.eye == 1 {
                sprite.set_pixel('O', Coord::new(13, 0));
            }
        }
        sprite
    }
    pub fn get_position(&self) -> Coord {
        self.collision_model.min
    }
}
