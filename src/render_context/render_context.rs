use rapier2d::math::*;

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::pixels::Color;

pub struct RenderContext {
    pub canvas: Canvas<Window>,
    pub screen_width: u32,
    pub screen_height: u32,
    pub pixels_per_meter: f32,
    pub camera_position: Vec2,
}

impl RenderContext {
    pub fn new(
        canvas: Canvas<Window>,
        screen_width:u32,
        screen_height:u32,
        pixels_per_meter:f32,
        ) -> Self {
        Self { 
            canvas, 
            screen_width, 
            screen_height, 
            pixels_per_meter,
            camera_position: Vec2::new(0.0, 0.0)
        }
    }

    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        let relative = world - self.camera_position;

        Vec2::new(
            relative.x * self.pixels_per_meter 
                + self.screen_width as f32 /2.0,
            relative.y * self.pixels_per_meter
                + self.screen_height as f32 /2.0,
        )
    }

    pub fn world_to_screen_scale(&self, world_length: f32) -> f32 {
        world_length * self.pixels_per_meter
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
