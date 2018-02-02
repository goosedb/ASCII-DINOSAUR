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
use cactus::*;
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
    let cactus = Cactus::new(&renderer.camera);
    println!("{}", background.clouds.sprite.len());
    while running {
        let now = Instant::now();
        renderer.clear();
        let campose_x = renderer.camera.get_border().min.x;
        renderer.put_sprite(&background.get_sprite(), Coord::new(campose_x, 0));
        renderer.put_sprite(&cactus.get_sprite(), cactus.get_position());
        renderer.put_sprite(&dino.get_sprite(), dino.get_position());
        renderer.present(win);
        renderer.camera.move_it(Coord::new(1, 0));
        background.move_ground();
        background.move_clouds();
        dino.step();
        dino.collision_model.min = dino.collision_model.min + Coord::new(1, 0);
        dino.collision_model.max = dino.collision_model.max + Coord::new(1, 0);
        //logic();
        //render();
        //draw();

        let after = Instant::now();
        let sleep = perfect_tick.subsec_nanos() - after.duration_since(now).subsec_nanos();
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
