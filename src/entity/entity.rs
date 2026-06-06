use sdl2::libc::option;
use sdl2::rect::{Rect,FPoint,FRect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::components::speeds::Rotation;
use crate::components::sprite::Sprite;
use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::components::hitbox::Hitbox;
use crate::components::Ship;
use crate::math::vec2math::Vec2;
use crate::rendering::assets::Assets;
use crate::traits::renderable::Renderable;

pub enum EntityType {
    Ship,
    Block,
}

pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    
    pub transform: Transform,
    pub velocity: Velocity,
    pub hitbox: Hitbox,
    pub sprite: Sprite,
    
    pub ship: Option<Ship>,
}

impl Entity {
    pub fn new(
        name: String,
        entity_type: EntityType,
        
        position: Vec2,
        rotation: f32,
        scale: Vec2,
        texture_id: &str,
        ship: Ship,
    ) -> Self {
        Self {
            name,
            entity_type,

            transform: Transform {
                position,
                rotation,
                scale,
            },
            velocity: Velocity {velocity: Vec2::new(0.0, 0.0)},
            hitbox: Hitbox::Circle { radius: 32.0 },
            sprite: Sprite {
                texture_id: texture_id.to_string(),
            },

            ship: Some(ship),
        }
    }
}

impl Renderable for Entity {
    fn render(&self, canvas: &mut Canvas<Window>, assets: &Assets) -> Result<(), String> {
        if let Some(texture) = assets.get(&self.sprite.texture_id) {
            canvas.copy_ex_f(
                texture,
                Rect::new(0, 0, texture.query().width, texture.query().height),
                FRect::from_center(
                    FPoint::new(self.transform.position.x, self.transform.position.y),
                    texture.query().width as f32 * self.transform.scale.x,
                    texture.query().height as f32 * self.transform.scale.y,
                    ),
                self.transform.rotation as f64,
                FPoint::new(
                    self.transform.scale.x + ((texture.query().width as f32 * self.transform.scale.x) / 2.0),
                    self.transform.scale.y + ((texture.query().height as f32 * self.transform.scale.y) / 2.0),
                ),
                false,
                false,
            ).expect("Entity render Failed");
        }
        Ok(())
    }
}
