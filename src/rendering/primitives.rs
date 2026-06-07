use crate::components::transform::Transform;
use crate::math::shapes::{Circle, Polygon};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::FPoint;

pub fn render_polygon(
    canvas: &mut Canvas<Window>,
    transform: &Transform,
    polygon: &Polygon,
) -> Result<(), String> {
    let transformed_polygon = polygon.apply_transformation(transform);

    for i in 0..transformed_polygon.points.len() {

        let current = &transformed_polygon.points[i];

        let next =
            &transformed_polygon.points[
                (i + 1) % transformed_polygon.points.len()
            ];

        canvas.draw_fline(
            FPoint::new(current.x, current.y),
            FPoint::new(next.x, next.y),
        ).expect("render_polygon failed");
    }

    Ok(())
}

pub fn render_circle(
    canvas: &mut Canvas<Window>,
    transform: &Transform,
    circle: &Circle,
) -> Result<(), String> {

    let segments = 32;

    for i in 0..segments {

        let theta1 =
            (i as f32 / segments as f32)
            * std::f32::consts::TAU;

        let theta2 =
            ((i + 1) as f32 / segments as f32)
            * std::f32::consts::TAU;

        let x1 =
            transform.position.x
            + circle.radius * theta1.cos();

        let y1 =
            transform.position.y
            + circle.radius * theta1.sin();

        let x2 =
            transform.position.x
            + circle.radius * theta2.cos();

        let y2 =
            transform.position.y
            + circle.radius * theta2.sin();

        canvas.draw_fline(
            FPoint::new(x1, y1),
            FPoint::new(x2, y2),
        ).expect("render_circle failed");
    }

    Ok(())
}
