
use rapier2d::prelude::*;

use crate::assets::ship_database::ShipDatabase;
use crate::components::projectile::{Projectile, ProjectileType};
use crate::components::ship::ShipInput;
use crate::components::{ShipComponent, ShipClass};
use crate::entity::{Entity,EntityType};

pub fn spawn_player(
    entity_id: usize,
    spawn_point:Vec2,
    class: ShipClass,
    ship_database: &ShipDatabase
    ) -> Entity {

    let mut player_entity = Entity {
        entity_id,
        entity_type: EntityType::Ship,
        ship_component: Some(ShipComponent {
            class: class,
            input: ShipInput::new(),
        }),
        projectile: None,

    };

    let ship_component = player_entity.ship_component.as_ref().unwrap();

    let ship_stats = ship_database.get(ship_component.class);

    player_entity

}

//------------------------------------------------------

pub fn spawn_prjectile(
    entity_id: usize,
    spawn_point:Vec2,
    rotation: f32,
    initial_velocity_magnitude: Option<f32>,
    projectile_type: ProjectileType,
    damage: f32,
) -> Entity {
    let mut projectile = Entity {
        entity_id,
        entity_type: EntityType::Projectile,
       ship_component: None,
        projectile: Some(
            Projectile {
                projectile_type: projectile_type,
                damage: damage,
            }
        )
    };

    projectile

}
