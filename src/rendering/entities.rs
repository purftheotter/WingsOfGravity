use sdl2::rect::{Rect,FPoint,FRect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::rendering::assets::Assets;
use crate::entity::Entity;
use crate::assets::ship_database::ShipDatabase;


pub fn render_ship(entity: &Entity, canvas: &mut Canvas<Window>, assets: &Assets,ship_db: &ShipDatabase) -> Result<(), String> {
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
