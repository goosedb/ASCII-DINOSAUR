use aabb::*;
use dino::*;
use coord::*;
use consts::*;
use cactus::*;
use renderer::*;
use background::*;
use pancurses::{Input, Window};

pub enum Event {
    NONE,
    UP,
    DOWN,
}

pub fn input(win: &mut Window) -> Event {
    match win.getch() {
        Some(Input::Character(c)) => {
            if c == 's' {
                Event::DOWN
            } else if c == 'w' {
                Event::UP
            } else {
                Event::NONE
            }
        }
        Some(input) => Event::NONE,
        None => Event::NONE,
    }
}
pub fn logic(
    win: &mut Window,
    renderer: &mut Renderer,
    dino: &mut Dino,
    cactuses: &Vec<Cactus>,
    background: &mut Background,
    running: &mut bool,
    score: &mut i32,
    time_to_step : &mut i32
) {
    //INPUT
    match input(win) {
        Event::UP => {
            dino.up();
        }
        Event::DOWN => {
            dino.down();
        }
        Event::NONE => {
            dino.straight();
        }
    }
    //MOVEING
    *time_to_step = (*time_to_step + 1) % 10;
    if *time_to_step == 0 {
        dino.step();
    }
    dino.frame();
    renderer.camera.move_it(dino.speed.x);
    background.move_it(dino.speed.x);
    *score = dino.get_position().x as i32 / 5;
    //COLLISIONS
    for i in cactuses {
        if detect_collision(dino.collision_model, i.collision_model) {
            dino.wham();
            *running = false;
        }
    }
}
pub fn render(
    renderer: &mut Renderer,
    dino: &Dino,
    cactuses: &Vec<Cactus>,
    background: &Background,
) {
    renderer.clear();
    let pos_of_camera = renderer.camera.get_border().min;
    renderer.put_sprite(&background.get_sprite(), Coord::new(pos_of_camera.x as i32, pos_of_camera.y as i32));
    for i in cactuses {
        renderer.put_sprite(&i.get_sprite(), i.get_position());
    }
    renderer.put_sprite(&dino.get_sprite(), dino.get_position());
}
pub fn draw(win: &mut Window, renderer: &Renderer, score: i32) {
    win.erase();

    win.mv(0, 1);
    for _ in 0..WIDTH {
        win.printw("-");
    }
    win.mv(HEIGHT + 1, 1);
    for _ in 0..WIDTH {
        win.printw("-");
    }
    for i in 1..HEIGHT + 1 {
        win.mv(i, 0);
        win.printw("|");
        win.mv(i, WIDTH + 2);
        win.printw("|");
    }
    win.mv(0, 0);
    win.printw("+");
    win.mv(0, WIDTH + 2);
    win.printw("+");
    win.mv(HEIGHT + 1, 0);
    win.printw("+");
    win.mv(HEIGHT + 1, WIDTH + 2);
    win.printw("+");

    renderer.present(win);

    let score = "score: ".to_string() + &score.to_string();
    win.mv(1, WIDTH + 1 - score.len() as i32);
    win.printw(&score);

    win.refresh();
}

fn detect_collision(dino: AABB, cactus: AABB) -> bool {
    if dino.max.x < cactus.min.x || dino.min.x > cactus.max.x {
        false
    } else if dino.max.y < cactus.min.y || dino.min.y > cactus.max.y {
        false
    } else {
        true
    }
}
