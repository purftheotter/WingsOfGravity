use crate::components::transform::Transform;
use crate::math::shapes::{Circle, Polygon};
use crate::math::vec2::{Vec2, project_points};

pub fn sat_collision(
    t1: &Transform,
    pg1: &Polygon,
    t2: &Transform,
    pg2: &Polygon,
    ) -> (bool, Option<Vec2>, Option<f32>) {

    let wspg1 = pg1.apply_transformation(t1);
    let wspg2 = pg2.apply_transformation(t2);

    let mut normal = Vec2::new(0.0, 0.0);
    let mut depth = f32::MAX;

    // Polygon 1 axes
    for i in 0..wspg1.points.len() {
        let va = wspg1.points[i];
        let vb = wspg1.points[(i + 1) % wspg1.points.len()];

        let axis = (va - vb).normal().normalize();

        let (mina, maxa) = project_points(&wspg1.points, axis);
        let (minb, maxb) = project_points(&wspg2.points, axis);

        if maxa < minb || maxb < mina {
            return (false, None, None);
        }

        let axis_depth = f32::min(maxb - mina,maxa - minb);

        if axis_depth < depth {
            depth = axis_depth;

            let mut final_axis = axis;
            if final_axis.dot(t2.position - t1.position) < 0.0 {
                final_axis = -final_axis;
            }

            normal = final_axis;
        }
    }

    //Polygon 2 axes
    for i in 0..wspg2.points.len() {
        let va = wspg2.points[i];
        let vb = wspg2.points[(i + 1) % pg2.points.len()];

        let axis = (vb-va).normal().normalize();

        let (mina, maxa) = project_points(&wspg1.points, axis);
        let (minb, maxb) = project_points(&wspg2.points, axis);

        if maxa < minb || maxb < mina {
            return (false, None, None);
        }

        let axis_depth = f32::min(maxb - mina,maxa - minb);

        if axis_depth < depth {

            depth = axis_depth;
            
            let mut final_axis = axis;
            if final_axis.dot(t2.position - t1.position) < 0.0 {
                final_axis = -final_axis;
            }

            normal = final_axis;
        }
    }

    return (true, Some(normal), Some(depth))
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
