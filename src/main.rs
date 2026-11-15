mod app;
mod logging;
mod camera;
mod gfx;
mod sprite_renderer;
mod texture;
mod sprite;
mod instance;
mod vertex;


mod components;
mod entity;
mod game;
mod physics_world;
mod systems;
mod assets;
mod system_utils;
mod core;

fn main() -> anyhow::Result<()>{
    app::run()?;
    Ok(())
}
