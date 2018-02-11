extern crate rand;
extern crate tcod;

mod aabb;
mod coord;
mod cactus;
mod consts;
mod mainloop;
mod gameloop;
mod gamestate;

use consts::{HEIGHT, TITLE, WIDTH};
use tcod::RootConsole;
use mainloop::{finish, game, start};

fn main() {
    let mut mainloop = true;

    let mut console = RootConsole::initializer()
        .size(WIDTH, HEIGHT)
        .title(TITLE)
        .init();

    while mainloop {
        start(&mut console);
        game(&mut console);
        finish(&mut console, &mut mainloop);
    }
}
