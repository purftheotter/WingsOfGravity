
use rapier2d::prelude::*;

use crate::assets::ShipDatabase;
use crate::components::projectile::Projectile;
use crate::components::ship::ShipInput;
use crate::components::sprite::Sprite;
use crate::components::weapons::Weapon;
use crate::components::*;
use crate::entity::*;
use crate::components::asteroid::*;
use crate::physics_world::*;
use crate::rendering::Assets;

pub fn spawn_player(
    spawn_pos:Pose2,
    class: ShipClass,
    ship_database: &ShipDatabase,
    rigidbody_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    assets: &Assets,
) -> Entity {
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(spawn_pos.translation)
        .build();

    let rigid_body_handle = rigidbody_set.insert(rigid_body);

    let sprite = Sprite::new(assets, "fighter".to_string());

    let player_entity = Entity {
        id: EntityId::PLACEHOLDER,
        rigid_body_handle,
        position: spawn_pos,
        entity_type: EntityType::Ship,
        ship_component: Some(ShipComponent {
            class,
            sprite,
            input: ShipInput::new(),
            weapons: vec![Weapon {
                weapon_type: weapons::WeaponType::MachineGun,
                local_offset: Pose2::new(Vec2::ZERO, 0.0),
                projectile_width: 0.5,
                init_vel_mag: 10.0,
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

pub fn fire_prjectile(
    spawn_pos: Pose2,
    velocity: Vec2,
    weapon: &Weapon,
    rigidbody_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    assets: &Assets,
) -> Entity {
    let rigid_body = RigidBodyBuilder::dynamic()
        .pose(spawn_pos)
        .linvel(velocity)
        .ccd_enabled(true)
        .lock_rotations()
        .build();

    let rigid_body_handle = rigidbody_set.insert(rigid_body);

    let projectile = Some(Projectile::new(
        weapon,
        rigid_body_handle,
        collider_set, 
        rigidbody_set,
        assets,
    ));

    let entity = Entity {
        id: EntityId::PLACEHOLDER,
        rigid_body_handle,
        position: spawn_pos,
        entity_type: EntityType::Projectile,
        ship_component: None,
        projectile,
        asteroid: None

    };

    entity
}

pub fn spawn_asteroid(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    assets: &Assets,
    spawn_pos: Pose2,
    subdivisions: usize,
    radius: f32,
    seed: u32,
    noise_strength: f32,
) -> Entity {
    let rigid_body = RigidBodyBuilder::dynamic()
        .can_sleep(false)
        .pose(spawn_pos)
        .build();

    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let asteroid = Asteroid::new(
        collider_set,
        rigid_body_set,
        &rigid_body_handle,
        subdivisions,
        radius,
        seed,
        noise_strength,
        Sprite::new(assets, "asteroid".to_string())
    );

    let entity = Entity {
        id: EntityId::PLACEHOLDER,
        rigid_body_handle,
        position: spawn_pos,
        entity_type: EntityType::Asteroid,
        ship_component: None,
        projectile: None,
        asteroid: Some(asteroid)
    };

    entity
}


