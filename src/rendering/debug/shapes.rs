use rapier2d::geometry::*;
use rapier2d::math::*;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

use crate::render_context::RenderContext;
use crate::core::*;


pub fn render_ws_line(render_context: &mut RenderContext, start: &Vec2, end: &Vec2, color: Color) -> Result<(), String> {
    let (x1, y1) = render_context.world_to_screen_i16(*start);
    let (x2, y2) = render_context.world_to_screen_i16(*end);
    render_context.canvas.line(x1, y1, x2, y2, color)
}

pub fn render_triangle(render_context: &mut RenderContext, pose: &Pose2, tri: &Triangle, color: Color) -> Result<(), String> {
    let t = tri.transformed(pose);
    let (x1, y1) = render_context.world_to_screen_i16(t.a);
    let (x2, y2) = render_context.world_to_screen_i16(t.b);
    let (x3, y3) = render_context.world_to_screen_i16(t.c);
    render_context.canvas.trigon(x1, y1, x2, y2, x3, y3, color)
}

pub fn render_convex_polygon(render_context: &mut RenderContext, pose: &Pose2, polygon: &ConvexPolygon, color: Color) -> Result<(), String> {
    let (vx, vy) = screen_poly(render_context, pose, polygon.points());
    render_context.canvas.polygon(&vx, &vy, color)
}

pub fn render_filled_polygon(render_context: &mut RenderContext, pose: &Pose2, polygon: &ConvexPolygon, color: Color) -> Result<(), String> {
    let (vx, vy) = screen_poly(render_context, pose, polygon.points());
    render_context.canvas.filled_polygon(&vx, &vy, color)
}

pub fn render_ball(render_context: &mut RenderContext, pose: &Pose2, ball: &Ball, color: Color) -> Result<(), String> {
    let (x, y) = render_context.world_to_screen_i16(pose.translation);
    let r = to_i16(render_context.world_to_screen_scale(ball.radius));
    render_context.canvas.circle(x, y, r, color)
}

pub fn render_compound(
    render_context: &mut RenderContext,
    position: &Pose2,
    compound: &Compound,
    color: Color,
) -> Result<(), String> {
    for shape in compound.shapes() {
        let shape_pos = &(position * &shape.0);
        match shape.1.as_typed_shape() {
            TypedShape::Ball(ball) => {
                render_ball(
                    render_context,
                    shape_pos,
                    ball,
                    color,
                )?
            }

            TypedShape::Triangle(triangle) => {
                render_triangle(
                    render_context,
                    shape_pos,
                    triangle,
                    color,
                )?
            }

            TypedShape::ConvexPolygon(polygon) => {
                render_convex_polygon(
                    render_context,
                    shape_pos,
                    polygon,
                    color,
                )?
            }

            TypedShape::Compound(compound) => {
                render_compound(
                    render_context,
                    shape_pos, 
                    compound,
                    color,
                )?
            }
            
            _ => {}
        }
    }
    Ok(())
}

pub fn screen_poly(render_context: &RenderContext, pose: &Pose2, points: &[Vec2]) -> (Vec<i16>, Vec<i16>) {
    points.iter()
        .map(|p| render_context.world_to_screen_i16(*pose * *p))
        .unzip()
}
