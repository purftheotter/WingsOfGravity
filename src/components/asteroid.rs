use crate::{components::transform::Transform, math::{shapes::polygon::Polygon, vec2::Vec2}, systems::{collision::{Collision, sat_collision}, physics::polygon_inertia}};


pub struct Asteroid {
    pub root: AsteroidChunk,
    pub max_depth: u8,
}

impl Asteroid {
    pub fn collides(
        &self,
        other: &Polygon,
        self_transform: &Transform,
        other_transform: &Transform,
    ) -> Option<Collision> {
        self.root.recursive_collide(other, &self_transform, &other_transform)
    }

    pub fn get_inertia(
        &self,
        mass: &f32,
        transform: &Transform
    )-> Option<f32>{
        self.root.recursive_get_inertia(mass, transform)
    }
    
}

pub struct AsteroidChunk {
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
            depth,
        }
    }
    pub fn subdivide(&mut self, max_depth: u8) {
        if self.depth >= max_depth {
            return;
        }

        if let Some(children) = &mut self.children {
            children[0].subdivide(max_depth);
            children[1].subdivide(max_depth);
            children[2].subdivide(max_depth);
            children[3].subdivide(max_depth);
        }else {

            let health = self.max_health;
            let depth = self.depth + 1;

            let polygon_parts = 
                self.shape.subdivide4();

            self.children = Some([
                Box::new(
                    AsteroidChunk::new(
                        polygon_parts[0].clone(),
                        health,
                        depth)),
                Box::new(
                    AsteroidChunk::new(
                        polygon_parts[1].clone(),
                        health,
                        depth)),
                Box::new(
                    AsteroidChunk::new(
                        polygon_parts[2].clone(),
                        health,
                        depth)),
                Box::new(
                    AsteroidChunk::new(
                        polygon_parts[3].clone(),
                        health,
                        depth)),


            ])

            
        }

    }
    pub fn recursive_collide(
        &self,
        other: &Polygon,
        self_transform: &Transform,
        other_transform: &Transform,
    ) -> Option<Collision> {
        if self.destroyed {
            return None;
        }

        if let Some(children) = &self.children {
            for child in children {
                let result = child.recursive_collide(other, self_transform, other_transform);
                if result.is_some() {
                    return result;
                }
            }
        }

        self.collides(other, self_transform, other_transform)
    }

    pub fn collides(
        &self,
        other: &Polygon,
        self_transform: &Transform,
        other_transform: &Transform,
    ) -> Option<Collision> {
        sat_collision(self_transform, &self.shape, other_transform, other)
    }

    pub fn recursive_get_inertia(
        &self,
        mass: &f32,
        self_transform: &Transform,
    ) -> Option<f32>{
        if self.destroyed {
            return None;
        }

        if let Some(children) = &self.children {
            let mut result:f32 = 0.0;
            for child in children {
                let child_result = child.recursive_get_inertia(mass, self_transform);
                if child_result.is_some() {
                    result += child_result.unwrap();
                }
            }
            return Some(result);
        }

        let self_mass = mass / (4.0 * (self.depth as f32 + 1.0));

        Some(polygon_inertia(&self_mass, &self.shape.apply_transformation(self_transform)))
    }
    
}
