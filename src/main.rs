mod components;
mod entities;
mod game;
mod math;
mod rendering;
mod systems;
mod traits;

use math::shapes::Polygon;
use math::vec2math::Vec2;

use game::game::Game;

fn main() -> Result<(), String> {
    let mut game = Game::new()?;

    game.run()?;

    Ok(())
}
