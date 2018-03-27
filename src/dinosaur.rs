use vec2::Vec2;
use aabb::AABB;
use consts::{FPS, ZERO};
use consts::{DINO_SIZE_X, DINO_SIZE_Y};
use consts::{DINO_START_POS_X, DINO_START_POS_Y};
use consts::{DINO_START_SPEED_X, DINO_START_SPEED_Y};
use consts::{DINO_ACCEL_OF_FALLING, DINO_ACCEL_SPEED_X, DINO_SPEED_OF_JUMP};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum State {
    IN_AIR,
    ON_GROUND,
}

#[derive(Clone, Copy)]
pub enum Legs {
    RIGHT,
    LEFT,
}

impl Legs {
    fn next(&mut self) -> Legs {
        match *self {
            Legs::LEFT => Legs::RIGHT,
            Legs::RIGHT => Legs::LEFT,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Eye {
    SMALL,
    BIG,
}

pub struct Dinosaur {
    eye: Eye,
    legs: Legs,
    aabb: AABB,
    state: State,
    speed: Vec2,
    accel: Vec2,
}

impl Dinosaur {
    pub fn new() -> Dinosaur {
        Dinosaur {
            eye: Eye::SMALL,
            legs: Legs::RIGHT,
            state: State::ON_GROUND,
            aabb: AABB::new(
                Vec2::new(DINO_START_POS_X, DINO_START_POS_Y),
                Vec2::new(
                    DINO_START_POS_X + DINO_SIZE_X,
                    DINO_START_POS_Y + DINO_SIZE_Y,
                ),
            ),
            speed: Vec2::new(DINO_START_SPEED_X, DINO_START_SPEED_Y),
            accel: Vec2::new(DINO_ACCEL_SPEED_X, ZERO),
        }
    }
    pub fn shift(&mut self, vector: Vec2) {
        self.aabb.shift(vector);
    }
    pub fn update_speed(&mut self) {
        self.speed += Vec2::new(self.accel.x / FPS, self.accel.y / FPS);
    }
    pub fn up(&mut self) {
        match self.state {
            State::ON_GROUND => {
                self.state = State::IN_AIR;
                self.speed.y = DINO_SPEED_OF_JUMP;
                self.accel.y = DINO_ACCEL_OF_FALLING;
            }
            State::IN_AIR => {
                //TODO: double jump
            }
        }
    }
    pub fn down(&mut self) {
        match self.state {
            State::ON_GROUND => {}
            State::IN_AIR => {
                if self.accel.y == DINO_ACCEL_OF_FALLING {
                    self.accel.y *= 2.0;
                }
            }
        }
    }
    pub fn stop_falling(&mut self) {
        self.aabb = AABB::new(
            Vec2::new(self.aabb().min().x, DINO_START_POS_Y),
            Vec2::new(self.aabb().max().x, DINO_START_POS_Y + DINO_SIZE_Y),
        );
        self.speed.y = ZERO;
        self.accel.y = ZERO;
        self.state = State::ON_GROUND;
    }
    pub fn wham(&mut self) {
        self.eye = Eye::BIG;
        self.speed = Vec2::new(ZERO, ZERO);
    }
    pub fn speed(&self) -> Vec2 {
        self.speed
    }

    pub fn aabb(&self) -> AABB {
        self.aabb
    }
    pub fn state(&self) -> State {
        self.state
    }
    pub fn eye(&self) -> Eye {
        self.eye
    }
    pub fn legs(&self) -> Legs {
        self.legs
    }
    pub fn next_leg(&mut self) {
        self.legs = self.legs().next();
    }
}
