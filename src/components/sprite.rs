use crate::rendering::Assets;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Sprite {
    pub texture_id: String,
    pub width: u32,
    pub height: u32,
}

impl Sprite {
    pub fn new(assets: &Assets, texture_id: String) -> Sprite {
        let texture = assets.get(&texture_id).unwrap();
        let width = texture.query().width;
        let height = texture.query().height;
        Sprite {
            texture_id,
            width,
            height
        }
    }
}
