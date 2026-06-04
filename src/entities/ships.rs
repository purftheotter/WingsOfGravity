use sdl2::rect::{Rect,Point,FPoint,FRect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::components::speeds::Cartisian;
use crate::components::speeds::Rotation;
use crate::components::sprite::Sprite;
use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::rendering::assets::Assets;
use crate::traits::renderable::Renderable;

pub struct Fighter {
    pub transform: Transform,
    pub velocity: Velocity,
    pub thrust: Cartisian,
    pub turn_rate: Rotation,
    pub sprite: Sprite,
}

impl Fighter {
    pub fn new(
        x: f32,
        y: f32,
        rotation: f32,
        scale_x: f32,
        scale_y: f32,
        texture_id: &str,
    ) -> Self {
        Self {
            transform: Transform {
                x,
                y,
                rotation,
                scale_x,
                scale_y,
            },
            velocity: Velocity { x: 0.0, y: 0.0 },
            thrust: Cartisian { speed: (200.0) },
            turn_rate: Rotation { speed: (150.0) },
            sprite: Sprite {
                texture_id: texture_id.to_string(),
            },
        }
    }
}

impl Renderable for Fighter {
    fn render(&self, canvas: &mut Canvas<Window>, assets: &Assets) -> Result<(), String> {
        if let Some(texture) = assets.get(&self.sprite.texture_id) {
            canvas.copy_ex_f(
                texture,
                Rect::new(0, 0, texture.query().width, texture.query().height),
                FRect::new(
                    self.transform.x,
                    self.transform.y,
                    (texture.query().width as f32) * self.transform.scale_x,
                    (texture.query().height as f32) * self.transform.scale_y,
                ),
                self.transform.rotation as f64,
                FPoint::new(
                    ((texture.query().width as f32) * self.transform.scale_x) / 2.0,
                    ((texture.query().height as f32) * self.transform.scale_y) / 2.0,
                ),
                false,
                false,
            ).expect("Ship render");
        }
        Ok(())
    }
}
