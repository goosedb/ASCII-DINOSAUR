use consts::WIDTH;
use cactus::Cactus;

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
}
