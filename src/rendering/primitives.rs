use std::ffi::c_float;

use rapier2d::geometry::*;
use rapier2d::math::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::FPoint;

pub fn render_ws_line(
    canvas: &mut Canvas<Window>,
    start: &Vec2,
    end: &Vec2,
) -> Result<(), String> {
    let start = FPoint::new(start.x, start.y);
    let end = FPoint::new(end.x, end.y);
    canvas.draw_fline(start, end)
}


pub fn render_triangle(
    canvas: &mut Canvas<Window>,
    position: &Pose2,
    triangle: &Triangle,
) -> Result<(), String> {

    let transformed_triangle = triangle.transformed(position);

    let a_fpoint = FPoint::new(
        transformed_triangle.a.x,
        transformed_triangle.a.x
        );
    let b_fpoint = FPoint::new(
        transformed_triangle.b.x,
        transformed_triangle.b.y
        );
    let c_fpoint = FPoint::new(
        transformed_triangle.c.x,
        transformed_triangle.c.y
        );

    canvas.draw_fline(a_fpoint, b_fpoint)?;
    canvas.draw_fline(b_fpoint, c_fpoint)?;
    canvas.draw_fline(c_fpoint, a_fpoint)?;

    Ok(())
}


pub fn render_polygon(
    canvas: &mut Canvas<Window>,
    position: &Pose2,
    polygon: &ConvexPolygon,
) -> Result<(), String> {

    let mut transformed_polygon: Vec<Vec2>= vec![];
    for point in polygon.points() {
        transformed_polygon.push(*position * *point);
    }

    for i in 0..transformed_polygon.len() {

        let current = &transformed_polygon[i];

        let next =
            &transformed_polygon[
                (i + 1) % transformed_polygon.len()
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
    position: &Pose2,
    polygon: &ConvexPolygon,
) -> Result<(), String> {

    let mut polygon_points: Vec<Vec2>= vec![];
    for point in polygon.points() {
        polygon_points.push(*position * *point);
    }

    let mut points = vec![];
    for point in polygon_points.iter() {
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

pub fn render_ball(
    canvas: &mut Canvas<Window>,
    position: &Pose2,
    ball: &Ball,
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
            position.translation.x
            + ball.radius * theta1.cos();

        let y1 =
            position.translation.y
            + ball.radius * theta1.sin();

        let x2 =
            position.translation.x
            + ball.radius * theta2.cos();

        let y2 =
            position.translation.y
            + ball.radius * theta2.sin();

        canvas.draw_fline(
            FPoint::new(x1, y1),
            FPoint::new(x2, y2),
        ).expect("render_ball failed");
    }

    Ok(())
}

