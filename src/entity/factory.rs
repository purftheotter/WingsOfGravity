use crate::assets::ship_database::ShipDatabase;
use crate::components::asteroid::{Asteroid, AsteroidChunk};
use crate::components::rigidbody::RigidBody;
use crate::components::hitbox::Hitbox;
use crate::components::ship::ShipInput;
use crate::components::velocity::Velocity;
use crate::components::{ShipComponent, ShipClass};
use crate::components::transform::Transform;
use crate::entity::{Entity,EntityType};
use crate::math::shapes::polygon::Polygon;
use crate::math::vec2::Vec2;
use crate::systems::physics::polygon_inertia;

pub fn spawn_player(spawn_point:Vec2, class: ShipClass, ship_database: &ShipDatabase) -> Entity {

    let mut player_entity = Entity {
        entity_type: EntityType::Ship,

        transform: Transform {
            position: spawn_point,
            rotation: 0.0,
            scale: Vec2::new(2.0, 2.0),
        },

        rigidbody: RigidBody {
            mass: 1.0,
            inertia: 67.0,
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

    };

    let ship_component = player_entity.ship_component.as_ref().unwrap();

    let ship_stats = ship_database.get(ship_component.class);

    let ship_hitbox:Option<&Polygon> = match &ship_stats.hitbox {
        Hitbox::Circle { circle: _ } => {
            None
        }

        Hitbox::Polygon { polygon } => {
            Some(&polygon)
        }
    };

    if ship_hitbox.is_some() {
        player_entity.rigidbody.inertia = 
            polygon_inertia(&player_entity.rigidbody.mass, ship_hitbox.expect("player failed to get_inertia"));
    }

    player_entity

}

pub fn spawn_asteroid(spawn_point:Vec2, scale: f32) -> Entity {
    let mut asteroid_entity = Entity {
        entity_type: EntityType::Asteroid,
        transform: Transform {
            position: spawn_point,
            rotation: 0.0,
            scale: Vec2::new(scale,scale),
        },
        rigidbody: RigidBody {
            mass: 10.0,
            inertia: 0.0,
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
    };

    let asteroid = asteroid_entity.asteroid.as_ref().unwrap();

    asteroid_entity.rigidbody.inertia = asteroid.get_inertia(&asteroid_entity.rigidbody.mass, &asteroid_entity.transform).expect("asteroid failed to get_inertia");
    println!("{}", &asteroid_entity.rigidbody.inertia);

    asteroid_entity
    
}
