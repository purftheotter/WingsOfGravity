use sdl2::pixels::Color;
use sdl2::rect::FRect;
use sdl2::render::Texture;

use crate::render_context::RenderContext;
use crate::rendering::*;

pub fn render_text_cached(
    rc: &mut RenderContext,
    key: &str,
    font_id: &str,
    text: &str,
    color: Color,
    x: f32,
    y: f32,
) -> Result<(), String> {
    let stale = rc
        .text_cache
        .get(key)
        .map_or(true, |c| c.text != text || c.color != color);

    if stale {
        if text.is_empty() {
            rc.text_cache.remove(key);
            return Ok(());
        }

        let font = rc.assets.get_font(font_id).ok_or("font not found")?;
        let surface = font
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())?;

        let tc = rc.canvas.texture_creator();
        let texture = tc
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        // Same trick you use in Assets::load_texture
        let texture: Texture<'static> = unsafe { std::mem::transmute(texture) };

        // Inserting under the same key drops the old texture, so the cache
        // never grows beyond one entry per key.
        rc.text_cache.insert(
            key.to_string(),
            CachedText { text: text.to_string(), color, texture },
        );
    }

    let cached = &rc.text_cache[key];
    let q = cached.texture.query();
    rc.canvas.copy_f(
        &cached.texture,
        None,
        FRect::new(x, y, q.width as f32, q.height as f32),
    )
}

pub fn render_text(
    render_context: &mut RenderContext,
    font_id: &str,
    text: &str,
    color: sdl2::pixels::Color,
    x: f32,
    y: f32,
) -> Result<(), String> {
    let texture_creator = render_context.canvas.texture_creator();

    let font = 
        render_context.assets.get_font(font_id).ok_or("font not found")?;
    let surface = 
        font.render(text).blended(color).map_err(|e| e.to_string())?;
    let texture = 
        texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
    let q = texture.query();
    render_context.canvas.copy_f(
        &texture,
        None,
        FRect::new(x, y, q.width as f32, q.height as f32))
}
