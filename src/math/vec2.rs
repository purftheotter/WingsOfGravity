use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign,Neg};

use crate::components::transform::Transform;

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize)]

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl Mul<Vec2> for Vec2{
    type Output = Self;

    fn mul(self, rhs: Vec2) -> Self {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
    
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, s: f32) -> Self {
        Vec2::new(self.x / s, self.y / s)
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normal(&self) -> Vec2 {
        Vec2 {x: -self.y, y: self.x,}
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(self) -> f32 {
       (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            return Self::new(0.0, 0.0);
        }
        self / len
    }

}

pub fn dot_product(v1: &Vec2, v2: &Vec2) -> f32 {(v1.x * v2.x) + (v1.y * v2.x)}

pub fn project_points(points: &[Vec2], axis: Vec2) -> (f32, f32) {
    let mut min = points[0].dot(axis);
    let mut max = min;

    for p in points.iter().skip(1) {
        let proj = p.dot(axis);

        if proj < min {min = proj;}
        if proj > max {max = proj;}
    }

    (min,max)
}

fn transform_points(points: &[Vec2], t: &Transform) -> Vec<Vec2> {
    let (sin, cos) = t.rotation.sin_cos();

    points.iter().map(|p| {
        let rotated = Vec2::new(
            p.x * cos - p.y * sin,
            p.x * sin + p.y * cos,
        );

        rotated + t.position
    }).collect()
}

pub fn clip_x(
    p1: &Vec2,
    p2: &Vec2,
    boundary_x: f32,
) -> Option<Vec2> {
    let delta_x = p2.x - p1.x;

    if delta_x == 0.0 {
        return None;
    }

    let t = (boundary_x - p1.x) / delta_x;

    if !(0.0..=1.0).contains(&t) {
        return None;
    }

    let y = p1.y + t * (p2.y - p1.y);

    Some(Vec2::new(boundary_x, y))
}

pub fn clip_y(
    p1: &Vec2,
    p2: &Vec2,
    boundary_y: f32,
) -> Option<Vec2> {
    let delta_y = p2.y - p1.y;

    if delta_y == 0.0 {
        return None;
    }
    
    let t = (boundary_y - p1.y) / delta_y;
    
    if !(0.0..=1.0).contains(&t) {
        return None;
    }

    let x = p1.x + t * (p2.x - p1.x);

    Some(Vec2::new(x, boundary_y))
}


pub fn cross(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

pub fn perpendicular(v: Vec2) -> Vec2 {
    Vec2::new(-v.y, v.x)
}
