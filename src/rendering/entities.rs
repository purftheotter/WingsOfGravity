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

        let world_width = texture.query().width as f32 * ship_stats.scale;
        let world_hight = texture.query().width as f32 * ship_stats.scale;

        let screen_width = render_context.world_to_screen_scale(world_width);
        let screen_hight = render_context.world_to_screen_scale(world_hight);

        render_context.canvas.copy_ex_f(
            texture,
            Rect::new(0, 0, texture.query().width, texture.query().height),
            FRect::from_center(
                FPoint::new(
                    entity_transform.x,
                    entity_transform.y),
                screen_width,
                screen_hight,
                ),
            entity.position.rotation.angle().to_degrees() as f64,
            FPoint::new(
                screen_width / 2.0,
                screen_hight / 2.0,
        ),
            false,
            false,
        )?;
    }

    Ok(())
}
