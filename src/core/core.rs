use std::usize;
use rapier2d::math::*;

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
