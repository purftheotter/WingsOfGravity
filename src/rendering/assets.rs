use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use crate::rendering::pixel_data::PixelData;
use crate::system_utils::*;

pub struct Assets {
    pub textures: HashMap<String, Texture<'static>>,
    pub fonts: HashMap<String, Font<'static, 'static>>,
    pub pixels: HashMap<String, PixelData>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            fonts: HashMap::new(),
            pixels: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        id: &str,
        path: &str,
    ) -> Result<(), String> {
        let path = asset_path(path);
        let texture = texture_creator.load_texture(path)?;
        let mut texture: Texture<'static> = unsafe {
            std::mem::transmute(texture)
        }; 

        texture.set_scale_mode(sdl2::render::ScaleMode::Nearest); 
        self.textures.insert(id.to_string(), texture);
        Ok(())
    }

    pub fn get_texture(&self, id: &str) -> Option<&Texture<'static>> {
        self.textures.get(id)
    }

    pub fn get_mut_texture(&mut self, id: &str) -> Option<&mut Texture<'static>> {
        self.textures.get_mut(id)
    }

    pub fn insert_texture(
        &mut self,
        id: &str,
        texture: Texture<'static>
    ) {
        self.textures.insert(id.to_string(), texture);
    }

    pub fn load_font(
        &mut self,
        ttf: &'static Sdl2TtfContext,
        id: &str,
        path: &str,
        point_size: u16,
    ) -> Result<(), String> {
        let font = ttf.load_font(asset_path(path), point_size)?;
        self.fonts.insert(id.to_string(), font);
        Ok(())
    }

    pub fn get_font(&self, id: &str) -> Option<&Font<'static, 'static>> {
        self.fonts.get(id)
    }

    pub fn load_pixels(
        &mut self,
        id: &str,
        path: &str
    ) -> Result<(), String> {
        self.pixels.insert(id.to_string(), PixelData::load(path)?);
        Ok(())
    }

    pub fn get_pixels(&self, id: &str) -> Option<&PixelData> {
        self.pixels.get(id)
    }
}
