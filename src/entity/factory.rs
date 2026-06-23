use crate::components::asteroid::{Asteroid, AsteroidChunk};
use crate::components::rigidbody::RigidBody;
use crate::components::ship::ShipInput;
use crate::components::velocity::Velocity;
use crate::components::{ShipComponent, ShipClass};
use crate::components::transform::Transform;
use crate::entity::{Entity,EntityType};
use crate::math::shapes::polygon::Polygon;
use crate::math::vec2::Vec2;

pub fn spawn_player(spawn_point:Vec2, class: ShipClass) -> Entity {


    Entity {
        entity_type: EntityType::Ship,

        transform: Transform {
            position: spawn_point,
            rotation: 0.0,
            scale: Vec2::new(2.0, 2.0),
        },

        rigidbody: RigidBody {
            mass: 1.0,
            inertia: 0.5,
            velocity: Velocity {
                linear: Vec2::new(0.0, 0.0),
                angular: 0.0,
            },
        },

        ship_component: Some(ShipComponent {
            class: class,
            input: ShipInput::new(),
        }),
        asteroid: None,

    }

}

pub fn spawn_asteroid(spawn_point:Vec2, scale: f32) -> Entity {
    Entity {
        entity_type: EntityType::Asteroid,
        transform: Transform {
            position: spawn_point,
            rotation: 0.0,
            scale: Vec2::new(scale,scale),
        },
        rigidbody: RigidBody {
            mass: 10.0,
            inertia: 5.0,
            velocity: Velocity {
                linear: Vec2::new(0.0, 0.0),
                angular: 0.0,
            }
        },
        ship_component: None,
        asteroid: Some(
            Asteroid {
                root: AsteroidChunk::new(
                          Polygon::new(vec![
                            Vec2::new(-100.0, -100.0),
                            Vec2::new(100.0, -100.0),
                            Vec2::new(100.0, 100.0),
                            Vec2::new(-100.0, 100.0),
                          ]), 4.0, 0),
                max_depth: 4,
            }
        )
    }
}
