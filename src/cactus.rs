use vec2::Vec2;
use aabb::AABB;
use rand::{thread_rng, Rng};
use consts::ZERO;
use consts::GROUND;
use consts::{CACTUS_SMALL_SIZE_X, CACTUS_SMALL_SIZE_Y};
use consts::{CACTUS_LARGE_SIZE_X, CACTUS_LARGE_SIZE_Y};

fn generate() -> Vec2 {
    match thread_rng().gen_range(0, 2) {
        0 => Vec2::new(CACTUS_SMALL_SIZE_X, CACTUS_SMALL_SIZE_Y),
        1 => Vec2::new(CACTUS_LARGE_SIZE_X, CACTUS_LARGE_SIZE_Y),
        _ => Vec2::new(ZERO, ZERO),
    }
}

#[derive(Clone, Copy)]
pub enum CactusSize {
    SMALL,
    LARGE,
}

#[derive(Clone, Copy)]
pub struct Cactus {
    aabb: AABB,
    size: CactusSize,
}

impl Cactus {
    pub fn new(mut x_coord: f64) -> Vec<Cactus> {
        let number = [1, 1, 2][thread_rng().gen_range(0, 3)];
        let mut cactuses: Vec<Cactus> = vec![];
        for _ in 0..number {
            let cactus = generate();
            let size = if cactus.y == CACTUS_LARGE_SIZE_Y {
                CactusSize::LARGE
            } else {
                CactusSize::SMALL
            };
            cactuses.push(Cactus {
                aabb: AABB::new(
                    Vec2::new(x_coord, GROUND - cactus.y),
                    Vec2::new(x_coord + cactus.x, GROUND),
                ),
                size: size,
            });
            x_coord += cactus.x + 1.0;
        }
        cactuses
    }
    pub fn aabb(&self) -> AABB {
        self.aabb
    }
    pub fn size(&self) -> CactusSize {
        self.size
    }
}
