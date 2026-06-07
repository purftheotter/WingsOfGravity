use crate::components::sprite::Sprite;
use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::components::hitbox::Hitbox;
use crate::components::ship::ShipComponent;
use crate::components::asteroid::Asteroid;

pub enum EntityType {
    Ship,
    Astroid,
}

pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    
    pub transform: Transform,
    pub velocity: Velocity,
    pub hitbox: Hitbox,

    pub sprite: Option<Sprite>,
    
    pub ship_component: Option<ShipComponent>,

    pub asteroid: Option<Asteroid>,
}

impl Entity {
    //idk some code ig
}

