use tcod::console::Root;
use gamestate::GameState;
use gameloop::{draw, logic, render};

pub fn start(console: &mut Root) {}

pub fn game(console: &mut Root) {

    let mut gamestate = GameState::new();

    while gamestate.gameloop {
        logic(console, &mut gamestate);
        render(console, &mut gamestate);
        draw(console, &mut gamestate);
    }
}

pub fn finish(console: &mut Root, mainloop : &mut bool) {
    *mainloop = false;
}
