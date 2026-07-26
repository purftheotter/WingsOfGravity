use rapier2d::geometry::*;
use rapier2d::math::*;
use rapier2d::parry::transformation::utils::transformed;
use crate::render_context;
use crate::render_context::RenderContext;
use sdl2::rect::FPoint;

pub fn render_ws_line(
    render_context: &mut RenderContext,
    start: &Vec2,
    end: &Vec2,
) -> Result<(), String> {
    let start = render_context.world_to_screen(*start);
    let end = render_context.world_to_screen(*end);

    let start = FPoint::new(start.x, start.y);
    let end = FPoint::new(end.x, end.y);

    render_context.canvas.draw_fline(start, end)
}

pub fn render_triangle(
    render_context: &mut RenderContext,
    position: &Pose2,
    triangle: &Triangle,
) -> Result<(), String> {

    let transformed_triangle = triangle.transformed(position);

    let a_vec2 = 
        render_context.world_to_screen(transformed_triangle.a);
    let b_vec2 =
        render_context.world_to_screen(transformed_triangle.b);
    let c_vec2 = 
        render_context.world_to_screen(transformed_triangle.c);

    let a_fpoint = FPoint::new(
        a_vec2.x,
        a_vec2.y
        );
    let b_fpoint = FPoint::new(
        b_vec2.x,
        b_vec2.y
        );
    let c_fpoint = FPoint::new(
        c_vec2.x,
        c_vec2.y
        );

    render_context.canvas.draw_fline(a_fpoint, b_fpoint)?;
    render_context.canvas.draw_fline(b_fpoint, c_fpoint)?;
    render_context.canvas.draw_fline(c_fpoint, a_fpoint)?;

    println!("{:?}", position);

    println!("a: {:?}, b: {:?}, c: {:?}", a_fpoint, b_fpoint, c_fpoint);

    Ok(())
}


pub fn render_convex_polygon(
    render_context: &mut RenderContext,
    position: &Pose2,
    polygon: &ConvexPolygon,
) -> Result<(), String> {

    let mut transformed_points: Vec<Vec2> = vec![];
    
    for point in polygon.points() {
        transformed_points.push(*position * *point);
    }

    let mut ws_points: Vec<Vec2> = vec![];

    for point in transformed_points.iter() {
        ws_points.push(render_context.world_to_screen(*point));
    }

    for i in 0..ws_points.len() {

        let current = &ws_points[i];

        let next =
            &ws_points[
                (i + 1) % transformed_points.len()
            ];

        render_context.canvas.draw_fline(
            FPoint::new(current.x, current.y),
            FPoint::new(next.x, next.y),
        )?;
    }

    Ok(())
}

pub fn render_filled_polygon(
    render_context: &mut RenderContext,
    position: &Pose2,
    polygon: &ConvexPolygon,
) -> Result<(), String> {

    let mut transformed_points: Vec<Vec2>= vec![];
    for point in polygon.points() {
        transformed_points.push(*position * *point);
    }

    let mut ws_points = vec![];
    for point in transformed_points {
        ws_points.push(render_context.world_to_screen(point));
    }

    let mut fpoints = vec![];
    for point in ws_points.iter() {
        fpoints.push(FPoint::new(point.x, point.y));
    }

    if fpoints.len() < 3 {
        return Ok(());
    }

    // Find bounding box
    let min_y = fpoints.iter().map(|p| p.y()).fold(f32::MAX, f32::min) as i32;
    let max_y = fpoints.iter().map(|p| p.y()).fold(f32::MIN, f32::max) as i32;

    // Scanline fill
    for y in min_y..=max_y {
        let y_f = y as f32;
        let mut intersections: Vec<f32> = Vec::new();
        let n = fpoints.len();

        for i in 0..n {
            let a = fpoints[i];
            let b = fpoints[(i + 1) % n];
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
                render_context.canvas.draw_fline(
                    FPoint::new(pair[0], y_f),
                    FPoint::new(pair[1], y_f),
                )?;
            }
        }
    }

    Ok(())
}

pub fn render_ball(
    render_context: &mut RenderContext,
    pos: &Pose2,
    ball: &Ball,
) -> Result<(), String> {

    let segments = 32;

    for i in 0..segments {
        let theta1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let theta2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

        let world1 = Vec2::new(
            pos.translation.x + ball.radius * theta1.cos(),
            pos.translation.y + ball.radius * theta1.sin(),
        );
        let world2 = Vec2::new(
            pos.translation.x + ball.radius * theta2.cos(),
            pos.translation.y + ball.radius * theta2.sin(),
        );

        let screen1 = render_context.world_to_screen(world1);
        let screen2 = render_context.world_to_screen(world2);

        render_context.canvas.draw_fline(
            FPoint::new(screen1.x, screen1.y),
            FPoint::new(screen2.x, screen2.y),
        ).expect("render_ball failed");
    }

    Ok(())
}

pub fn render_compound(
    render_context: &mut RenderContext,
    position: &Pose2,
    compound: &Compound
) -> Result<(), String> {
    for shape in compound.shapes() {
        let shape_pos = &(position * &shape.0);
        match shape.1.as_typed_shape() {
            TypedShape::Ball(ball) => {
                render_ball(
                    render_context,
                    shape_pos,
                    ball
                )?
            }

            TypedShape::Triangle(triangle) => {
                render_triangle(
                    render_context,
                    shape_pos,
                    triangle
                )?
            }

            TypedShape::ConvexPolygon(polygon) => {
                render_convex_polygon(
                    render_context,
                    shape_pos,
                    polygon
                )?
            }

            TypedShape::Compound(compound) => {
                render_compound(
                    render_context,
                    shape_pos, 
                    compound
                )?
            }
            
            _ => {}
        }
    }
    Ok(())
}
