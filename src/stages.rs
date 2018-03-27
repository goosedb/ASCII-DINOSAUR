use std::thread;
use renderer::Renderer;
use tcod::Console;
use tcod::input::*;
use tcod::console::Root;
use gamestate::Gamestate;
use consts::{HEIGHT, WIDTH};
use consts::{FPS, MILLIS_IN_SEC};
use std::time::{Duration, Instant};


pub fn game(window: &mut Root, renderer: &mut Renderer) {
    let mut gamestate = Gamestate::new();
    let mut start_frame = Instant::now();
    let mut finish_frame = Instant::now();
    while !window.window_closed() {
        //===logic===//
        gamestate.input(window);
        gamestate.frame();
        //===render===//
        renderer.clear();
        renderer.put_sprite(gamestate.get_background_sprite());
        renderer.put_sprite(gamestate.get_cactuses_sprite());
        renderer.put_sprite(gamestate.get_dinosaur_sprite());
        renderer.put_sprite(gamestate.get_score_sprite());
        //===drawing===//
        window.clear();
        renderer.present(window);
        window.flush();
        //===sleep===//
        sleep(&mut start_frame, &mut finish_frame);
        if !gamestate.is_dino_alive() {
            break;
        }
    }
}

pub fn finish(window: &mut Root, exit: &mut bool) {
    let gameover = ['G', 'A', 'M', 'E', ' ', 'O', 'V', 'E', 'R'];
    let alert = ['e','s','c',' ','t','o',' ','e','x','i','t',' ','e','n','t','e','r',' ','t','o',' ','t','r','y',' ','a','g','a','i','n'];
    for x in 0..gameover.len() {
        window.set_char(
            (WIDTH as i32 / 2 - (gameover.len() / 2) as i32) + x as i32,
            HEIGHT as i32 / 2 - 1 as i32,
            gameover[x],
        );
    }
    for x in 0..alert.len() {
        window.set_char(
            (WIDTH as i32 / 2 - (alert.len() / 2) as i32) + x as i32,
            HEIGHT as i32 / 2 + 1 as i32,
            alert[x],
        );
    }
    window.flush();
    'stop: while !window.window_closed() {
        while let Some(e) = window.check_for_keypress(KEY_PRESSED) {
            if e.pressed {
                match e.code {
                    KeyCode::Enter => break 'stop,
                    KeyCode::Escape => {
                        *exit = true;
                        break 'stop;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn sleep(start_frame: &mut Instant, finish_frame: &mut Instant) {
    *finish_frame = Instant::now();
    let frame_time = finish_frame.duration_since(*start_frame).subsec_millis() as f64;
    if frame_time < MILLIS_IN_SEC / FPS {
        thread::sleep(Duration::from_millis(
            (MILLIS_IN_SEC / FPS - frame_time) as u64,
        ));
    }
    *start_frame = Instant::now();
}
