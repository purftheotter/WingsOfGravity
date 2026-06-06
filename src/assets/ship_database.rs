use std::fs;
use crate::components::{ShipStats,ShipClass};

#[derive(serde::Deserialize)]
pub struct ShipDatabase {
    pub scout: ShipStats,
    pub fighter: ShipStats,
    pub freighter: ShipStats,

}

impl ShipDatabase {
    pub fn load() -> Result<Self,Box<dyn std::error::Error>> {
        let ron_string =
            fs::read_to_string("assets/ships.ron")?;

        let database = ron::from_str(&ron_string)?;

        Ok(database)
    }

    pub fn get(&self, class: ShipClass) -> &ShipStats {
        match class {
            ShipClass::Scout => &self.scout,
            ShipClass::Fighter => &self.fighter,
            ShipClass::Freighter => &self.freighter,
        }
    }
}
