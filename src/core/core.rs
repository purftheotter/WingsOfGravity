use std::usize;

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
