use tcod::RootConsole;
use gamestate::GameState;
use gameloop::{draw, logic, render};

pub fn start(console: &mut RootConsole) {}

pub fn game(console: &mut RootConsole) {
    let mut gameloop = true;

    let mut gamestate = GameState::new();

    while gameloop {
        logic();
        render();
        draw(console);
    }
}

pub fn finish(console: &mut RootConsole, mainloop: &mut bool) {}
