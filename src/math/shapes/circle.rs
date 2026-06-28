use crate::components::transform::Transform;
use crate::math::vec2::{Vec2, project_points};


#[derive(Clone, serde::Deserialize)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        Self {radius:r}
    }
}

pub fn project_circle(
    circle: &Circle,
    transform: &Transform,
    axis: Vec2
    ) -> (f32, f32) {
    let direction = axis.normalize();
    let direction_radius = direction * circle.radius;

    let p1 = transform.position + direction_radius;
    let p2 = transform.position - direction_radius;

    project_points(&vec![p1,p2], axis)
} 
