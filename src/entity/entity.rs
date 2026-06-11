use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::components::ship::ShipComponent;
use crate::components::asteroid::Asteroid;

pub enum EntityType {
    Ship,
    Astroid,
}

pub struct Entity {
    pub entity_type: EntityType,
    
    pub transform: Transform,
    pub velocity: Velocity,

    pub ship_component: Option<ShipComponent>,

    pub asteroid: Option<Asteroid>,
}
