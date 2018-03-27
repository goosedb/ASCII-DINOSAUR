use vec2::Vec2;
use sprite::Sprite;
use tcod::Console;
use tcod::console::Root;
use consts::{HEIGHT, WIDTH};

pub struct Renderer {
    buffer: [char; (WIDTH * HEIGHT) as usize],
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            buffer: [' '; (WIDTH * HEIGHT) as usize],
        }
    }
    pub fn clear(&mut self) {
        self.buffer = [' '; (WIDTH * HEIGHT) as usize];
    }
    pub fn put_sprite(&mut self, sprite: Sprite) {
        let pos = sprite.position();
        for y in 0..sprite.size().y as i64 {
            for x in 0..sprite.size().x as i64 {
                let rel_x = (pos.x + x as f64) as usize;
                let rel_y = (pos.y + y as f64) as usize;
                if sprite.get_pixel(Vec2::new(x as f64, y as f64)) == ' ' {
                    continue;
                }
                self.buffer[rel_x + rel_y * WIDTH as usize] =
                    sprite.get_pixel(Vec2::new(x as f64, y as f64));
            }
        }
    }
    pub fn present(&self, window: &mut Root) {
        for y in 0..HEIGHT as i32 {
            for x in 0..WIDTH as i32 {
                window.set_char(x, y, self.buffer[(x + y * WIDTH as i32) as usize]);
            }
        }
    }
}
