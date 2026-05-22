use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::rendering::assets::Assets;

pub trait Renderable {
    fn render(&self, canvas: &mut Canvas<Window>, assets: &Assets) -> Result<(), String>;
}
