use consts::{FRAMES_PER_SECOND, WIDTH};
use cactus::Cactus;
use render::Render;
use dinosaur::Dinosaur;
use background::Background;
use std::time::Instant;

pub struct GameState {
    pub score: i32,
    pub frame: i32,
    pub gameloop: bool,
    pub no_pressed_buttons: u32,
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
            gameloop: true,
            no_pressed_buttons: 0,
            now: Instant::now(),
            after: Instant::now(),
            render: Render::new(),
            dinosaur: Dinosaur::new(),
            cactuses: vec![Cactus::new_group(WIDTH)],
            background: Background::new(),
        }
    }
    pub fn clean_cactuses(&mut self) {
        let mut hollow_i: Vec<usize> = vec![];
        for i in 0..self.cactuses.len() {
            let mut hollow_j: Vec<usize> = vec![];
            for j in 0..self.cactuses[i].len() {
                if self.cactuses[i][j].get_max().x < self.render.camera.get_min().x {
                    hollow_i.push(j);
                }
            }
            for j in 0..hollow_j.len() {
                self.cactuses[i].remove(hollow_j[j]);
            }
            if self.cactuses[i].len() == 0 {
                hollow_i.push(i);
            }
        }
        for i in 0..hollow_i.len() {
            self.cactuses.remove(hollow_i[i]);
        }
    }
    pub fn move_it(&mut self) {
        let speed = self.dinosaur.get_speed();
        let move_x = speed.x / FRAMES_PER_SECOND as f64;
        let move_y = speed.y / FRAMES_PER_SECOND as f64;
        self.dinosaur.move_it(move_x, move_y);
        self.background.move_it(move_x, move_y);
        self.render.camera.move_it(move_x, move_y);
    }
}
