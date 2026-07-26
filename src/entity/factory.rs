
use rapier2d::prelude::*;

use crate::assets::ShipDatabase;
use crate::components::projectile::*;
use crate::components::ship::ShipInput;
use crate::components::*;
use crate::entity::*;
use crate::components::asteroid::*;

pub fn spawn_player(
    entity_id: usize,
    spawn_pos:Pose2,
    class: ShipClass,
    ship_database: &ShipDatabase,
    rigidbody_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    ) -> Entity {

    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(spawn_pos.translation)
        .build();

    let rigid_body_handle = rigidbody_set.insert(rigid_body);

    let player_entity = Entity {
        entity_id,
        rigid_body_handle,
        position: spawn_pos,
        entity_type: EntityType::Ship,
        ship_component: Some(ShipComponent {
            class: class,
            input: ShipInput::new(),
        }),
        projectile: None,
        asteroid: None

    };

    let ship_stats = ship_database.get(class);

    let collider = ColliderBuilder::new(ship_stats.hitbox.clone()).build();

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
        let world_impusle =
            rigid_body.rotation().transform_vector(local_forward);
        rigid_body.apply_impulse(world_impusle, true);
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
        position: spawn_pos,
        entity_type: EntityType::Projectile,
        ship_component: None,
        projectile: Some(
            Projectile {
                projectile_type: projectile_type,
                damage: damage,
            }
        ),
        asteroid: None

    };

    projectile

}

//------------------------------------------------------

pub fn spawn_asteroid(
    entity_id: usize,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    spawn_pos:Pose2,
    subdivisions: usize,
    radius:f32,
) -> Entity {

    let rigid_body = RigidBodyBuilder::dynamic()
        .pose(spawn_pos)
        .build();

    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let entity = Entity {
        entity_id,
        rigid_body_handle,
        position: spawn_pos,
        entity_type: EntityType::Asteroid,
        ship_component: None,
        projectile: None,
        asteroid: Some(Asteroid::new(
                collider_set,
                subdivisions,
                radius,
                entity_id as u32,
                "asteroid".to_string()
        ))
    };

    let asteroid = entity.asteroid.as_ref().unwrap();

    collider_set.insert_with_parent(
        asteroid.build_asteroid_collider(),
        rigid_body_handle,
        rigid_body_set
    );

    entity
    
}


