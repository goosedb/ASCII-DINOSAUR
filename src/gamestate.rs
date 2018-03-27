use vec2::Vec2;
use aabb::AABB;
use tcod::input::*;
use sprite::Sprite;
use consts::{FPS, ZERO};
use tcod::console::Root;
use dinosaur::{Dinosaur, Eye, Legs, State};
use consts::{HEIGHT, WIDTH};
use consts::{CLOUDS, GROUND};
use rand::{thread_rng, Rng};
use cactus::{Cactus, CactusSize};
use consts::{DINO_SIZE_X, DINO_SIZE_Y, SPRITE_DINO};
use consts::{SPRITE_CACTUS_LARGE, SPRITE_CACTUS_SMALL};
use consts::{SPRITE_CLOUDS, SPRITE_GROUND_BOT, SPRITE_GROUND_TOP};
use consts::{SPRITE_CLOUDS_SIZE_XY, SPRITE_GROUND_BOT_SIZE, SPRITE_GROUND_TOP_SIZE};
use consts::{CACTUS_LARGE_SIZE_X, CACTUS_LARGE_SIZE_Y, CACTUS_SMALL_SIZE_X, CACTUS_SMALL_SIZE_Y};

pub struct Gamestate {
    camera: AABB,
    gameover: bool,
    dinosaur: Dinosaur,
    cactuses: Vec<Vec<Cactus>>,
}

impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            gameover: false,
            dinosaur: Dinosaur::new(),
            cactuses: vec![Cactus::new(WIDTH)],
            camera: AABB::new(Vec2::new(ZERO, ZERO), Vec2::new(WIDTH, HEIGHT)),
        }
    }
    pub fn is_dino_alive(&self) -> bool {
        !self.gameover
    }
    pub fn input(&mut self, window: &mut Root) {
        while let Some(e) = window.check_for_keypress(KEY_PRESSED) {
            if e.pressed {
                match e.code {
                    KeyCode::Up => {
                        self.dinosaur.up();
                    }
                    KeyCode::Down => {
                        self.dinosaur.down();
                    }
                    _ => {}
                }
            }
        }
    }
    pub fn frame(&mut self) {
        self.update_state();
        self.detect_collision();
        self.create_new_cactuses();
    }
    pub fn get_background_sprite(&self) -> Sprite {
        let mut sprite = Sprite::new(Vec2::new(ZERO, ZERO), Vec2::new(WIDTH, HEIGHT));
        //================================//
        {
            let start_gr_top = self.camera.min().x as usize % SPRITE_GROUND_TOP_SIZE;
            for i in 0..sprite.size().x as i32 {
                sprite.set_pixel(
                    Vec2::new(i as f64, GROUND),
                    SPRITE_GROUND_TOP[(start_gr_top + i as usize) % SPRITE_GROUND_TOP_SIZE],
                )
            }
        }
        //================================//
        {
            let start_gr_bot = self.camera.min().x as usize % SPRITE_GROUND_BOT_SIZE;
            for i in 0..sprite.size().x as i32 {
                sprite.set_pixel(
                    Vec2::new(i as f64, GROUND + 1.0),
                    SPRITE_GROUND_BOT[(start_gr_bot + i as usize) % SPRITE_GROUND_BOT_SIZE],
                );
            }
        }
        //================================//
        {
            let start_clouds = (self.camera.min().x / 20.0) as usize % SPRITE_CLOUDS_SIZE_XY.0;
            let size_x = sprite.size().x;
            for i in 0..size_x as i32 {
                for j in 0..SPRITE_CLOUDS_SIZE_XY.1 {
                    sprite.set_pixel(
                        Vec2::new(i as f64, CLOUDS + j as f64),
                        SPRITE_CLOUDS[(SPRITE_CLOUDS_SIZE_XY.0 * j as usize
                                          + (start_clouds + i as usize) % SPRITE_CLOUDS_SIZE_XY.0)],
                    );
                }
            }
        }
        //================================//
        sprite
    }
    pub fn get_dinosaur_sprite(&self) -> Sprite {
        let mut sprite = Sprite::new(
            self.dinosaur.aabb().min() - self.camera.min(),
            Vec2::new(DINO_SIZE_X, DINO_SIZE_Y),
        );
        for y in 0..DINO_SIZE_Y as i64 {
            for x in 0..DINO_SIZE_X as i64 {
                sprite.set_pixel(
                    Vec2::new(x as f64, y as f64),
                    SPRITE_DINO[(x as f64 + y as f64 * DINO_SIZE_X) as usize],
                );
            }
        }
        match self.dinosaur.eye() {
            Eye::SMALL => {
                sprite.set_pixel(Vec2::new(8.0, 0.0), 'o');
            }
            Eye::BIG => {
                sprite.set_pixel(Vec2::new(8.0, 0.0), 'O');
            }
        }
        match self.dinosaur.legs() {
            Legs::LEFT => {
                sprite.set_pixel(Vec2::new(4.0, 6.0), 'I');
                sprite.set_pixel(Vec2::new(4.0, 7.0), 'L');
                sprite.set_pixel(Vec2::new(6.0, 6.0), 'L');
            }
            Legs::RIGHT => {
                sprite.set_pixel(Vec2::new(6.0, 6.0), 'I');
                sprite.set_pixel(Vec2::new(6.0, 7.0), 'L');
                sprite.set_pixel(Vec2::new(4.0, 6.0), 'L');
            }
        }
        sprite
    }
    pub fn get_cactuses_sprite(&self) -> Sprite {
        let mut sprite = Sprite::new(Vec2::new(ZERO, ZERO), Vec2::new(WIDTH, HEIGHT));
        let mut cactuses: Vec<Cactus> = Vec::new();
        for i in 0..self.cactuses.len() {
            for j in 0..self.cactuses[i].len() {
                if self.cactuses[i][j].aabb().max().x >= self.camera.min().x {
                    cactuses.push(self.cactuses[i][j]);
                }
            }
        }
        for i in 0..cactuses.len() {
            match cactuses[i].size() {
                CactusSize::SMALL => for y in 0..CACTUS_SMALL_SIZE_Y as i64 {
                    for x in 0..CACTUS_SMALL_SIZE_X as i64 {
                        let coord_y = cactuses[i].aabb().min().y + y as f64;
                        let coord_x = cactuses[i].aabb().min().x + x as f64;
                        let rel_x = coord_x - self.camera.min().x;
                        let rel_y = coord_y - self.camera.min().y;
                        if rel_x >= 0.0 && rel_x < sprite.size().x {
                            sprite.set_pixel(
                                Vec2::new(rel_x, rel_y),
                                SPRITE_CACTUS_SMALL[(x + y * CACTUS_SMALL_SIZE_X as i64) as usize],
                            );
                        }
                    }
                },
                CactusSize::LARGE => for y in 0..CACTUS_LARGE_SIZE_Y as i64 {
                    for x in 0..CACTUS_LARGE_SIZE_X as i64 {
                        let coord_y = cactuses[i].aabb().min().y + y as f64;
                        let coord_x = cactuses[i].aabb().min().x + x as f64;
                        let rel_x = coord_x - self.camera.min().x;
                        let rel_y = coord_y - self.camera.min().y;
                        if rel_x >= 0.0 && rel_x < sprite.size().x {
                            sprite.set_pixel(
                                Vec2::new(rel_x, rel_y),
                                SPRITE_CACTUS_LARGE[(x + y * CACTUS_LARGE_SIZE_X as i64) as usize],
                            );
                        }
                    }
                },
            }
        }
        sprite
    }
    pub fn get_score_sprite(&self) -> Sprite {
        let score = self.get_score();
        let mut score = "score: ".to_string() + &score.to_string();
        let size_x = score.len() as f64 + 2.0;
        let size_y = 3.0;
        let mut sprite = Sprite::new(
            Vec2::new(WIDTH - (size_x + 1.0), CLOUDS),
            Vec2::new(size_x, size_y),
        );
        for i in 0..size_y as i64 {
            sprite.set_pixel(Vec2::new(0.0, i as f64), '|');
            sprite.set_pixel(Vec2::new(size_x - 1.0, i as f64), '|');
        }
        for i in 0..size_x as i64 {
            sprite.set_pixel(Vec2::new(i as f64, 0.0), '-');
            sprite.set_pixel(Vec2::new(i as f64, 2.0), '-');
        }
        for i in 0..score.len() as i64 {
            sprite.set_pixel(
                Vec2::new(size_x - (i as f64 + 2.0), 1.0),
                match score.pop() {
                    Some(n) => n,
                    None => 'x',
                },
            );
        }
        sprite
    }
    fn update_state(&mut self) {
        self.dinosaur.update_speed();
        self.shift();
        if self.dinosaur.aabb().min().x % 10.0 < 1.0 && self.dinosaur.state() != State::IN_AIR {
            self.dinosaur.next_leg();
        }
    }
    fn detect_collision(&mut self) {
        let dinosaur = AABB::new(
            Vec2::new(
                self.dinosaur.aabb().min().x + 2.0,
                self.dinosaur.aabb().min().y,
            ),
            Vec2::new(
                self.dinosaur.aabb().max().x - 2.0,
                self.dinosaur.aabb().max().y - 2.0,
            ),
        );
        for i in 0..self.cactuses.len() {
            for j in 0..self.cactuses[i].len() {
                let cactus = self.cactuses[i][j].aabb();
                if dinosaur.vs_aabb(cactus) {
                    self.game_over();
                }
            }
        }
        let dinosaur = self.dinosaur.aabb();
        let ground = AABB::new(
            Vec2::new(self.camera.min().x, GROUND),
            Vec2::new(self.camera.max().x, HEIGHT),
        );
        if dinosaur.vs_aabb(ground) {
            self.dinosaur.stop_falling();
        }
    }
    fn create_new_cactuses(&mut self) {
        let i = self.cactuses.len();
        let j = self.cactuses[i - 1].len();
        if self.camera.max().x - self.cactuses[i - 1][j - 1].aabb().max().x > WIDTH / 4.0
            && self.camera.max().x as i64 % 10 == 0
        {
            match thread_rng().gen_range(0, 5) {
                1 => self.cactuses.push(Cactus::new(self.camera.max().x)),
                _ => {}
            }
        }
    }
    fn shift(&mut self) {
        let speed = self.dinosaur.speed();
        let step = Vec2::new(speed.x / FPS, speed.y / FPS);
        self.dinosaur.shift(step);
        self.camera.shift(Vec2::new(step.x, 0.0));
    }
    fn game_over(&mut self) {
        self.gameover = true;
        self.dinosaur.wham();
    }
    pub fn get_score(&self) -> i64 {
        (self.dinosaur.aabb().min().x / 10.0) as i64
    }
}
