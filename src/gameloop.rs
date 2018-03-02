use std::thread;
use std::time::{Duration, Instant};
use tcod::Console;
use tcod::console::Root;
use dinosaur::Dinosaur;
use consts::{FRAMES_PER_SECOND, MILLIS_IN_CESOND};
use gamestate::GameState;
use tcod::input::{KeyCode, KEY_PRESSED, KEY_RELEASED};

pub fn logic(console: &mut Root, gamestate: &mut GameState) {
    sleep(gamestate);
    input(console, &mut gamestate.dinosaur);
    gamestate.move_it();
    gamestate.clean_cactuses();
    if gamestate.frame % 10 == 0 {
        gamestate.dinosaur.step();
    }
    gamestate.frame = (gamestate.frame + 1) % FRAMES_PER_SECOND;
}
pub fn render(console: &mut Root, gamestate: &mut GameState) {
    gamestate.render.clear();
    gamestate.render.put_sprite(
        &gamestate.dinosaur.get_sprite(),
        gamestate.dinosaur.get_position(),
    );
}
pub fn draw(console: &mut Root, gamestate: &mut GameState) {
    console.clear();
    gamestate.render.present(console);
    console.flush();
}

fn sleep(gamestate: &mut GameState) {
    gamestate.after = Instant::now();
    let sleep = (MILLIS_IN_CESOND / FRAMES_PER_SECOND) as i64
        - (gamestate.after.duration_since(gamestate.now).subsec_nanos() / 1_000_000) as i64;
    if sleep > 0 {
        thread::sleep(Duration::from_millis(sleep as u64));
    }
    gamestate.now = Instant::now();
}

fn input(console: &mut Root, dinosaur: &mut Dinosaur) {
    while let Some(e) = console.check_for_keypress(KEY_PRESSED | KEY_RELEASED) {
        if e.pressed {
            match e.code {
                KeyCode::Up => dinosaur.up(),
                KeyCode::Down => dinosaur.down(),
                _ => {}
            }
        } else {
            match e.code {
                KeyCode::Down => {
                    dinosaur.straight();
                }
                _ => {}
            }
        }
    }
}
