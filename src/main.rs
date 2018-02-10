
extern crate pancurses;
extern crate rand;

mod aabb;
mod dino;
mod coord;
mod camera;
mod sprite;
mod consts;
mod renderer;
mod background;
mod cactus;
mod stages;
mod gameloop;

use stages::*;

fn setting(win: &mut pancurses::Window) {
    pancurses::noecho();
    pancurses::curs_set(0);
    win.nodelay(true);
    win.keypad(true);
}

fn main() {
    let mut window = pancurses::initscr();
    setting(&mut window);

    let mut go = false;

    start(&mut window);
    while go {
        game(&mut window);
        finish(&mut go);
    }

    pancurses::endwin();
}
