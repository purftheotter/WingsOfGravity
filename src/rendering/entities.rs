use std::thread::yield_now;

use sdl2::rect::{Rect,FPoint,FRect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::components::asteroid::AsteroidChunk;
use crate::components::transform::Transform;
use crate::math::shapes::Circle;
use crate::math::vec2::Vec2;
use crate::rendering::assets::Assets;
use crate::entity::Entity;
use crate::assets::ship_database::ShipDatabase;
use crate::rendering::primitives::{render_circle, render_polygon};


pub fn render_ship(
    entity: &Entity,
    canvas: &mut Canvas<Window>,
    assets: &Assets,
    ship_db: &ShipDatabase
    ) -> Result<(), String> {
    let ship_stats = ship_db.get(entity.ship_component.as_ref().unwrap().class);

    if let Some(texture) = assets.get(&ship_stats.texture_id) {
        canvas.copy_ex_f(
            texture,
            Rect::new(0, 0, texture.query().width, texture.query().height),
            FRect::from_center(
                FPoint::new(entity.transform.position.x, entity.transform.position.y),
                texture.query().width as f32 * entity.transform.scale.x,
                texture.query().height as f32 * entity.transform.scale.y,
                ),
            entity.transform.rotation as f64,
            FPoint::new(
                (texture.query().width as f32 * entity.transform.scale.x) / 2.0,
                (texture.query().height as f32 * entity.transform.scale.y) / 2.0,
        ),
            false,
            false,
        )?;
    }

    Ok(())
}

pub fn render_asteriod(
    entity: &Entity,
    canvas: &mut Canvas<Window>,
    assets: &Assets,
    ) -> Result<(), String> {
    render_chunk(canvas, assets, &entity.asteroid.as_ref().unwrap().root, &entity.transform)
}

pub fn render_chunk(
    canvas: &mut Canvas<Window>,
    assets: &Assets,
    chunk: &AsteroidChunk,
    transform: &Transform,

    ) -> Result<(),String>{

    let mut world_transform = *transform;
    world_transform.position += chunk.offset;

    if let Some(children) = &chunk.children {
        for child in children {
            render_chunk(canvas, assets, child, &world_transform)?;
        }
    }else {
        canvas.set_draw_color(Color::RGB(100, 0, 100));

        render_circle(canvas, &world_transform, &Circle::new(5.0))?;
    }


    Ok(())

}
