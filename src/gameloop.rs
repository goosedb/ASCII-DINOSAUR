use std::thread;
use std::time::{Duration, Instant};
use tcod::console::Root;
use tcod::Console;
use dinosaur::Body;
use consts::{FRAMES_PER_SECOND, MILLIS_IN_CESOND};
use gamestate::GameState;
use tcod::input::{Key, KeyCode, KEY_PRESSED};
use tcod::input::KeyCode::{Down, Escape, Left, Right, Up};

pub fn logic(console: &mut Root, gamestate: &mut GameState) {
    sleep(gamestate);
    let keypress = console.check_for_keypress(KEY_PRESSED);
    let mut b = false;
    match keypress {
        Some(keypress) => {
            let button = keypress.code;
            if button == KeyCode::Up {
                gamestate.dinosaur.up();
            }
            if button == KeyCode::Down {
                gamestate.dinosaur.down();
            }
        }
        None => {
            if gamestate.dinosaur.body == Body::CROUCH { println!("NONE!");}
            gamestate.dinosaur.up();
        }
    }
}
pub fn render(console: &mut Root, gamestate: &mut GameState) {
    gamestate.render.clear();
    gamestate.render.put_sprite(&gamestate.dinosaur.get_sprite(), gamestate.dinosaur.get_position());
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
