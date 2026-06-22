use crate::math::vec2::Vec2;

#[derive(Debug,Clone,Copy)]
pub struct Velocity {
    pub linear :Vec2,
    pub angular:f32,
}
