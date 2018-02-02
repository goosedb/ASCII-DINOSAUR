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

use dino::*;
use coord::*;
use sprite::*;
use consts::*;
use renderer::*;
use std::thread;
use pancurses::*;
use background::*;
use std::time::{Duration, Instant};

fn start(win: &mut pancurses::Window) {
    let perfect_tick = Duration::from_millis(50);

    //====================//
    let mut alert = String::new();
    alert += "Please set ";
    alert += &(WIDTH + 2).to_string();
    alert += " collumns and ";
    alert += &(HEIGHT + 2).to_string();
    alert += " lines\n";
    let note = "Press space to play";
    //====================//

    loop {
        win.erase();

        win.mv(
            win.get_max_y() / 2,
            win.get_max_x() / 2 - (alert.len()) as i32 / 2,
        );
        win.printw(&alert);
        win.mv(
            win.get_max_y() / 2 + 1,
            win.get_max_x() / 2 - (note.len()) as i32 / 2,
        );
        win.printw(&note);

        let now = Instant::now();

        match win.getch() {
            Some(Input::Character(c)) => {
                if c == ' '
                /*&& win.get_max_y() == HEIGHT + 2 && win.get_max_x() == WIDTH + 2*/
                {
                    break;
                }
            }
            Some(input) => {}
            None => (),
        }

        win.refresh();

        let after = Instant::now();
        thread::sleep(perfect_tick - after.duration_since(now));
    }
}

fn game(win: &mut pancurses::Window) {
    let mut running = true;
    let perfect_tick = Duration::from_millis(PERFECT_TICK);

    let mut renderer = Renderer::new();
    let mut background = Background::new();
    let mut dino = Dino::new();
    println!("{}", background.clouds.sprite.len());
    while running {
        let now = Instant::now();

        //logic();
        //render();
        //draw();

        let after = Instant::now();
        let sleep = perfect_tick.as_secs() - after.duration_since(now).as_secs();
        if sleep > 0 {
            thread::sleep(perfect_tick - after.duration_since(now));
        }
    }
}

fn finish(go: &mut bool) {}

fn setting(win: &mut pancurses::Window) {
    noecho();
    curs_set(0);
    win.nodelay(true);
    win.keypad(true);
}

fn main() {
    let mut window = initscr();
    setting(&mut window);

    let mut go = true;

    start(&mut window);
    window.erase();
    while go {
        game(&mut window);
        finish(&mut go);
    }
    endwin();
    window.delwin();
}
