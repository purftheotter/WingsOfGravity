use crate::components::speeds::{Cartisian, Rotation};
use crate::components::velocity::Velocity;
use crate::components::sprite::Sprite;
use crate::components::{Ship, ShipClass};
use crate::components::hitbox::Hitbox;
use crate::components::transform::Transform;
use crate::entity::{Entity,EntityType};
use crate::math::vec2math::Vec2;

pub fn spawn_player(x:f32, y:f32) -> Entity {
    Entity {
        name: "Player".to_string(),
        entity_type: EntityType::Ship,

        transform: Transform {
            position: Vec2::new(x, y),
            rotation: 0.0,
            scale: Vec2::new(2.0, 2.0),
        },
        velocity: Velocity { velocity: Vec2::new(0.0,0.0) },
        hitbox: Hitbox::Polygon { points: vec![
            Vec2::new(0.0, -13.86),
            Vec2::new(-16.0, 13.86),
            Vec2::new(16.0, 13.86),
        ] },
        sprite: Sprite { texture_id: "fighter".to_string() },
        ship: Some(Ship {
            class: ShipClass::Scout,
            thrust: Cartisian { speed: 300.0 },
            turn_rate: Rotation { speed: 200.0},

        })

    }

}
