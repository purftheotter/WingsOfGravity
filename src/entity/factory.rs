
use rapier2d::prelude::*;

use crate::assets::ShipDatabase;
use crate::components::projectile::Projectile;
use crate::components::ship::ShipInput;
use crate::components::weapons::Weapon;
use crate::components::*;
use crate::entity::*;
use crate::components::asteroid::*;
use crate::physics_world::*;

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
            weapons: vec![Weapon {
                weapon_type: weapons::WeaponType::MachineGun,
                local_offset: Pose2::new(Vec2::ZERO, 0.0),
                projectile_width: 0.5,
                init_vel_mag: 1.0,
                explosion_width: None,
                rpm: 60.0,
                mag_size: 20,
                cooldown_remaining: 0.0,
            }]
        }),
        projectile: None,
        asteroid: None

    };

    let ship_stats = ship_database.get(class);

    let collider = 
        ColliderBuilder::new(
            ship_stats.hitbox.clone()
        ).collision_groups(InteractionGroups::new(
            SHIP,
            SHIP | ASTEROID_HULL,
            InteractionTestMode::And,
            ))
        .build();

    collider_set.insert_with_parent(collider, rigid_body_handle, rigidbody_set);


    player_entity

}

//------------------------------------------------------

pub fn fire_prjectile(
    entity_id: usize,
    spawn_pos:Pose2,
    init_vel_mag: Option<f32>,
    weapon: &Weapon,
    rigidbody_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
) -> Entity {

    let rigid_body = RigidBodyBuilder::dynamic()
        .pose(spawn_pos)
        .ccd_enabled(true)
        .lock_rotations()
        .build();
    let rigid_body_handle = rigidbody_set.insert(rigid_body);

    let projectile = Some(Projectile::new(
        weapon,
        spawn_pos,
        rigid_body_handle,
        collider_set, 
        rigidbody_set
        ));

    let entity = Entity {
        entity_id,
        rigid_body_handle,
        position: spawn_pos,
        entity_type: EntityType::Projectile,
        ship_component: None,
        projectile,
        asteroid: None

    };

    let rigid_body = rigidbody_set.get_mut(entity.rigid_body_handle).unwrap();

    if let Some(impulse) = init_vel_mag {
        let local_forward =
            Vec2::new(0.0, impulse);
        let world_impusle =
            rigid_body.rotation().transform_vector(local_forward);
        rigid_body.apply_impulse(world_impusle, true);
    }


    entity

}

//------------------------------------------------------

pub fn spawn_asteroid(
    entity_id: usize,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    spawn_pos: Pose2,
    subdivisions: usize,
    radius: f32,
    noise_strength: f32,
) -> Entity {

    let rigid_body = RigidBodyBuilder::dynamic()
        .pose(spawn_pos)
        .build();

    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let mut entity = Entity {
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
                noise_strength,
                "asteroid".to_string()
        ))
    };

    let asteroid = entity.asteroid.as_mut().unwrap();

    collider_set.insert_with_parent(
        asteroid.build_asteroid_collider(),
        rigid_body_handle,
        rigid_body_set
    );

    entity
    
}


