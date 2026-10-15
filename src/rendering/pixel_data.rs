
use sdl2::pixels::PixelFormatEnum;
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;

use crate::system_utils::asset_path;


pub struct PixelData {
    pub w: usize,
    pub h: usize,
    pub rgba: Vec<u8>,
}

impl PixelData {
    pub fn load(path: &str) -> Result<Self, String> {
        let surface = Surface::from_file(asset_path(path))?
            .convert_format(PixelFormatEnum::RGBA32)?;
        let (w, h) = (surface.width() as usize, surface.height() as usize);
        let pitch = surface.pitch() as usize;
        let px = surface.without_lock().ok_or("rock surface needs locking")?;

        let mut rgba = Vec::with_capacity(w * h * 4);
        for y in 0..h {
            rgba.extend_from_slice(&px[y * pitch..y * pitch + w * 4]);
        }
        Ok(Self { w, h, rgba })
    }
}
