use crate::math::shapes::{Circle, Polygon};

enum Collider {
    Circle(Circle),
    Polygon(Polygon),
}
