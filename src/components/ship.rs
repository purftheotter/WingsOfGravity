use crate::components::speeds::Cartisian;
use crate::components::speeds::Rotation;

#[derive(Debug,Clone)]
pub enum ShipClass {
    Scout,
    Fighter,
    Freighter,
}

pub struct Ship {
    pub class: ShipClass,
    pub thrust: Cartisian,
    pub turn_rate: Rotation,
}
