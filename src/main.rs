#![feature(duration_extras)]
extern crate rand;
extern crate tcod;

mod vec2;
mod aabb;
mod sprite;
mod cactus;
mod stages;
mod consts;
mod dinosaur;
mod renderer;
mod gamestate;

use renderer::Renderer;
use tcod::console::Root;
use stages::{finish, game};
use consts::{HEIGHT, WIDTH};

fn main() {
    let mut exit = false;
    let mut renderer = Renderer::new();
    let mut window = Root::initializer()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("DINO")
        .init();
    while !window.window_closed() && !exit {
        game(&mut window, &mut renderer);
        finish(&mut window, &mut exit);
    }
}
