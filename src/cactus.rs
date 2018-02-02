extern crate rand;

use aabb::AABB;
use rand::Rng;
use coord::Coord;
use consts::*;
use camera::Camera;
use sprite::Sprite;

#[derive(Clone, Copy)]
enum Size {
    SMALL,
    BIG,
}

impl Size {
    fn new(n: i32) -> Size {
        let sizes = [Size::SMALL, Size::BIG];
        sizes[n as usize]
    }
}

pub struct Cactus {
    collision_model: AABB,
    size: Size,
}

impl Cactus {
    pub fn new(camera: &Camera) -> Cactus {
        let s = /*rand::thread_rng().gen_range(0, 2);*/ 1; // for beautyfully test
        let mut aabb: AABB = AABB::new(Coord::new(0, 0), Coord::new(0, 0));
        match Size::new(s) {
            Size::SMALL => {
                aabb = AABB::new(
                    Coord::new(camera.get_border().min.x + WIDTH + 5, HEIGHT - 6),
                    Coord::new(camera.get_border().min.x + WIDTH + 11, HEIGHT - 1),
                )
            }
            Size::BIG => {
                aabb = AABB::new(
                    Coord::new(WIDTH + 5, HEIGHT - 9),
                    Coord::new(WIDTH + 13, HEIGHT - 1),
                )
            }
        }
        Cactus {
            collision_model: aabb,
            size: Size::new(s),
        }
    }
    pub fn get_sprite(&self) -> Sprite {
        let mut sprite = Sprite::new(vec![], Coord::new(0, 0));
        match self.size {
            Size::SMALL => {
                sprite = Sprite::new(
                    vec![
                        ' ', ' ', '#', '#', ' ', ' ', '#', ' ', '#', '#', ' ', '#', ' ', '#', '#',
                        '#', '#', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ',
                    ],
                    Coord::new(6, 5),
                )
            }
            Size::BIG => {
                sprite = Sprite::new(
                    vec![
                        ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', '#', '#', ' ', '#', '#', ' ', ' ',
                        ' ', '#', '#', ' ', '#', '#', ' ', '#', '#', '#', '#', ' ', '#', '#', '#',
                        '#', ' ', ' ', '#', '#', '#', '#', ' ', ' ', ' ', ' ', ' ', '#', '#', '#',
                        ' ', ' ', ' ', ' ', ' ', '#', '#', '#', ' ', ' ', ' ', ' ', ' ', '#', '#',
                        '#', ' ', ' ', ' ',
                    ],
                    Coord::new(8, 8),
                )
            }
        }

        sprite
    }
    pub fn get_position(&self) -> Coord {
        self.collision_model.min
    }
}
