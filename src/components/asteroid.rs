use crate::math::{shapes::Polygon, vec2::Vec2};


pub struct Asteroid {
    pub root: AsteroidChunk,
    pub max_depth: u8,
}

pub struct  AsteroidChunk {
    pub shape: Polygon,
    pub offset: Vec2,
    pub health: f32,
    pub max_health: f32,
    pub destroyed: bool,
    pub depth: u8,
    pub children: Option<[Box<AsteroidChunk>;4]>,
}


impl AsteroidChunk {
    pub fn new(shape:Polygon, offset: Vec2, health:f32, depth:u8) -> Self {
        Self {
            shape: shape,
            offset,
            health,
            max_health: health,
            destroyed: false,
            children: None,
            depth,
        }
    }
    pub fn subdivide(&mut self, max_depth: u8) {
        println!("max_depth:{},my depth:{}", max_depth, self.depth);
        if self.depth >= max_depth {
            return;
        }

        if let Some(children) = &mut self.children {
            children[0].subdivide(max_depth);
            children[1].subdivide(max_depth);
            children[2].subdivide(max_depth);
            children[3].subdivide(max_depth);
        }else {

            let (min_x_size, max_x_size) = self.shape.project_polygon(Vec2::new(1.0, 0.0));

            let (min_y_size, max_y_size) = self.shape.project_polygon(Vec2::new(0.0, 1.0));

            let size = Vec2::new(
                max_x_size - min_x_size,
                max_y_size - min_y_size,
            );

            let health = self.max_health;
            let depth = self.depth + 1;

            let polygon_parts = 
                self.shape.subdivide4();

            self.children = Some([
                Box::new(
                    AsteroidChunk::new(
                        polygon_parts[0].clone(),
                        (-size/4.0) + (size/4.0),
                        health,
                        depth)),
                Box::new(
                    AsteroidChunk::new(
                        polygon_parts[1].clone(),
                        Vec2::new((size.x/4.0) - (size.x /4.0), (-size.y/4.0) + (size.y/4.0)),
                        health,
                        depth)),
                Box::new(
                    AsteroidChunk::new(
                        polygon_parts[2].clone(),
                        Vec2::new(-size.x/4.0 + (size.x/4.0), size.y/4.0 - (size.y/4.0)),
                        health,
                        depth)),
                Box::new(
                    AsteroidChunk::new(
                        polygon_parts[3].clone(),
                        size/4.0 - (size/4.0),
                        health,
                        depth)),


            ])

            
        }

    }
}
