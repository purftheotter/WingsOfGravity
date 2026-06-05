use crate::math::shapes::{Circle, Polygon};
use crate::math::vec2math::project_polygon;

pub fn sap_collision(pg1: &Polygon, pg2: &Polygon) -> bool {

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

pub fn two_circle_collision(c1: &Circle, c2: &Circle) -> bool {
    let dx = c1.center.x - c2.center.x;
    let dy = c1.center.y - c2.center.y;

    let distance_sqaured = dx * dx + dy * dy;

    let radius_sum = c1.radius + c2.radius; 
    
    if distance_sqaured <= radius_sum * radius_sum{
        true
    }else {
        false
    }
}
