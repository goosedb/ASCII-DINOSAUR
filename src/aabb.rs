use coord::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AABB {
    pub min: Coord_f,
    pub max: Coord_f,
}

impl AABB {
    pub fn new(min: Coord_f, max: Coord_f) -> AABB {
        AABB { min: min, max: max }
    }
}
