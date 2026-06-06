use crate::{components::transform::Transform, math::vec2math::Vec2};

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        Self {radius:r}
    }
}

#[derive(Clone,serde::Deserialize)]
pub struct Polygon {
    pub points: Vec<Vec2>,
}

impl Polygon {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
    pub fn apply_transformation(
        &self,
        transform: &Transform,
    ) -> Polygon {
        let rotation = transform.rotation.to_radians();
        let (sin,cos) = rotation.sin_cos();

        let points = self.points
            .iter()
            .map(|p| {

                let x = p.x * transform.scale.x;
                let y = p.y * transform.scale.y;


                let rotated_x = x * cos - y * sin;
                let rotated_y = x * sin + y * cos;
                Vec2::new(
                    rotated_x,
                    rotated_y,
                ) + transform.position
            })
            .collect();

        Polygon::new(points)

    }
}
