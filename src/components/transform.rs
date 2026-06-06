use crate::math::vec2math::Vec2;

#[derive(Debug)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}
