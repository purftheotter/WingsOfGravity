use std::usize;
use rapier2d::math::*;
use sdl2::pixels::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolarIndex {
    pub ring: usize,
    pub point: usize,
}

impl PolarIndex {
    pub fn new(ring: usize, point:usize) -> Self {
        Self {
            ring,
            point
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Slot {
    pub generation: u32,
    pub index: Option<usize>,
}

pub fn rotate_pos_around(pose: Pose2, pivot: Vec2, rot: Rot2) -> Pose2 {
    Pose2::from_parts(
        pivot + rot * (pose.translation - pivot),
        rot * pose.rotation,
    )
}

pub fn rotate_vec_around(point: Vec2, pivot: Vec2, rot: Rot2) -> Vec2 {
    pivot + rot * (point - pivot)
}

pub fn point_in_triangle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let edge = |u: Vec2, v: Vec2, p: Vec2| (v.x - u.x) * (p.y - u.y) - (v.y - u.y) * (p.x - u.x);
    let w0 = edge(a, b, p);
    let w1 = edge(b, c, p);
    let w2 = edge(c, a, p);
    (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0)
}

pub fn to_i16(v: f32) -> i16 {
    v.round() as i16
}

pub const fn to_gfx(color: Color) -> Color {
    Color::RGBA(color.a, color.b, color.g, color.r)
}
