use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct Assets<'a> {
    pub textures: HashMap<String, Texture<'a>>,
}

impl<'a> Assets<'a> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        id: &str,
        path: &str,
    ) -> Result<(), String> {
        let mut texture = texture_creator.load_texture(path).expect("woops Assets Failed");
        texture.set_scale_mode(sdl2::render::ScaleMode::Nearest); 
        self.textures.insert(id.to_string(), texture);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Texture<'a>> {
        self.textures.get(id)
    }
}
