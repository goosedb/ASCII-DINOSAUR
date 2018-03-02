use aabb::AABB;
use coord::Coord;
use sprite::Sprite;
use consts::GROUND;
use consts::FRAMES_PER_SECOND;
use consts::ACCEL_OF_FREE_FALLING;
use consts::{DINOSAUR_DEF_ACCEL_X, DINOSAUR_DEF_ACCEL_Y};
use consts::{DINOSAUR_DEF_SPEED_X, DINOSAUR_DEF_SPEED_Y};
use consts::{DINOSAUR_CROUCH_SIZE_X, DINOSAUR_CROUCH_SIZE_Y};
use consts::{DINOSAUR_STRAIGHT_SIZE_X, DINOSAUR_STRAIGHT_SIZE_Y};
use consts::{HEIGHT_OF_DOUBLE_JUMP, HEIGHT_OF_SINGLE_JUMP, SPEED_OF_JUMP};

#[derive(PartialEq)]
pub enum Body {
    STRAIGHT,
    CROUCH,
}

#[derive(Clone, Copy)]
enum Legs {
    LEFT,
    RIGHT,
}

enum Eye {
    SMALL,
    BIG,
}

impl Legs {
    fn from_int(t: i32) -> Legs {
        let types = [Legs::LEFT, Legs::RIGHT];
        types[t as usize]
    }
    fn to_int(&self) -> i32 {
        match *self {
            Legs::LEFT => 0,
            Legs::RIGHT => 1,
        }
    }
}

pub struct Dinosaur {
    aabb: AABB,
    eye: Eye,
    pub body: Body,
    legs: Legs,
    speed: Coord,
    accel: Coord,
}

impl Dinosaur {
    pub fn new() -> Dinosaur {
        Dinosaur {
            aabb: AABB::new(
                Coord::new(3.0, (GROUND - DINOSAUR_STRAIGHT_SIZE_Y) as f64),
                Coord::new(3.0 + DINOSAUR_STRAIGHT_SIZE_X as f64, GROUND as f64),
            ),
            eye: Eye::SMALL,
            body: Body::STRAIGHT,
            legs: Legs::LEFT,
            speed: Coord::new(DINOSAUR_DEF_SPEED_X, DINOSAUR_DEF_SPEED_Y),
            accel: Coord::new(DINOSAUR_DEF_ACCEL_X, DINOSAUR_DEF_ACCEL_Y),
        }
    }
    pub fn get_speed(&self) -> Coord {
        self.speed
    }
    pub fn get_position(&self) -> Coord {
        self.aabb.min
    }
    #[warn(dead_code)]
    pub fn get_aabb(&self) -> AABB {
        self.aabb
    }
    pub fn step(&mut self) {
        self.legs = Legs::from_int((self.legs.to_int() + 1) % 2);
    }
    pub fn move_it(&mut self, move_x: f64, move_y: f64) {
        self.aabb.min = self.aabb.min + Coord::new(move_x, move_y);
        self.aabb.max = self.aabb.max + Coord::new(move_x, move_y);
    }
    pub fn straight(&mut self) {
        self.body = Body::STRAIGHT;
        self.aabb.min.y -= DINOSAUR_STRAIGHT_SIZE_Y as f64 - DINOSAUR_CROUCH_SIZE_Y as f64;
        self.aabb.max.x -= DINOSAUR_CROUCH_SIZE_X as f64 - DINOSAUR_STRAIGHT_SIZE_X as f64;
    }
    pub fn crouch(&mut self) {
        self.body = Body::CROUCH;
        self.aabb.min.y += DINOSAUR_STRAIGHT_SIZE_Y as f64 - DINOSAUR_CROUCH_SIZE_Y as f64;
        self.aabb.max.x += DINOSAUR_CROUCH_SIZE_X as f64 - DINOSAUR_STRAIGHT_SIZE_X as f64;
    }
    pub fn up(&mut self) {}
    pub fn down(&mut self) {}
    pub fn get_sprite(&self) -> Sprite {
        let mut sprite: Vec<char>;
        let mut size: Coord;
        match self.body {
            Body::STRAIGHT => {
                sprite = vec![
                    ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@',
                    ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@',
                    '@', ' ', ' ', ' ', ' ', '@', '@', '@', '@', ' ', ' ',
                    '@', '@', ' ', ' ', '@', '@', '@', '@', '@', '~', '~',
                    ' ', '@', '@', '@', '@', '@', '@', '@', ' ', ' ', ' ',
                    ' ', ' ', '@', '@', '@', '@', '@', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ];
                match self.eye {
                    Eye::BIG => sprite[7] = 'O',
                    Eye::SMALL => sprite[7] = 'o',
                }
                match self.legs {
                    Legs::LEFT => {
                        sprite[80] = 'L';
                        sprite[71] = 'L';
                        sprite[69] = 'I';
                    }
                    Legs::RIGHT => {
                        sprite[82] = 'L';
                        sprite[71] = 'I';
                        sprite[69] = 'L';
                    }
                }
                size = Coord::new(
                    DINOSAUR_STRAIGHT_SIZE_X as f64,
                    DINOSAUR_STRAIGHT_SIZE_Y as f64,
                );
                /*
                        @o@@@@
                        @@@@@@
                @    @@@@@@  
                @@  @@@@@@@~~
                 @@@@@@@@@   
                  @@@@@@     
                   @  @@     
                   @@         
                */
            }
            Body::CROUCH => {
                sprite = vec![
                    '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    '@', '@', ' ', ' ', '@', '@', '@', '@', ' ', '@', '@', '@', '@', '@',
                    ' ', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@',
                    ' ', ' ', '@', '@', '@', '@', '@', '@', '@', '@', '@', ' ', ' ', ' ',
                    ' ', ' ', ' ', '@', '@', '@', '@', ' ', 'S', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', '@', ' ', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',' ', ' ',
                ];
                match self.eye {
                    Eye::BIG => sprite[24] = 'O',
                    Eye::SMALL => sprite[24] = 'o',
                }
                size = Coord::new(DINOSAUR_CROUCH_SIZE_X as f64, DINOSAUR_CROUCH_SIZE_Y as f64);
                /*
                    @    @@@@@@ @o@@@@
                    @@  @@@@@@@ @@@@@@
                     @@@@@@@@@@@@@
                       @@@@@@  S
                       @  @@
                       @@
                */
            }
        }
        Sprite::new(sprite, size)
    }
}
