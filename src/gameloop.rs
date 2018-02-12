use std::thread;
use std::time::{Duration, Instant};
use tcod::console::Root;
use tcod::Console;
use consts::{FRAMES_PER_SECOND, MILLIS_IN_CESOND};
use gamestate::GameState;
use tcod::input::{Key, KEY_PRESSED, KeyCode};
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape};

pub fn logic(console: &mut Root, gamestate: &mut GameState, mainloop : &mut bool) {
    sleep(gamestate);
    let keypress = console.check_for_keypress(KEY_PRESSED);
    match keypress {
        Some(keypress) => {
            let button = keypress.code;
            if button == KeyCode::Up {
                println!("up!");
                *mainloop = false;
            }
            if button == KeyCode::Down {
                println!("down!");
            }
        }
        None => {}
    }

}
pub fn render(console: &mut Root, gamestate: &mut GameState) {}
pub fn draw(console: &mut Root, gamestate: &mut GameState) {
    console.clear();
    console.set_char(10, 10, '+');
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
    println!("frame!");
}
