use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::render::Texture;


pub struct CachedText {
    pub text: String,
    pub color: Color,
    pub texture: Texture<'static>,
}

pub type TextCache = HashMap<String, CachedText>;
