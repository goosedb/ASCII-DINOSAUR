use aabb::AABB;
use coord::Coord;
use sprite::Sprite;
use consts::{ACCEL_OF_FREE_FALLING, DINOSAUR_CROUCH_SIZE_X, DINOSAUR_CROUCH_SIZE_Y,
             DINOSAUR_DEF_ACCEL_X, DINOSAUR_DEF_ACCEL_Y, DINOSAUR_DEF_SPEED_X,
             DINOSAUR_DEF_SPEED_Y, DINOSAUR_STRAIGHT_SIZE_X, DINOSAUR_STRAIGHT_SIZE_Y,
             FRAMES_PER_SECOND, GROUND, HEIGHT_OF_DOUBLE_JUMP, HEIGHT_OF_SINGLE_JUMP,
             SPEED_OF_JUMP};

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
            body: Body::CROUCH,
            legs: Legs::LEFT,
            speed: Coord::new(DINOSAUR_DEF_SPEED_X, DINOSAUR_DEF_SPEED_X),
            accel: Coord::new(DINOSAUR_DEF_ACCEL_X, DINOSAUR_DEF_ACCEL_Y),
        }
    }
    pub fn get_speed(&self) -> Coord {
        self.speed
    }
    pub fn get_position(&self) -> Coord {
        self.aabb.min
    }
    pub fn get_size(&self) -> Coord {
        match self.body {
            Body::STRAIGHT => Coord::new(
                DINOSAUR_STRAIGHT_SIZE_X as f64,
                DINOSAUR_STRAIGHT_SIZE_Y as f64,
            ),
            Body::CROUCH => {
                Coord::new(DINOSAUR_CROUCH_SIZE_X as f64, DINOSAUR_CROUCH_SIZE_Y as f64)
            }
        }
    }
    pub fn step(&mut self) {
        self.legs = Legs::from_int((self.legs.to_int() + 1) % 2);
    }
    pub fn move_it(&mut self) {
        let move_x = self.speed.x / FRAMES_PER_SECOND as f64;
        let mut move_y = self.speed.y / FRAMES_PER_SECOND as f64;

        self.speed.y += self.accel.y / FRAMES_PER_SECOND as f64;

        if self.aabb.max.y as i32 == HEIGHT_OF_SINGLE_JUMP {
            // if up-button is not pressed
            if self.speed.y == SPEED_OF_JUMP {
                self.accel.y == ACCEL_OF_FREE_FALLING;
            } else {
                self.speed.y /= 2.0;
            }
        }
        if self.aabb.max.y as i32 == HEIGHT_OF_DOUBLE_JUMP {
            self.accel.y == ACCEL_OF_FREE_FALLING;
        }

        if self.aabb.max.y + move_y > GROUND as f64 {
            move_y = 0.0;
            self.speed.y = 0.0;
            self.accel.y = 0.0;
        }

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
    pub fn up(&mut self) {
        if self.body == Body::CROUCH {
            self.straight();
            self.speed.y += SPEED_OF_JUMP;
        }
        // double jump
        else if self.aabb.max.y as i32 == HEIGHT_OF_SINGLE_JUMP {
            self.speed.y *= 2.0;
        }
    }
    pub fn down(&mut self) {
        if self.aabb.max.y == GROUND as f64 && self.body != Body::CROUCH{
            self.crouch();
        } else {
            self.accel.y *= 2.0;
        }
    }
    pub fn get_sprite(&self) -> Sprite {
        let mut sprite: Vec<char>;
        let mut size: Coord;
        match self.body {
            Body::STRAIGHT => {
                sprite = vec![
                    ' ', ' ', ' ', ' ', ' ', ' ', '@', 'O', '@', '@', '@',
                    ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', '@', '@', '@',
                    '@', ' ', ' ', ' ', ' ', '@', '@', '@', '@', ' ', ' ',
                    '@', '@', ' ', ' ', '@', '@', '@', '@', '@', '~', '~',
                    ' ', '@', '@', '@', '@', '@', '@', '@', ' ', ' ', ' ',
                    ' ', ' ', '@', '@', '@', '@', '@', ' ', ' ', ' ',' ',
                    ' ', ' ', ' ', 'I', ' ', 'L', ' ', ' ', ' ', ' ',' ',
                    ' ', ' ', ' ', 'L', ' ', ' ', ' ', ' ', ' ', ' ',' ',
                ];
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
                    '@', ' ', ' ', ' ', ' ', '@', '@', '@', '@', ' ', '@', 'O', '@', '@', '@', 
                    '@', '@', ' ', ' ', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@',
                    ' ', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', ' ', ' ', ' ',
                    ' ', ' ', ' ', '@', '@', '@', '@', '@', ' ', 'S', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', 'I', ' ', 'L', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', 'L', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ];
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
