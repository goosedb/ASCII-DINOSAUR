use consts::WIDTH;
use cactus::Cactus;
use render::Render;

pub struct GameState {
    pub score: i32,
    pub render: Render,
    pub dinosaur: Dinosaur,
    pub cactuses: Vec<Vec<Cactus>>,
    pub background: Background,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            score: 0,
            render: Render::new(),
            dinosaur: Dinosaur::new(),
            cactuses: vec![Cactus::new_group(WIDTH)],
            background: Background::new(),
        }
    }
    pub fn clean_cactuses(&mut self) {
        for i in self.cactuses {
            for j in i {
                if j.get_max().x < self.render.camera.get_min().x {
                   j.is_in_scope = false;
                }
            }
            i.retain(|&x| x.is_in_scope);
        }
        self.cactuses.retain(|&x| x.len() != 0);
    }
}
