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

    let square1: Polygon = Polygon::new(
        vec![
            Vec2::new(5.0, 10.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(10.0, 15.0),
            Vec2::new(5.0, 15.0),
        ],
        Vec2::new(0.0, 0.0),
    );
    let square2: Polygon = Polygon::new(
        vec![
            Vec2::new(0.0, 10.0),
            Vec2::new(5.0, 10.0),
            Vec2::new(5.0, 15.0),
            Vec2::new(0.0, 15.0),
        ],
        Vec2::new(0.0, 0.0),
    );

    game.run()?;

    Ok(())
}
