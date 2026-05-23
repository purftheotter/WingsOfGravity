use crate::math::shapes::Circle;



pub fn two_circle_collision(c1: Circle, c2: Circle) -> bool {
    if c1.center.x - c2.center.x + c1.center.y-c2.center.y < (c1.radius +c2.radius) {
        true
    }else {
        false
    }
}
