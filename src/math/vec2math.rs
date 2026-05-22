#[derive(Clone, Copy)]

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn sum(vectors: &Vec<Vec2>) -> Vec2 {
    let mut result: Vec2 = Vec2::new(0.0, 0.0);

    for vec in vectors {
        result.x += vec.x;
        result.y += vec.y;
    }

    result
}

pub fn difference(vectors: &Vec<Vec2>) -> Vec2 {
    let mut result: Vec2 = Vec2::new(vectors.get(0).unwrap().x, vectors.get(0).unwrap().y);

    for vec in vectors {
        result.x -= vec.x;
        result.y -= vec.y;
    }

    result
}

pub fn normal(v1: &Vec2, v2: &Vec2) -> Vec2 {
    Vec2 {
        x: (-difference(&vec![*v1, *v2]).y),
        y: (difference(&vec![*v1, *v2]).x),
    }
}

pub fn dot_product(v1: &Vec2, v2: &Vec2) -> f32 {(v1.x * v2.x) + (v1.y * v2.x)}
