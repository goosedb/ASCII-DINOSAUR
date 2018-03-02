use rand;
use rand::Rng;
use aabb::AABB;
use coord::Coord;
use sprite::Sprite;
use consts::{CACTUS_BIG_SIZE_X, CACTUS_BIG_SIZE_Y, CACTUS_SMALL_SIZE_X, CACTUS_SMALL_SIZE_Y,
             GROUND};

#[derive(Clone, Copy)]
pub enum CactusSize {
    SMALL,
    BIG,
}

impl CactusSize {
    pub fn from_int(t: i32) -> CactusSize {
        let types = [CactusSize::SMALL, CactusSize::BIG];
        types[t as usize]
    }
    /*pub fn to_int(t: CactusSize) -> i32 {
        match t {
            CactusSize::SMALL => 0,
            CactusSize::BIG => 1,
        }
    }*/
}

#[warn(dead_code)]
pub struct Cactus {
    aabb: AABB,
    size: CactusSize,
}

impl Cactus {
    pub fn new_group(position: i32) -> Vec<Cactus> {
        let mut distance = 0;
        let mut cactuses: Vec<Cactus> = Vec::new();

        // 25% -- one cactus, 50% -- two cactus, 25% three cactus.
        let probability = [1, 2, 2, 3];

        for _ in 0..probability[rand::thread_rng().gen_range(0, 4) as usize] {
            let mut cactus: Cactus;
            let size = CactusSize::from_int(rand::thread_rng().gen_range(0, 2));
            let start_point_x = position + distance;
            match size {
                CactusSize::SMALL => {
                    cactus = Cactus::new(
                        CACTUS_SMALL_SIZE_X,
                        CACTUS_SMALL_SIZE_Y,
                        size,
                        start_point_x,
                    );
                    distance += CACTUS_SMALL_SIZE_X;
                }
                CactusSize::BIG => {
                    cactus = Cactus::new(CACTUS_BIG_SIZE_X, CACTUS_BIG_SIZE_Y, size, start_point_x);
                    distance += CACTUS_BIG_SIZE_X;
                }
            }
            cactuses.push(cactus);
        }
        cactuses
    }
    pub fn new(cx: i32, cy: i32, size: CactusSize, sp: i32) -> Cactus {
        Cactus {
            aabb: AABB::new(
                Coord::new(sp as f64, (GROUND - cy) as f64),
                Coord::new((sp + cx) as f64, GROUND as f64),
            ),
            size: size,
        }
    }
    pub fn get_max(&mut self) -> Coord {
        self.aabb.max
    }
    #[warn(dead_code)]
    pub fn get_position(&self) -> Coord {
        self.aabb.min
    }
    #[warn(dead_code)]
    pub fn get_sprite(&self) -> Sprite {
        let mut sprite: Vec<char>;
        let mut size: Coord;
        match self.size {
            CactusSize::SMALL => {
                sprite = vec![];
                size = Coord::new(CACTUS_SMALL_SIZE_X as f64, CACTUS_SMALL_SIZE_Y as f64);
            }
            CactusSize::BIG => {
                sprite = vec![];
                size = Coord::new(CACTUS_BIG_SIZE_X as f64, CACTUS_BIG_SIZE_Y as f64);
            }
        }
        Sprite::new(sprite, size)
    }
}
