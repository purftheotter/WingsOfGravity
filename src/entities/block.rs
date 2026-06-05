use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::Vec2;

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
    pub fn new(position: Vec2, velocity: Vec2, texture_id: &str) -> Self {
        Self {
            transform: Transform {
                position,
                rotation: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
            },
            velocity: Velocity {
                velocity,
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
                    self.transform.position.x as i32,
                    self.transform.position.y as i32,
                    texture.query().width,
                    texture.query().height,
                ),
            ).expect_err("Block render failed");
        }
        Ok(())
    }
}
