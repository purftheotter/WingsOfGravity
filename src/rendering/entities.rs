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
        render_context.assets.get_texture(
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

    if let Some(texture) = render_context.assets.get_texture(&ship_component.sprite.texture_id) {
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

pub fn render_asteroid(
    entity: &mut Entity,
    render_context:&mut RenderContext,
) -> Result<(), String> {
   

    let Some(asteroid) =
        entity.asteroid.as_mut() else { return Ok(()) };
    let Some(rock) =
        render_context.assets.get_pixels("asteroid") 
            else { return Ok(()) };

    let texture_creator = &render_context.canvas.texture_creator();

    let centre = render_context.world_to_screen(entity.position.translation);

    let screen_size = render_context.world_to_screen_scale(
        asteroid.mask.size as f32 * render_context.meters_per_texel,
    );
    let angle = render_context
        .world_to_screen_rot(entity.position.rotation.angle())
        .to_degrees() as f64;


    let key = ((entity.id.slot as u64) << 32) | entity.id.generation as u64;

    let tex = render_context
        .asteroid_textures
        .get(texture_creator, key, &mut asteroid.mask, rock)?;

    render_context.canvas.copy_ex_f(
        tex,
        None,
        FRect::from_center(FPoint::new(centre.x, centre.y), screen_size, screen_size),
        angle,
        FPoint::new(screen_size / 2.0, screen_size / 2.0),
        false,
        false,
    )

}
