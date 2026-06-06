use crate::components::transform::Transform;
use crate::math::shapes::{Circle, Polygon};
use crate::math::vec2math::project_polygon;

pub fn sat_collision(pg1: &Polygon, pg2: &Polygon) -> bool {

    for i in 0..pg1.points.len() {
        let va = pg1.points[i];
        let vb = pg1.points[(i + 1) % pg1.points.len()];

        let edge = va - vb;
        let axis = edge.normal();
        let (mina, maxa) = project_polygon(&pg1.points, axis);
        let (minb, maxb) = project_polygon(&pg2.points, axis);

        if maxa < minb || maxb < mina {
            return false;
        }
    }

    for i in 0..pg2.points.len() {
        let va = pg2.points[i];
        let vb = pg2.points[(i + 1) % pg2.points.len()];

        let edge = va - vb;
        let axis = edge.normal();
        let (mina, maxa) = project_polygon(&pg1.points, axis);
        let (minb, maxb) = project_polygon(&pg2.points, axis);

        if maxa < minb || maxb < mina {
            return false;
        }
    }

    return true;
}

pub fn two_circle_collision(c1: &Circle,t1: &Transform, c2: &Circle, t2: &Transform) -> bool {
    let dx = t1.position.x - t2.position.x;
    let dy = t1.position.y - t2.position.y;

    let distance_sqaured = dx * dx + dy * dy;

    let radius_sum = c1.radius + c2.radius; 
    
    if distance_sqaured <= radius_sum * radius_sum{
        true
    }else {
        false
    }
}
