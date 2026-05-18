#[derive(Debug)]

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Quad {
    pub points: [Vec2; 4],
}

impl Quad {
    pub fn new_exact(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) -> Self {
        Self {
            points: [p1, p2, p3, p4],
        }
    }
}
