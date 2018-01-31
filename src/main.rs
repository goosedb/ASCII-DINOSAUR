extern crate pancurses;

mod consts;
mod coord;
mod camera;
mod renderer;
mod aabb;
mod sprite;

use coord::*;
use sprite::*;
use consts::*;
use renderer::*;
use std::thread;
use std::time::{Duration, Instant};
use pancurses::*;

fn new_frame(win : &mut pancurses::Window) {
    win.refresh();
    win.erase();
}

fn start(win : &mut pancurses::Window) {
    let perfect_tick = Duration::from_millis(200);

    let mut s = String::new();
    s += "Please set ";
    s += &(WIDTH + 2).to_string();
    s += " collumns and ";
    s += &(HEIGHT + 2).to_string();
    s += " lines\n";
    
    win.mv(win.get_max_y() / 2, win.get_max_x() / 2 - (s.len()) as i32 / 2);
    win.printw(&s);

    loop {
        let now = Instant::now();

        let input = win.getch();
        
        match input {
            Some(Input::Character(c)) => {
                if c == 's' {
                    break;
                }
            }
            Some(Input::KeyDC) => break,
            Some(input) => {
            }
            None => (),
        }

        new_frame(win);

        let after = Instant::now();
        thread::sleep(perfect_tick - after.duration_since(now));
    }
}

fn game(win : &mut pancurses::Window) {
    let mut running = true;
    let perfect_tick = Duration::from_millis(20);
    while running {
        let now = Instant::now();

        let after = Instant::now();
        thread::sleep(perfect_tick - after.duration_since(now));
    }
}

fn main() {
    let mut window = initscr();
    noecho();
    curs_set(0);
    window.keypad(true);
    let mut go = false;

    start(&mut window);
    while go {
        
        game(&mut window);
        //finish();

    }
    endwin();
    window.delwin();
}
