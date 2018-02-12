use tcod::console::Root;
use gamestate::GameState;
use gameloop::{draw, logic, render};

pub fn start(console: &mut Root, mainloop : &mut bool) {}

pub fn game(console: &mut Root, mainloop : &mut bool) {
    let mut gameloop = true;

    let mut gamestate = GameState::new();

    while gameloop {
        logic(console, &mut gamestate, mainloop);
        render(console, &mut gamestate);
        draw(console, &mut gamestate);
    }
}

pub fn finish(console: &mut Root, mainloop : &mut bool) {}
