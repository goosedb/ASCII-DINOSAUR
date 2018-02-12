use coord::Coord;

pub struct AABB {
    pub min: Coord,
    pub max: Coord,
}

impl AABB {
    pub fn new(min: Coord, max: Coord) -> AABB {
        AABB { min: min, max: max }
    }
}