use crate::math::shapes::Polygon;
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
