use crate::components::rigidbody::RigidBody;
use crate::components::transform::Transform;
use crate::components::ship::ShipComponent;
use crate::components::asteroid::Asteroid;

#[derive(Debug, PartialEq, Eq)]
pub enum EntityType {
    Ship,
    Asteroid,
}

pub struct Entity {
    pub entity_type: EntityType,
    
    pub transform: Transform,

    pub rigidbody: RigidBody,

    pub ship_component: Option<ShipComponent>,

    pub asteroid: Option<Asteroid>,
}
