use crate::components::transform::Transform;
use crate::math::shapes::Circle;
use crate::math::shapes::Polygon;
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
        )?;
    }

    Ok(())
}

pub fn render_filled_polygon(
    canvas: &mut Canvas<Window>,
    transform: &Transform,
    polygon: &Polygon,
) -> Result<(), String> {
    let ws_polygon = polygon.apply_transformation(transform);
    let mut points = vec![];
    for point in ws_polygon.points.iter() {
        points.push(FPoint::new(point.x, point.y));
    }

    if points.len() < 3 {
        return Ok(());
    }

    // Find bounding box
    let min_y = points.iter().map(|p| p.y()).fold(f32::MAX, f32::min) as i32;
    let max_y = points.iter().map(|p| p.y()).fold(f32::MIN, f32::max) as i32;

    // Scanline fill
    for y in min_y..=max_y {
        let y_f = y as f32;
        let mut intersections: Vec<f32> = Vec::new();
        let n = points.len();

        for i in 0..n {
            let a = points[i];
            let b = points[(i + 1) % n];
            let ay = a.y();
            let by = b.y();

            // Check if scanline crosses this edge
            if (ay <= y_f && by > y_f) || (by <= y_f && ay > y_f) {
                let t = (y_f - ay) / (by - ay);
                intersections.push(a.x() + t * (b.x() - a.x()));
            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                canvas.draw_fline(
                    FPoint::new(pair[0], y_f),
                    FPoint::new(pair[1], y_f),
                )?;
            }
        }
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

