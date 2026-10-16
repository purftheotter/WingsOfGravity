use rapier2d::math::*;

use rapier2d::utils::PoseOps;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::*;

use crate::core::to_i16;
use crate::rendering::Assets;
use crate::rendering::asteroid_textures::AsteroidTextures;
use crate::rendering::text_cache::TextCache;


pub struct RenderContext {
    pub canvas: Canvas<Window>,
    pub assets: Assets,
    pub asteroid_textures: AsteroidTextures,
    pub text_cache: TextCache,
    pub ttf_context: &'static sdl2::ttf::Sdl2TtfContext,
    pub screen_width: u32,
    pub screen_height: u32,
    pub pixels_per_meter: f32,
    pub meters_per_texel:f32,
    pub camera_position: Pose2,
}

impl RenderContext {
    pub fn new(
        canvas: Canvas<Window>,
        assets: Assets,
        asteroid_textures: AsteroidTextures,
        ttf_context:&'static sdl2::ttf::Sdl2TtfContext,
        pixels_per_meter:f32,
        meters_per_texel:f32,
        ) -> Self {
        let (screen_width, screen_height) = canvas.output_size().unwrap();
        Self { 
            canvas,
            assets,
            asteroid_textures,
            text_cache: TextCache::new(),
            ttf_context,
            screen_width, 
            screen_height, 
            pixels_per_meter,
            meters_per_texel,
            camera_position: Pose2::new(Vec2::ZERO, 0.0)
        }
    }

    pub fn update_screen_size(&mut self) {
        if let Ok((w, h)) = self.canvas.output_size() {
            self.screen_width = w;
            self.screen_height = h;
        }
    }

    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        let relative = world - self.camera_position.translation();

        let rotated = self.camera_position.rotation.inverse() * relative;

        Vec2::new(
            rotated.x * self.pixels_per_meter 
                + self.screen_width as f32 /2.0,
            rotated.y * self.pixels_per_meter
                + self.screen_height as f32 /2.0,
        )
    }

    pub fn world_to_screen_scale(&self, world_length: f32) -> f32 {
        world_length * self.pixels_per_meter
    }
    
    pub fn world_to_screen_rot(&self, world_rot: f32) -> f32 {
        world_rot - self.camera_position.rotation.angle()
    }

    pub fn world_to_screen_i16(&self, world: Vec2) -> (i16, i16) {
        let screen = self.world_to_screen(world);
        (to_i16(screen.x), to_i16(screen.y))
    }

    pub fn render_bg(
        &mut self,
        bg_id: &str,
        parallax: f32,
        scale: f32,
        ) -> Result<(), String> {

        if let Some(texture) = self.assets.get_texture(bg_id) {
            let tile = texture.query().width as f32 * scale;
            let cam = self.camera_position.translation * self.pixels_per_meter * parallax;
            let offset_x = (-cam.x).rem_euclid(tile);
            let offset_y = (-cam.y).rem_euclid(tile);

            let cols = 
                (self.screen_width as f32 / tile).ceil() as i32 + 1;
            let rows = 
                (self.screen_height as f32 / tile).ceil() as i32 + 1;

            for row in -1..rows {
                for col in -1..cols {
                    let dst = FRect::new(
                        offset_x + col as f32 * tile,
                        offset_y + row as f32 * tile,
                        tile,
                        tile,
                    );
                    self.canvas.copy_f(texture, None, dst)?;
                }
            }

        }

        Ok(())

    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
