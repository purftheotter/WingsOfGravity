use crate::math::shapes::Polygon;

#[derive(Clone, serde::Deserialize)]
pub enum Hitbox {
    Circle { radius: f32 },
    Polygon {
        polygon: Polygon
    },
}
