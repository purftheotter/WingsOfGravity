use crate::math::shapes::Polygon;


pub struct Asteroid {
    pub root: AsteroidChunk,
    pub max_depth: u8,
}

pub struct  AsteroidChunk {
    pub shape: Polygon,
    pub health: f32,
    pub max_health: f32,
    pub destroyed: bool,
    pub depth: u8,
    pub children: Option<[Box<AsteroidChunk>;4]>,
}

impl AsteroidChunk {
    pub fn new(shape:Polygon, health:f32, depth:u8) -> Self {
        Self {
            shape: shape,
            health,
            max_health: health,
            destroyed: false,
            children: None, 
        }
    }
    pub fn subdivide(&mut self) {
        let health = self.max_health;
        let depth = self.depth + 1;

        self.children = Some(
            Box::new(AsteroidChunk::new(Polygon, health, depth))
        )
    }
}
