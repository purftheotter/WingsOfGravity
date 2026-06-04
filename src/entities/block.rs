use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::components::sprite::Sprite;
use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::rendering::assets::Assets;
use crate::traits::renderable::Renderable;

pub struct Stone {
    pub transform: Transform,
    pub velocity: Velocity,
    pub sprite: Sprite,
}

impl Stone {
    pub fn new(x: f32, y: f32, x_velocity: f32, y_velocity: f32, texture_id: &str) -> Self {
        Self {
            transform: Transform {
                x,
                y,
                rotation: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
            },
            velocity: Velocity {
                x: x_velocity,
                y: y_velocity,
            },
            sprite: Sprite {
                texture_id: texture_id.to_string(),
            },
        }
    }
}

impl Renderable for Stone {
    fn render(&self, canvas: &mut Canvas<Window>, assets: &Assets) -> Result<(), String> {
        if let Some(texture) = assets.get(&self.sprite.texture_id) {
            canvas.copy(
                texture,
                None,
                Rect::new(
                    self.transform.x as i32,
                    self.transform.y as i32,
                    texture.query().width,
                    texture.query().height,
                ),
            ).expect_err("Block render failed");
        }
        Ok(())
    }
}
