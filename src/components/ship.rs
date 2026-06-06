use crate::components::hitbox::Hitbox;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Deserialize)]
pub enum ShipClass {
    Scout,
    Fighter,
    Freighter,
}

pub struct ShipComponent {
    pub class: ShipClass,
}

#[derive(serde::Deserialize)]
pub struct ShipStats {
    pub thrust: f32,
    pub angular_thrust: f32,
    pub angular_velocity_dampener: f32,
    pub texture_id: String,
    pub hitbox: Hitbox,
}
