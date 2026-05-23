use crate::math::shapes::{Circle, Polygon};
use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3::render::FPoint;

pub fn render_polygon(
    canvas: &mut Canvas<Window>,
    polygon: &Polygon,
) -> Result<(), String> {

    for i in 0..polygon.points.len() {

        let current = &polygon.points[i];

        let next =
            &polygon.points[
                (i + 1) % polygon.points.len()
            ];

        canvas.draw_line(
            FPoint::new(current.x, current.y),
            FPoint::new(next.x, next.y),
        ).expect("render_polygon failed");
    }

    Ok(())
}

pub fn render_circle(
    canvas: &mut Canvas<Window>,
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
            circle.center.x
            + circle.radius * theta1.cos();

        let y1 =
            circle.center.y
            + circle.radius * theta1.sin();

        let x2 =
            circle.center.x
            + circle.radius * theta2.cos();

        let y2 =
            circle.center.y
            + circle.radius * theta2.sin();

        canvas.draw_line(
            FPoint::new(x1, y1),
            FPoint::new(x2, y2),
        ).expect("render_circle failed");
    }

    Ok(())
}
