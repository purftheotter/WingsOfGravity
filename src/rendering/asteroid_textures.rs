use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{BlendMode, ScaleMode, Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

use crate::components::asteroid::AsteroidMask;
use crate::rendering::pixel_data::PixelData;

pub struct AsteroidTextures {
    textures: HashMap<u64, Texture<'static>>, // key: your entity id
}

impl AsteroidTextures {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    /// Returns an up-to-date texture for this asteroid.
    /// Only uploads when the mask changed.
    pub fn get(
        &mut self,
        creator: &TextureCreator<WindowContext>,
        id: u64,
        mask: &mut AsteroidMask,
        rock: &PixelData,
    ) -> Result<&Texture<'static>, String> {
        if !self.textures.contains_key(&id) {
            let tex = creator
                .create_texture_streaming(
                    PixelFormatEnum::RGBA32, // byte order R,G,B,A matches to_rgba
                    mask.size as u32,
                    mask.size as u32,
                )
                .map_err(|e| e.to_string())?;
            let mut tex: Texture<'static> = unsafe { std::mem::transmute(tex) };
            tex.set_blend_mode(BlendMode::Blend);
            tex.set_scale_mode(ScaleMode::Nearest); // same as Assets::load_texture
            self.textures.insert(id, tex);
            mask.dirty = true;
        }

        let tex = self.textures.get_mut(&id).unwrap();
        if mask.dirty {
            tex.update(None, &mask.to_rgba(rock), mask.size * 4)
                .map_err(|e| e.to_string())?;
            mask.dirty = false;
        }
        Ok(tex)
    }

    /// Call when an asteroid entity is destroyed.
    pub fn remove(&mut self, id: u64) {
        self.textures.remove(&id);
    }
}
