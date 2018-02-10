use consts::*;
use pancurses::{Input, Window};
use std::thread;
use std::time::{Duration, Instant};
use dino::*;
use renderer::*;
use background::*;
use cactus::*;
use gameloop::*;

pub fn start(win: &mut Window) {
    let mut alert1 = String::new();
    alert1 += "Please set ";
    alert1 += &(WIDTH + 2).to_string();
    alert1 += " collumns and ";
    alert1 += &(HEIGHT + 2).to_string();
    alert1 += " lines.\n";
    let note: String = "Press space to play".to_string();

    let mut after = Instant::now();
    let mut now = Instant::now();

    loop {
        win.erase();
        let mut alert2 = String::new();
        alert2 += "Now: ";
        alert2 += &win.get_max_y().to_string();
        alert2 += " lines and ";
        alert2 += &win.get_max_x().to_string();
        alert2 += " collumns";

        print_string(win, &alert1, 0);
        print_string(win, &alert2, 1);
        print_string(win, &note, 2);

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

        after = Instant::now();
        sleep(&mut now, &mut after);
        now = after;
    }
}

pub fn game(win: &mut Window) {
    let mut running = true;

    let mut renderer = Renderer::new();
    let mut background = Background::new();
    let mut dino = Dino::new();
    let mut cactuses = vec![Cactus::new(&renderer.camera)];

    let mut now = Instant::now();
    let mut after = Instant::now();

    let mut score = 0;
    let mut time_to_step = 0;
    while running {
        logic(
            win,
            &mut renderer,
            &mut dino,
            &mut cactuses,
            &mut background,
            &mut running,
            &mut score,
            &mut time_to_step,
        );
        render(&mut renderer, &dino, &cactuses, &background);
        draw(win, &renderer, score);
        sleep(&mut now, &mut after);
    }
}

pub fn finish(win: &mut Window, go: &mut bool) {
    let gameover = vec![
        "                                                            ",
        " GGGGGG AAAAAA M    M EEEEEE   OOOOOO V     V EEEEEE RRRRRR ",
        " G      A    A MM  MM EEEE     O    O  V   V  EEEE   R    R ",
        " G   GG AAAAAA M MM M E        O    O   V V   E      RRRRRR ",
        " GGGGGG A    A M    M EEEEEE   OOOOOO    V    EEEEEE R    R ",
        "                                                            ",
    ];
    for i in 0..gameover.len() {
        win.mv(
            HEIGHT / 2 - 3 + i as i32,
            (WIDTH + 2) / 2 - gameover[i].len() as i32 / 2,
        );
        win.printw(gameover[i]);
    }

    let note = "Press \"p\" to try again and \"q\" to exit.".to_string();
    win.mv(HEIGHT - 7, (WIDTH + 2) / 2 - note.len() as i32 / 2);
    win.printw(&note);

    win.refresh();

    loop {
        match win.getch() {
            Some(Input::Character(c)) => {
                if c == 'p' {
                    break;
                }
                if c == 'q' {
                    *go = false;
                    break;
                }
            }
            Some(input) => {}
            None => (),
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn print_string(win: &mut Window, string: &String, line: i32) {
    win.mv(
        win.get_max_y() / 2 + line,
        win.get_max_x() / 2 - (string.len()) as i32 / 2,
    );
    win.printw(string);
}

fn sleep(now: &mut Instant, after: &mut Instant) {
    let sleep = (PERFECT_TICK_DUR.subsec_nanos() / 1000) as i64
        - (after.duration_since(*now).subsec_nanos() / 1000) as i64;
    if sleep > 0 {
        thread::sleep(PERFECT_TICK_DUR - after.duration_since(*now));
    }
    *now = *after;
}
