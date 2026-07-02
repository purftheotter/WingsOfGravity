use core::f32;

use sdl2::rect::{Rect,FPoint,FRect};
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::{Color, PixelFormatEnum};

use crate::components::asteroid::Asteroid;
use crate::components::asteroid::AsteroidChunk;
use crate::components::projectile::ProjectileType;
use crate::components::sprite;
use crate::components::transform::Transform;
use crate::math::shapes::polygon::project_polygon;
use crate::rendering::assets::Assets;
use crate::entity::Entity;
use crate::assets::ship_database::ShipDatabase;
use crate::rendering::primitives::render_polygon;

use crate::math::vec2::Vec2;
use crate::math::vec2::project_points;

pub fn render_ship(
    entity: &Entity,
    canvas: &mut Canvas<Window>,
    assets: &Assets,
    ship_db: &ShipDatabase
    ) -> Result<(), String> {
    let ship_stats = 
        ship_db.get(entity.ship_component.as_ref().unwrap().class);

    if let Some(texture) = assets.get(&ship_stats.texture_id) {
        canvas.copy_ex_f(
            texture,
            Rect::new(0, 0, texture.query().width, texture.query().height),
            FRect::from_center(
                FPoint::new(
                    entity.transform.position.x,
                    entity.transform.position.y),
                texture.query().width as f32 * entity.transform.scale.x,
                texture.query().height as f32 * entity.transform.scale.y,
                ),
            entity.transform.rotation.to_degrees() as f64,
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

pub fn render_asteroid(
    entity: &Entity,
    canvas: &mut Canvas<Window>,
    assets: &Assets,
) -> Result<(),String>{
    let asteroid = entity.asteroid.as_ref().unwrap();

    let sprite_id = 
        asteroid.sprite_id.as_ref().unwrap();

    let sprite_texture = match assets.get(&sprite_id) {
        Some(t) => t,
        None => return Ok(()),
    };

    let (asteroid_max_w, asteroid_min_w) = 
        project_polygon(&asteroid.root.shape, Vec2::new(1.0, 0.0));

    let asteroid_w = (asteroid_max_w - asteroid_min_w).abs();

    let (asteroid_max_h, asteroid_min_h) = 
        project_polygon(&asteroid.root.shape, Vec2::new(0.0, 1.0));

    let asteroid_h = (asteroid_max_h - asteroid_min_h).abs();

    let dst = FRect::from_center(
        FPoint::new(entity.transform.position.x, entity.transform.position.y),
        asteroid_w * entity.transform.scale.x,
        asteroid_h * entity.transform.scale.y,
        );
    let angle = entity.transform.rotation.to_degrees() as f64;

    let center = FPoint::new(
        (asteroid_w * entity.transform.scale.x) / 2.0,
        (asteroid_h * entity.transform.scale.y) / 2.0,
    );

    canvas.copy_ex_f(sprite_texture, None, dst, angle, center, false, false)?;

    Ok(())
    

}

pub fn build_asteroid_mask(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    asteroid: &Asteroid,
) -> Result<Texture<'static>,String> {
    let tex_size = 64u32;

    let (min_x, max_x) = 
        project_points(
            &asteroid.root.shape.points,
            Vec2::new(1.0, 0.0)
        );
    let (min_y, max_y) = 
        project_points(
            &asteroid.root.shape.points,
            Vec2::new(0.0, 1.0)
        );
    let bounds_min = Vec2::new(min_x, min_y);
    let bounds_max = Vec2::new(max_x, max_y);

    let mut mask = texture_creator
        .create_texture_target(
            Some(PixelFormatEnum::RGBA8888),
            tex_size,
            tex_size)
        .map_err(|e| e.to_string())?;

    canvas.with_texture_canvas(&mut mask, |c| {
        c.set_draw_color(Color::RGBA(0, 0, 0, 0));
        c.clear();
        c.set_draw_color(Color::RGBA(255, 255, 255, 255));
        asteroid.get_mask_texture(
            c, &bounds_min, &bounds_max, tex_size
        )
    }).map_err(|e| e.to_string())?;

    let mask: Texture<'static> = unsafe {std::mem::transmute(mask)};

    Ok(mask)
}

pub fn build_asteroid_sprite(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    rock_texture: &mut Texture,
    mask_texture: &mut Texture,
    tex_size: u32,
) -> Result<Texture<'static>, String> {
    let mut sprite = texture_creator
        .create_texture_target(Some(PixelFormatEnum::RGBA8888), tex_size, tex_size)
        .map_err(|e| e.to_string())?;

    canvas.with_texture_canvas(&mut sprite, |c| {
        c.set_draw_color(Color::RGBA(0, 0, 0, 0));
        c.clear();

        mask_texture.set_blend_mode(BlendMode::None);
        let _ = c.copy(mask_texture, None, None);


        rock_texture.set_blend_mode(BlendMode::Mod);
        let _ = c.copy(rock_texture, None, None);

    }).map_err(|e| e.to_string())?;

    sprite.set_blend_mode(BlendMode::Blend);

    let sprite: Texture<'static> = 
        unsafe { std::mem::transmute(sprite) };

    Ok(sprite)
}

pub fn debug_render_asteriod(
    entity: &Entity,
    canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
    debug_render_chunk(canvas, &entity.asteroid.as_ref().unwrap().root, &entity.transform)
}

pub fn debug_render_chunk(
    canvas: &mut Canvas<Window>,
    chunk: &AsteroidChunk,
    transform: &Transform,

    ) -> Result<(),String>{
    if let Some(children) = &chunk.children {
        for child in children {
            debug_render_chunk(canvas, child, &transform)?;
        }
        
        render_polygon(canvas, &transform, &chunk.shape)?;

    }else {
        canvas.set_draw_color(Color::RGB(100, 0, 100));

        render_polygon(canvas, &transform, &chunk.shape)?;
    }


    Ok(())

}


pub fn render_projectile(
    entity: &Entity,
    canvas: &mut Canvas<Window>,
    assets: &Assets,
    ) -> Result<(), String> {
    let projectile = entity.projectile.as_ref().unwrap();

    let texture_id = match projectile.projectile_type {
        ProjectileType::Bullet => "green_bullet",
        ProjectileType::Missle => "red_missle",
        ProjectileType::Laser => "green_laser"
    };

    if let Some(texture) = assets.get(texture_id) {

        canvas.copy_ex_f(
            texture,
            None,
            FRect::from_center(
                FPoint::new(
                    entity.transform.position.x,
                    entity.transform.position.y),
                texture.query().width as f32 * entity.transform.scale.x,
                texture.query().height as f32 * entity.transform.scale.y,
                ),
            entity.transform.rotation.to_degrees() as f64,
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
