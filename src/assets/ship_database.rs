use std::fs;
use crate::components::{ShipStats,ShipClass};
use crate::system_utils::directory::asset_path;

#[derive(serde::Deserialize)]
pub struct ShipDatabase {
    pub scout: ShipStats,
    pub fighter: ShipStats,
    pub freighter: ShipStats,

}

impl ShipDatabase {
    pub fn load() -> Result<Self,Box<dyn std::error::Error>> {
        let path = asset_path("/assets/ships.ron");
        let ron_string =
            fs::read_to_string(path)?;

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
