use crate::components::primitives::Vec2;

pub fn subtract(v1: &Vec2, v2: &Vec2) -> Vec2 {
    Vec2 {
        x: (v1.x - v2.x),
        y: (v1.y - v2.y),
    }
}

pub fn normal(v1: &Vec2, v2: &Vec2) -> Vec2 {
    Vec2 {
        x: (-(subtract(v1, v2).y)),
        y: (subtract(v1, v2).x),
    }
}
