extern crate pancurses;

mod consts;
mod coord;
mod camera;
mod renderer;
mod aabb;
mod sprite;
mod dino;
mod background;

use dino::*;
use background::*;
use coord::*;
use sprite::*;
use consts::*;
use renderer::*;
use std::thread;
use std::time::{Duration, Instant};
use pancurses::*;

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
                if c == ' ' && win.get_max_y() == HEIGHT + 2 && win.get_max_x() == WIDTH + 2 {
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

    while running {
        let now = Instant::now();

        //logic();
        //render();
        //draw();

        let after = Instant::now();
        thread::sleep(perfect_tick - after.duration_since(now));
    }
}

fn finish(go: &mut bool) {}

fn main() {
    let mut window = initscr();
    noecho();
    curs_set(0);
    window.nodelay(true);
    window.keypad(true);
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
