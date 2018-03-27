use vec2::Vec2;

#[derive(Clone, Copy)]
pub struct AABB {
    min: Vec2,
    max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> AABB {
        AABB { min: min, max: max }
    }
    pub fn shift(&mut self, vector: Vec2) {
        self.min += vector;
        self.max += vector;
    }
    pub fn min(&self) -> Vec2 {
        self.min
    }
    pub fn max(&self) -> Vec2 {
        self.max
    }
    pub fn vs_aabb(&self, other: AABB) -> bool {
        if self.max().x < other.min().x || self.min().x > other.max().x {
            return false;
        }
        if self.max().y < other.min().y || self.min().y > other.max().y {
            return false;
        }
        true
    }
}
