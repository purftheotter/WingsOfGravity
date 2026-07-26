mod components;
mod entity;
mod game;
mod render_context;
mod physics_world;
mod rendering;
mod systems;
mod assets;
mod system_utils;
mod math;

use game::game::Game;

fn main() -> Result<(), String> {
    let mut game = Game::new()?;

    game.run()?;

    Ok(())
}
