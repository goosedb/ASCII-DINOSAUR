use sprite::*;
use coord::*;
use consts::{HEIGHT, WIDTH};

pub struct Background {
    ground: Sprite,
    clouds: Sprite,
    gr_position: i32,
    cl_position: i32,
}

impl Background {
    pub fn new() -> Background {
        Background {
            ground: Sprite::new(
                vec![
                    '~', '~', '-', '-', '-', '~', '~', '-', '-', '~', '-', 'v', '-', '~', '~', '~',
                    '~', '-', '-', '~', '~', '^', '^', '-', '\'', ' ', '\'', ' ', '.', ' ', '`',
                    ' ', '\'', ' ', '\'', ' ', '\'', ' ', '.', ' ', ' ', '.', '`', ' ', ' ', '.',
                    ' ', '\'',
                ],
                Coord::new(24, 2),
            ),
            clouds: Sprite::new(
                vec![
                    ' ', ' ', '.', '.', ' ', '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\'', '\'', ' ', '\'', '\'', '\'', '\'',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', '.', '.', ' ', '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '`', '`',
                    '`', '`', '`', '`', '`', '`', '`', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', '\'', '\'', ' ', '\'', '\'', '\'', '\'', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '.', '.', ' ',
                    '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '`', '`', '`',
                    '`', '`', '`', '`', '`', '`', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', '\'', '\'', ' ', '\'', '\'', '\'', '\'', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '`', '`', '`', '`', '`', '`', '`',
                    '`', '`', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ],
                Coord::new(53, 6),
            ),
            gr_position: 0,
            cl_position: 0,
        }
    }
    pub fn move_clouds(&mut self) {
        self.cl_position = (self.cl_position + 1) % self.clouds.get_size().x;
    }
    pub fn move_ground(&mut self) {
        self.gr_position = (self.gr_position + 1) % self.ground.get_size().x;
    }
    pub fn get_sprite(&self) -> Sprite {
        let mut sprite = Sprite::new(vec![' '], Coord::new(WIDTH, HEIGHT));
        for i in 0..WIDTH * self.clouds.get_size().y {
            sprite.set_pixel(
                self.clouds
                    .get_pixel((self.cl_position + i) % self.clouds.get_size().x),
                Coord::new(i % WIDTH, ((i - (i % WIDTH)) / WIDTH) + 1),
            );
        }
        for i in 0..WIDTH * self.ground.get_size().y {
            sprite.set_pixel(
                self.ground
                    .get_pixel((self.gr_position + i) % self.ground.get_size().x),
                Coord::new(i % WIDTH, HEIGHT - self.ground.get_size().y + ((i - (i % WIDTH)) / WIDTH)),
            );
        }
        sprite
    }
}
