extern crate rand;
extern crate tcod;

mod aabb;
mod coord;
mod cactus;
mod camera;
mod consts;
mod render;
mod sprite;
mod dinosaur;
mod mainloop;
mod gameloop;
mod gamestate;
mod background;

use tcod::console::Root;
use consts::{HEIGHT, TITLE, WIDTH};
use mainloop::{finish, game, start};

fn main() {
    let mut mainloop = true;
    let mut console = Root::initializer().size(WIDTH, HEIGHT).title(TITLE).init();
    while mainloop {
        start(&mut console);
        game(&mut console);
        finish(&mut console, &mut mainloop);
    }
}
