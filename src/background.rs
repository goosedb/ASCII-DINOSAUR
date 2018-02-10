use sprite::*;
use coord::*;
use consts::*;
use consts::{HEIGHT, WIDTH};

pub struct Background {
    ground: Sprite,
    pub clouds: Sprite,
    gr_position: f32,
    cl_position: f32,
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
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', '\'', '\'', ' ', '\'', '\'', '\'', '\'', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', '.', '.', ' ', '.', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', '`', '`', '`', ' ', ' ', '`', '`', ' ', '`', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', '.', '.', ' ', '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\'', '\'', ' ', '\'', ' ', '\'', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', '\'', '\'', ' ', '\'', '\'', '\'', '\'', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '`', ' ', '`', '`', '`', '`',
                    '`', '`', '`', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '`', '`', '`', '`', '`', '`', '`', '`',
                    '`', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ],
                Coord::new(81, 5),
            ),
            gr_position: 0.0,
            cl_position: 0.0,
        }
    }
    pub fn move_it(&mut self, speed: i32) {
        self.cl_position += speed as f32 / FPS as f32 / 100.0;
        if self.cl_position > self.clouds.get_size().x as f32 {
            self.cl_position -= self.clouds.get_size().x as f32;
        }
        self.gr_position += speed as f32 / FPS as f32;
        if self.gr_position > self.ground.get_size().x as f32 {
            self.gr_position -= self.ground.get_size().x as f32;
        }
        self.cl_position %= self.clouds.get_size().x as f32;
        self.gr_position %= self.ground.get_size().x as f32;
    }
    pub fn get_sprite(&self) -> Sprite {
        let mut sprite = Sprite::new(
            vec![' '; (WIDTH * HEIGHT) as usize],
            Coord::new(WIDTH, HEIGHT),
        );
        for i in 0..self.clouds.get_size().y {
            for j in 0..WIDTH {
                sprite.set_pixel(
                    self.clouds.get_pixel(
                        i * self.clouds.get_size().x
                            + (self.cl_position as i32 + j) % self.clouds.get_size().x,
                    ),
                    Coord::new(j, i),
                );
            }
        }
        for i in 0..self.ground.get_size().y {
            for j in 0..WIDTH {
                sprite.set_pixel(
                    self.ground.get_pixel(
                        i * self.ground.get_size().x
                            + (self.gr_position as i32 + j) % self.ground.get_size().x,
                    ),
                    Coord::new(j, HEIGHT - self.ground.get_size().y + i),
                );
            }
        }
        sprite
    }
}
