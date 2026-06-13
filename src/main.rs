mod components;
mod entity;
mod game;
mod math;
mod rendering;
mod systems;
mod assets;

use game::game::Game;

fn main() -> Result<(), String> {
    let mut game = Game::new()?;

    game.run()?;

    Ok(())
}
