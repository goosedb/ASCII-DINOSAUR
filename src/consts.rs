use std::time::Duration;

pub const WIDTH: i32 = 100;
pub const HEIGHT: i32 = 20;
pub const PERFECT_TICK: i32 = 20;
pub const PERFECT_TICK_DUR: Duration = Duration::from_millis(PERFECT_TICK as u64);

pub const GROUND: i32 = HEIGHT - 1;
