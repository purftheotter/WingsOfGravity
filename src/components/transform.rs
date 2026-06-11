use crate::math::vec2::Vec2;

#[derive(Debug,Clone,Copy)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}
