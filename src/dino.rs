use coord::*;
use aabb::*;
use consts::*;
use sprite::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Eye {
    SMALL,
    BIG,
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Legs {
    LEFT,
    RIGHT,
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Pose {
    STRAIGHT,
    CROUCH,
}

impl Eye {
    fn to_int(&self) -> i32 {
        match *self {
            Eye::SMALL => 0,
            Eye::BIG => 1,
        }
    }
    fn from_int(n: i32) -> Eye {
        let eyes = [Eye::SMALL, Eye::BIG];
        eyes[n as usize]
    }
    fn next(&self) -> Eye {
        Eye::from_int((self.to_int() + 1) % 2)
    }
}

impl Legs {
    fn to_int(&self) -> i32 {
        match *self {
            Legs::LEFT => 0,
            Legs::RIGHT => 1,
        }
    }
    fn from_int(n: i32) -> Legs {
        let legs = [Legs::LEFT, Legs::RIGHT];
        legs[n as usize]
    }
    fn next(&self) -> Legs {
        Legs::from_int((self.to_int() + 1) % 2)
    }
}

impl Pose {
    fn to_int(&self) -> i32 {
        match *self {
            Pose::STRAIGHT => 0,
            Pose::CROUCH => 1,
        }
    }
    fn from_int(n: i32) -> Pose {
        let poses = [Pose::STRAIGHT, Pose::CROUCH];
        poses[n as usize]
    }
    fn next(&self) -> Legs {
        Legs::from_int((self.to_int() + 1) % 2)
    }
}

pub struct Dino {
    pub collision_model: AABB,
    eye: Eye,
    legs: Legs,
    pose: Pose,
    speed: Coord,
    accel: Coord,
}

impl Dino {
    pub fn new() -> Dino {
        Dino {
            collision_model: AABB::new(Coord::new(3, HEIGHT - 11), Coord::new(3 + 14, HEIGHT - 4)),
            eye: Eye::SMALL,
            legs: Legs::LEFT,
            pose: Pose::STRAIGHT,
            speed: Coord::new(1, 0),
            accel: Coord::new(0, -1),
        }
    }
    pub fn step(&mut self) {
        self.legs = self.legs.next();
    }
    pub fn jump(&mut self) {
        self.speed = self.speed + Coord::new(0, 3);
    }
    pub fn get_sprite(&self) -> Sprite {
        let mut sprite = Sprite::new(vec![' '], Coord::new(0, 0));

        match self.pose {
            Pose::STRAIGHT => {
                sprite = Sprite::new(
                    vec![
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@', '@', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@', '@', '@', ' ', ' ', ' ',
                        ' ', '@', '@', '@', '@', '@', '@', ' ', ' ', '@', '@', ' ', ' ', '@', '@',
                        '@', '@', '@', '@', '@', '~', '~', ' ', '@', '@', '@', '@', '@', '@', '@',
                        '@', '@', ' ', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@', '@', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ],
                    Coord::new(13, 8), //self.collision_model.max - self.collision_model.mi
                );
                match self.legs {
                    Legs::LEFT => {
                        sprite.set_pixel('@', Coord::new(3, 6));
                        sprite.set_pixel('@', Coord::new(3, 7));
                        sprite.set_pixel('@', Coord::new(4, 7));
                        sprite.set_pixel('@', Coord::new(6, 6));
                        sprite.set_pixel('@', Coord::new(7, 6));
                    }
                    Legs::RIGHT => {
                        sprite.set_pixel('@', Coord::new(6, 6));
                        sprite.set_pixel('@', Coord::new(6, 7));
                        sprite.set_pixel('@', Coord::new(7, 7));
                        sprite.set_pixel('@', Coord::new(3, 6));
                        sprite.set_pixel('@', Coord::new(4, 6));
                    }
                }
                match self.eye {
                    Eye::SMALL => sprite.set_pixel('o', Coord::new(8, 0)),
                    Eye::BIG => sprite.set_pixel('O', Coord::new(8, 0)),
                }
            }
            Pose::CROUCH => {
                sprite = Sprite::new(
                    vec![
                        '@', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@', '@', ' ', '@', '@', '@',
                        '@', '@', '@', '@', '@', ' ', ' ', '@', '@', '@', '@', '@', '@', '@', ' ',
                        '@', '@', '@', '@', '@', '@', ' ', '@', '@', '@', '@', '@', '@', '@', '@',
                        '@', '@', '@', '@', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', '@',
                        '@', '@', '@', ' ', ' ', 'S', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ',
                    ],
                    self.collision_model.max - self.collision_model.min,
                );
                match self.legs {
                    Legs::LEFT => {
                        sprite.set_pixel('I', Coord::new(4, 4));
                        sprite.set_pixel('L', Coord::new(4, 5));
                        sprite.set_pixel('L', Coord::new(6, 4));
                    }
                    Legs::RIGHT => {
                        sprite.set_pixel('I', Coord::new(6, 4));
                        sprite.set_pixel('L', Coord::new(6, 5));
                        sprite.set_pixel('L', Coord::new(4, 4));
                    }
                }

                match self.eye {
                    Eye::SMALL => sprite.set_pixel('o', Coord::new(13, 0)),
                    Eye::BIG => sprite.set_pixel('O', Coord::new(13, 0)),
                }
            }
        }
        sprite
    }
    pub fn get_position(&self) -> Coord {
        self.collision_model.min
    }
}
