use crate::math::vec2math::Vec2;

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        Self {radius:r}
    }
}

pub struct Polygon {
    pub points: Vec<Vec2>,
}

impl Polygon {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
}
