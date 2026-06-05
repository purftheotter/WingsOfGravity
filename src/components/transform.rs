use crate::Vec2;

#[derive(Debug)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}
