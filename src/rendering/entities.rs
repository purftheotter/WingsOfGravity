use sdl2::rect::*;

use crate::entity::*;
use crate::assets::ShipDatabase;

use crate::render_context::RenderContext;

pub fn render_projectile(
    entity: &Entity,
    render_context:&mut RenderContext,
    ) -> Result<(), String> {

    let entity_transform = 
        render_context.world_to_screen(entity.position.translation);

    if let Some(texture) = 
        render_context.assets.get(
            entity.projectile.as_ref().unwrap()
                .sprite.texture_id.as_str()
        ) {
    
        let world_width = 1.0;
        let world_height = 1.0;

        let screen_width = 
            render_context.world_to_screen_scale(world_width);
        let screen_height = 
            render_context.world_to_screen_scale(world_height);

        render_context.canvas.copy_ex_f(
            texture,
            Rect::new(0, 0, texture.query().width, texture.query().height),
            FRect::from_center(
                FPoint::new(
                    entity_transform.x,
                    entity_transform.y),
                screen_width,
                screen_height,
                ),
                render_context.world_to_screen_rot(
                    entity.position.rotation.angle()).to_degrees() as f64,
            FPoint::new(
                screen_width / 2.0,
                screen_height / 2.0,
            ),
            false,
            false,
        )?;

    }

    Ok(())

}

pub fn render_ship(
    entity: &Entity,
    render_context:&mut RenderContext,
    ship_db: &ShipDatabase
    ) -> Result<(), String> {
    let ship_component = entity.ship_component.as_ref().unwrap();
    let ship_stats = 
        ship_db.get(ship_component.class);

    let entity_transform = 
        render_context.world_to_screen(entity.position.translation);

    if let Some(texture) = render_context.assets.get(&ship_component.sprite.texture_id) {
        let world_width = ship_stats.size.x;
        let world_height = ship_stats.size.y;

        let screen_width = 
            render_context.world_to_screen_scale(world_width);
        let screen_height = 
            render_context.world_to_screen_scale(world_height);

render_context.canvas.copy_ex_f(
            texture,
            Rect::new(0, 0, texture.query().width, texture.query().height),
            FRect::from_center(
                FPoint::new(
                    entity_transform.x,
                    entity_transform.y),
                screen_width,
                screen_height,
                ),
                render_context.world_to_screen_rot(
                    entity.position.rotation.angle()).to_degrees() as f64,
            FPoint::new(
                screen_width / 2.0,
                screen_height / 2.0,
        ),
            false,
            false,
        )?;
    }

    Ok(())
}
