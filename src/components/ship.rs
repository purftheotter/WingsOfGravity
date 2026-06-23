use crate::components::hitbox::Hitbox;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Deserialize)]
pub enum ShipClass {
    Scout,
    Fighter,
    Freighter,
}

pub struct ShipComponent {
    pub class: ShipClass,
    pub input: ShipInput,
}

#[derive(serde::Deserialize)]
pub struct ShipStats {
    pub mass: f32,
    pub inertia: f32,

    pub thrust: f32,
    pub torque: f32,

    pub linear_drag: f32,
    pub angular_drag: f32,
    
    pub texture_id: String,
    pub hitbox: Hitbox,
}


#[derive(Default)]
pub struct ShipInput {
    pub thrust: f32,
    pub turn: f32,
}

impl ShipInput {
    pub fn zero(&mut self) {
        self.thrust = 0.0;
        self.turn = 0.0;
        
    }

    pub fn new() -> ShipInput {
        ShipInput { thrust: 0.0, turn: 0.0 }
    }

}

