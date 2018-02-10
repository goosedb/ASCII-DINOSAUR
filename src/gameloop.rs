use dino::*;
use coord::*;
use consts::*;
use cactus::*;
use renderer::*;
use background::*;
use pancurses::{Input, Window};

pub fn input() {}
pub fn logic() {}
pub fn render(
    renderer: &mut Renderer,
    dino: &Dino,
    cactuses: &Vec<Cactus>,
    background: &Background,
) {
    renderer.put_sprite(&background.get_sprite(), Coord::new(0, 0));
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
    win.mv(HEIGHT + 2, 1);
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
    win.mv(HEIGHT + 2, 0);
    win.printw("+");
    win.mv(0, WIDTH + 2);
    win.printw("+");
    win.mv(HEIGHT + 2, WIDTH + 2);
    win.printw("+");

    let score = "score: ".to_string() + &score.to_string();
    win.mv(2, WIDTH + 2 - score.len() as i32);
    win.printw(&score);

    renderer.present(win);

    win.refresh();
}
