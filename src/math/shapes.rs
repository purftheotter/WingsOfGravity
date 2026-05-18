use crate::math::vec2math::Vec2;

#[derive(Debug)]

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        Self { radius: r }
    }
}

pub struct Polygon {
    pub points: Vec<Vec2>,
    pub center: Vec2,
}

impl Polygon {
    pub fn new(points: Vec<Vec2>, center: Vec2) -> Self {
        Self { points, center }
    }
}
