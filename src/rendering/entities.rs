use sdl2::rect::{Rect,FPoint,FRect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::components::asteroid::AsteroidChunk;
use crate::components::transform::Transform;
use crate::math::vec2::Vec2;
use crate::rendering::assets::Assets;
use crate::entity::Entity;
use crate::assets::ship_database::ShipDatabase;
use crate::rendering::primitives::render_polygon;


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


    if let Some(children) = &chunk.children {

        let (min_x_size, max_x_size) = chunk
            .shape
            .apply_transformation(transform)
            .project_polygon(
                Vec2::new(
                    1.0,
                    0.0
                )
            );

        let (min_y_size, max_y_size) = chunk
            .shape
            .apply_transformation(transform)
            .project_polygon(
                Vec2::new(
                    0.0,
                    1.0
                )
            );

        let middle = Vec2::new(
            min_x_size + (min_x_size - max_x_size) / 2.0,
            min_y_size + (min_y_size - max_y_size) / 2.0);

        let top_left = &children[0];

        let mut top_left_transform: Transform = *transform;
        top_left_transform.position = middle / 2.0;

        let top_right = &children[1];

        let mut top_right_transform: Transform = *transform;
        top_right_transform.position = middle * Vec2::new(0.75, 0.25);

        let bottom_left = &children[2];

        let mut bottom_left_transform: Transform = *transform;
        bottom_left_transform.position = middle * Vec2::new(0.25, 0.75);

        let bottom_right = &children[3];

        let mut bottom_right_transform: Transform = *transform;
        bottom_right_transform.position = middle * 0.75;

        render_chunk(canvas, assets,top_left, &top_left_transform);
        render_chunk(canvas, assets,top_right, &top_right_transform);
        render_chunk(canvas, assets, bottom_left, &bottom_left_transform);
        render_chunk(canvas, assets, bottom_right, &bottom_right_transform);

    }else {
        render_polygon(canvas, transform, &chunk.shape)?;
        
    }

    Ok(())

}
