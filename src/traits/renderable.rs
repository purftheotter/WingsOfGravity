use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::rendering::assets::Assets;

pub trait Renderable {
    fn render(&self, canvas: &mut Canvas<Window>, assets: &Assets) -> Result<(), String>;
}
