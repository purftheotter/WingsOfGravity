use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use crate::system_utils::directory::asset_path;

pub struct Assets {
    pub textures: HashMap<String, Texture<'static>>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
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

    pub fn get(&self, id: &str) -> Option<&Texture<'static>> {
        self.textures.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Texture<'static>> {
        self.textures.get_mut(id)
    }

    pub fn insert_texture(&mut self, id: &str, texture: Texture<'static>) {
        self.textures.insert(id.to_string(), texture);
    }   
}
