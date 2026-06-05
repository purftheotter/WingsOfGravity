use crate::math::shapes::{Circle,Polygon};
use crate::math::vec2math::Vec2;

pub enum Hitbox {
    Circle { radius: f32 },
    Polygon { points: Vec<Vec2> },
}
