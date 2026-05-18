mod components;
mod entities;
mod game;
mod rendering;
mod systems;
mod traits;

use components::primitives::Quad;
use components::primitives::Vec2;
use systems::vec2math;

use game::game::Game;

fn main() -> Result<(), String> {
    let mut game = Game::new()?;

    let square: Quad = Quad::new_exact(
        Vec2::new(5.0, 10.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(10.0, 15.0),
        Vec2::new(5.0, 15.0),
    );

    let axis: Vec2 = vec2math::normal(&square.points[0], &square.points[1]);

    println!("x:{}, y:{}", axis.x, axis.y);

    game.run()?;

    Ok(())
}
