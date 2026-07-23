
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
    ship_database: &ShipDatabase,
    rigidbody_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    ) -> Entity {

    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(spawn_point)
        .build();

    let rigid_body_handle = rigidbody_set.insert(rigid_body);

    let player_entity = Entity {
        entity_id,
        rigid_body_handle,
        entity_type: EntityType::Ship,
        ship_component: Some(ShipComponent {
            class: class,
            input: ShipInput::new(),
        }),
        projectile: None,

    };

    let ship_stats = ship_database.get(class);

    let collider = ColliderBuilder::new(ship_stats.hitbox.clone());


    collider_set.insert_with_parent(collider, rigid_body_handle, rigidbody_set);


    player_entity

}

//------------------------------------------------------

pub fn spawn_prjectile(
    entity_id: usize,
    spawn_pos:Pose2,
    impusle_magnitude: Option<f32>,
    projectile_type: ProjectileType,
    damage: f32,
    rigidbody_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
) -> Entity {

    let mut rigid_body = RigidBodyBuilder::dynamic()
        .pose(spawn_pos)
        .build();

    if let Some(impulse) = impusle_magnitude {
        let local_forward =
            Vec2::new(0.0, impulse);
        let impusle_vec =
            rigid_body.rotation().transform_vector(local_forward);
        rigid_body.apply_impulse(impusle_vec, true);
    }

    let rigid_body_handle = rigidbody_set.insert(rigid_body);

    match projectile_type {
        ProjectileType::Bullet => {
            let collider = ColliderBuilder::ball(0.05);
            collider_set.insert_with_parent(
                collider,
                rigid_body_handle, 
                rigidbody_set
            );
        }

        _ => {}
        
    }
    let projectile = Entity {
        entity_id,
        rigid_body_handle,
        entity_type: EntityType::Projectile,
        ship_component: None,
        projectile: Some(
            Projectile {
                projectile_type: projectile_type,
                damage: damage,
            }
        ),

    };

    projectile

}
