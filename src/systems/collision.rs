use crate::math::shapes::Circle;



pub fn two_circle_collision(c1: &Circle, c2: &Circle) -> bool {
    let dx = c1.center.x + c2.center.x;
    let dy = c1.center.y + c2.center.y;

    let distance_sqaured = dx * dx + dy * dy;

    let radius_sum = c1.radius + c2.radius; 
    
    if distance_sqaured < radius_sum * radius_sum{
        true
    }else {
        false
    }
}
