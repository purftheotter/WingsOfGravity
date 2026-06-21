use crate::math::shapes::{Circle, Polygon};

#[derive(Clone, serde::Deserialize)]
pub enum Hitbox {
    Circle { circle: Circle },
    Polygon {polygon: Polygon},
}
