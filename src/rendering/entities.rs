use sdl2::rect::*;

use crate::rendering::*;

use crate::entity::*;
use crate::assets::ShipDatabase;

use crate::render_context::RenderContext;

pub fn render_ship(
    entity: &Entity,
    render_context:&mut RenderContext,
    assets: &Assets,
    ship_db: &ShipDatabase
    ) -> Result<(), String> {
    let ship_stats = 
        ship_db.get(entity.ship_component.as_ref().unwrap().class);

    let entity_transform = 
        render_context.world_to_screen(entity.position.translation);

    if let Some(texture) = assets.get(&ship_stats.texture_id) {
        render_context.canvas.copy_ex_f(
            texture,
            Rect::new(0, 0, texture.query().width, texture.query().height),
            FRect::from_center(
                FPoint::new(
                    entity_transform.x,
                    entity_transform.y),
                texture.query().width as f32,
                texture.query().height as f32,
                ),
            entity.position.rotation.angle().to_degrees() as f64,
            FPoint::new(
                (texture.query().width as f32) / 2.0,
                (texture.query().height as f32) / 2.0,
        ),
            false,
            false,
        )?;
    }

    Ok(())
}
