use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::math::Pose2;

use crate::components::asteroid::Asteroid;
use crate::components::ship::ShipComponent;
use crate::components::projectile::Projectile;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct EntityId {
    pub slot: u32,
    pub generation: u32,
}

impl EntityId {
    pub const PLACEHOLDER: EntityId = EntityId {
        slot: u32::MAX,
        generation: u32::MAX
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum EntityType {
    Ship,
    Asteroid,
    Projectile,
}

#[derive(Debug)]
pub struct Entity {
    pub id: EntityId,

    pub rigid_body_handle: RigidBodyHandle,

    pub position: Pose2,

    pub entity_type: EntityType,

    pub ship_component: Option<ShipComponent>,

    pub asteroid: Option<Asteroid>,

    pub projectile: Option<Projectile>,

}
