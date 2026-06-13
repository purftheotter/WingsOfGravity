use crate::components::asteroid::{Asteroid, AsteroidChunk};
use crate::components::ship::ShipInput;
use crate::components::velocity::Velocity;
use crate::components::{ShipComponent, ShipClass};
use crate::components::transform::Transform;
use crate::entity::{Entity,EntityType};
use crate::math::shapes::Polygon;
use crate::math::vec2::Vec2;

pub fn spawn_player(spawn_point:Vec2, class: ShipClass) -> Entity {


    Entity {
        entity_type: EntityType::Ship,

        transform: Transform {
            position: spawn_point,
            rotation: 0.0,
            scale: Vec2::new(2.0, 2.0),
        },
        velocity: Velocity { linear: Vec2::new(0.0,0.0),angular: 0.0 },
        ship_component: Some(ShipComponent {
            class: class,
            input: ShipInput::new(),
        }),
        asteroid: None,

    }

}

pub fn spaw_asteroid(spawn_point:Vec2, scale: f32) -> Entity {
    Entity {
        entity_type: EntityType::Asteroid,
        transform: Transform {
            position: spawn_point,
            rotation: 0.0,
            scale: Vec2::new(scale,scale),
        },
        velocity: Velocity {
            linear: Vec2::new(0.0, 0.0),
            angular: 0.0,
        },
        ship_component: None,
        asteroid: Some(
            Asteroid {
                root: AsteroidChunk {
                    shape: Polygon::new(vec![
                               Vec2::new(-100.0, -100.0),
                               Vec2::new(100.0,-100.0),
                               Vec2::new(100.0,100.0),
                               Vec2::new(-100.0, 100.0)
                    ]),
                    offset: Vec2::new(0.0, 0.0),
                    health: 4.0,
                    max_health: 4.0,
                    destroyed: false,
                    depth: 0,
                    children: None 
                },
                max_depth: 2,
            }
        )
    }
}
