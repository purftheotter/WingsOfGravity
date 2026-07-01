use crate::components::rigidbody::RigidBody;
use crate::components::transform::Transform;
use crate::components::ship::ShipComponent;
use crate::components::asteroid::Asteroid;
use crate::components::projectile::Projectile;

#[derive(Debug, PartialEq, Eq)]
pub enum EntityType {
    Ship,
    Asteroid,
    Projectile,
}

pub struct Entity {
    pub entity_id: usize,

    pub entity_type: EntityType,
    
    pub transform: Transform,

    pub rigidbody: RigidBody,

    pub ship_component: Option<ShipComponent>,

    pub asteroid: Option<Asteroid>,

    pub projectile: Option<Projectile>
}
