use rapier2d::geometry::SharedShape;


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

#[derive(Clone, serde::Deserialize)]
pub struct ShipStats {

    pub thrust: f32,
    pub torque: f32,
 
    pub hitbox:SharedShape,

    pub texture_id: String,
    pub scale: f32,
}


#[derive(Default)]
pub struct ShipInput {
    pub thrust: f32,
    pub turn: f32,
    pub primary_fire: bool,
}

impl ShipInput {
    pub fn zero(&mut self) {
        self.thrust = 0.0;
        self.turn = 0.0;
        
    }

    pub fn new() -> ShipInput {
        ShipInput { thrust: 0.0, turn: 0.0, primary_fire: false}
    }

}

