mod components;
mod entities;
mod game;
mod rendering;
mod systems;
mod traits;

use game::game::Game;

fn main() -> Result<(), String> {
    let mut game = Game::new()?;

    game.run()?;

    Ok(())
}
