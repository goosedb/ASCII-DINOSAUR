use consts::WIDTH;
use cactus::Cactus;
use render::Render;
use dinosaur::Dinosaur;
use background::Background;
use std::time::{Duration, Instant};

pub struct GameState {
    pub score: i32,
    pub frame: i32,
    pub now: Instant,
    pub after: Instant,
    pub render: Render,
    pub dinosaur: Dinosaur,
    pub cactuses: Vec<Vec<Cactus>>,
    pub background: Background,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            score: 0,
            frame: 0,
            now: Instant::now(),
            after: Instant::now(),
            render: Render::new(),
            dinosaur: Dinosaur::new(),
            cactuses: vec![Cactus::new_group(WIDTH)],
            background: Background::new(),
        }
    }
    pub fn clean_cactuses(&mut self) {
        for i in 0..self.cactuses.len() {
            for j in 0..self.cactuses[i].len() {
                if self.cactuses[i][j].get_max().x < self.render.camera.get_min().x {
                    self.cactuses[i].remove(j);
                }
            }
            if self.cactuses[i].len() == 0 {
                self.cactuses.remove(i);
            }
        }
    }
}
