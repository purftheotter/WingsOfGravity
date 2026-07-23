use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::math::Pose2;

use crate::components::ship::ShipComponent;
use crate::components::projectile::Projectile;

#[derive(Debug, PartialEq, Eq)]
pub enum EntityType {
    Ship,
    Asteroid,
    Projectile,
}

pub struct Entity {
    pub entity_id: usize,

    pub rigid_body_handle: RigidBodyHandle,

    pub position: Pose2,

    pub entity_type: EntityType,

    pub ship_component: Option<ShipComponent>,

    pub projectile: Option<Projectile>
}
