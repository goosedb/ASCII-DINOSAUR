use camera::Camera;
use sprite::Sprite;
use coord::Coord;
use tcod::Console;
use tcod::console::Root;
use consts::{HEIGHT, WIDTH};

pub struct Render {
    pub camera: Camera,
    render: Vec<char>,
}

impl Render {
    pub fn new() -> Render {
        Render {
            camera: Camera::new(),
            render: vec![' '; (WIDTH * HEIGHT) as usize],
        }
    }
    pub fn present(&mut self, console: &mut Root) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                console.set_char(j as i32, i as i32, self.render[(i * WIDTH + j) as usize])
            }
        }
    }
    pub fn clear(&mut self) {
        for i in 0..self.render.len() {
            self.render[i] = ' ';
        }
    }
    pub fn put_sprite(&mut self, sprite: &Sprite, position: Coord) {
        let relative_x = (position.x - self.camera.get_min().x) as i32;
        let relative_y = (position.y - self.camera.get_min().y) as i32;
        let sprite_width = sprite.size.x as i32;
        let sprite_height = sprite.size.y as i32;
        for y in 0..sprite_height {
            for x in 0..sprite_width {
                if (position.x + x as f64) > self.camera.get_max().x
                    || (position.y + y as f64) > self.camera.get_max().y
                    || (position.x + x as f64) < self.camera.get_min().x
                    || (position.y + y as f64) < self.camera.get_min().y
                {
                    continue;
                }
                let pos_of_pix = (relative_y + y) * WIDTH + relative_x + x;
                let ch = sprite.sprite[(y * sprite_width + x) as usize];
                if ch != ' ' {
                    self.render[pos_of_pix as usize] = ch;
                }
            }
        }
    }
}
