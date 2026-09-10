use rapier2d::prelude::*;

use crate::physics_world::*;

use crate::components::weapons::*;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProjectileType {
    Bullet,
    Missle,
    Laser,
}

pub struct Projectile {
    pub projectile_type: ProjectileType,
    pub collider_handle: ColliderHandle,
    pub explosion_handle: Option<ColliderHandle>,
    pub exploded: bool,
}

impl Projectile {

    pub fn new(
        weapon: &Weapon,
        rigid_body_handle: RigidBodyHandle,
        collider_set: &mut ColliderSet,
        rigid_body_set: &mut RigidBodySet,
        ) -> Self {

        let projectile_type = match weapon.weapon_type {
            WeaponType::MachineGun => { ProjectileType::Bullet }
            WeaponType::MissleLauncher => { ProjectileType::Missle }
            WeaponType::LaserBeam => { ProjectileType::Laser }
        };

        let collider_handle;
        let explosion_handle;

        match projectile_type {
            ProjectileType::Bullet => { 
                collider_handle = collider_set.insert_with_parent(
                   ColliderBuilder::ball(weapon.projectile_width / 2.0)
                        .sensor(true)
                        .collision_groups(
                            InteractionGroups::new(
                                BULLET,
                                ASTEROID_HULL,
                                InteractionTestMode::And,
                            )
                        ),
                    rigid_body_handle,
                    rigid_body_set,

                );

                let crater_radius = calc_bullet_crater_radius(weapon.init_vel_mag, weapon.projectile_width);

                explosion_handle = Some(collider_set.insert_with_parent(
                    ColliderBuilder::ball(crater_radius)
                        .sensor(true)
                        .mass(0.0)
                        .enabled(false)
                        .collision_groups(InteractionGroups::new(
                            BULLET,
                            ASTEROID_VERT,
                            InteractionTestMode::And,
                        )),
                    rigid_body_handle,
                    rigid_body_set,
                ));
            }
            ProjectileType::Missle => {
                collider_handle = collider_set.insert_with_parent(
                    ColliderBuilder::ball(weapon.projectile_width / 2.0)
                        .sensor(true)
                        .collision_groups(
                            InteractionGroups::new(
                                BULLET,
                                ASTEROID_HULL,
                                InteractionTestMode::And,
                            )
                        ),
                    rigid_body_handle,
                    rigid_body_set,
                );
                explosion_handle = Some(collider_set.insert_with_parent(
                    ColliderBuilder::ball(weapon.explosion_width.unwrap() / 2.0)
                        .sensor(true)
                        .mass(0.0)
                        .enabled(false)
                        .collision_groups(
                            InteractionGroups::new(
                                BULLET,
                                ASTEROID_VERT,
                                InteractionTestMode::And,
                            )
                    ),
                    rigid_body_handle,
                    rigid_body_set,
                ));

            }
            ProjectileType::Laser => {
                collider_handle = collider_set.insert_with_parent(
                    ColliderBuilder::ball(weapon.projectile_width / 2.0)
                    .sensor(true)
                    .collision_groups(
                        InteractionGroups::new(
                            BULLET,
                            ASTEROID_HULL,
                            InteractionTestMode::And,
                        )
                    ),
                    rigid_body_handle,
                    rigid_body_set,
                );
                explosion_handle = Some(
                    collider_set.insert_with_parent(
                        ColliderBuilder::ball(weapon.projectile_width / 2.0)
                        .sensor(true)
                        .mass(0.0)
                        .enabled(false)
                        .collision_groups(
                            InteractionGroups::new(
                                BULLET,
                                BULLET|ASTEROID_VERT,
                                InteractionTestMode::And,
                            )
                        ),
                        rigid_body_handle,
                        rigid_body_set,
                    )
                );
            }
        }

        Self { 
            projectile_type,
            collider_handle,
            explosion_handle,
            exploded: false,
        }
    }

}

pub fn calc_bullet_crater_radius(velocity: f32, projectile_width: f32) -> f32 {
    let base = projectile_width * 1.5;
    let velocity_scale = velocity.powf(0.5) * 0.01;
    base + velocity_scale
}

