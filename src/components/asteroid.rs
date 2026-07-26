use noise::Fbm;
use noise::NoiseFn;
use noise::Perlin;
use rapier2d::math::*;
use rapier2d::geometry::*;

use std::collections::HashMap;
use std::f32::consts::PI;

use crate::math::math::PolarIndex;

pub struct Asteroid {
    pub terrain_verts: Vec<AsteroidVert>,
    pub boundary: Vec<PolarIndex>,
    pub index_map: HashMap<PolarIndex, usize>,
    pub sprite_id: Option<String>,
}

impl Asteroid {

    pub fn new(
        collider_set: &mut ColliderSet,
        subdivisions: usize,
        radius: f32,
        seed: u32,
        sprite_id: String,
        ) -> Self {

        let mut noise = Fbm::<Perlin>::new(seed);
        noise.octaves = 4;
        noise.frequency = 0.5;
        noise.lacunarity = 2.0;
        noise.persistence = 0.5;

        let (terrain_verts, boundary) = Asteroid::make_astroid_verts(
            collider_set,
            &subdivisions,
            radius,
            &noise
        );
        let index_map = terrain_verts.iter()
            .enumerate()
            .map(|(array_pos, vert)| (vert.index, array_pos))
            .collect();

        Self {
            terrain_verts,
            index_map,
            boundary,
            sprite_id: Some(sprite_id),
        }
    }

    pub fn make_astroid_verts(
        collider_set: &mut ColliderSet,
        subdivisions: &usize,
        base_radius:f32,
        noise: &Fbm<Perlin>,
    ) -> (Vec<AsteroidVert>, Vec<PolarIndex>){
        let mut verticies:Vec<AsteroidVert> =  vec![];
        let mut boundary:Vec<PolarIndex> = vec![];

        let subdivisions = *subdivisions;

        for i in 0..subdivisions{
            let ring_radius =
                (2.0 * i as f32)
                * (base_radius / subdivisions as f32);
            
            let segments = 6 * i;

            for j in 0..segments {
                
                let angle = 
                    (2.0 * PI * j as f32 / segments as f32) 
                        + (PI * (6.0 * i as f32));

                let displacement = Asteroid::radius_displacement(
                    noise,
                    angle,
                    0.2
                );

                let radius = ring_radius * (1.0 + displacement);

                let local_transform = Vec2::new(
                    angle.cos() * radius,
                    angle.sin() * radius
                );

                let polar_idx = PolarIndex::new(i, j);

                verticies.push(AsteroidVert::new(collider_set, local_transform, polar_idx));

                if i == subdivisions - 1 {
                    boundary.push(polar_idx)
                }
            }
        }

        (verticies, boundary)
    }

    pub fn build_asteroid_collider(&self) -> Collider {
        let outline: Vec<Vec2> = self.boundary.iter()
            .map(|polar_index| {
                let array_pos = self.index_map[polar_index];
                self.terrain_verts[array_pos].local_transform
            })
            .collect();
            

        ColliderBuilder::convex_decomposition(
            &outline,
            &Asteroid::build_polyline_indices(outline.len()),
            ).build()
    }

    pub fn build_polyline_indices(outline_len: usize) -> Vec<[u32; 2]> {
        (0..outline_len)
            .map(|i| [i as u32, ((i + 1) % outline_len) as u32])
            .collect()
    }

    fn radius_displacement(noise:&Fbm<Perlin>, angle: f32, wobble_strength: f32) -> f32 {
        let sample_x = angle.cos() as f64;
        let sample_y = angle.sin() as f64;

        let raw_dis = noise.get([sample_x, sample_y]);
        raw_dis as f32 * wobble_strength
    }

}


pub struct AsteroidVert {
    pub local_transform: Vec2,
    pub index: PolarIndex,
    pub sensor_handle: ColliderHandle,
    pub destroid: bool,
}

impl AsteroidVert {
    pub fn new(
        collider_set: &mut ColliderSet,
        local_transform: Vec2,
        index: PolarIndex,
        ) -> Self{
        let sensor = ColliderBuilder::ball(1.0)
            .sensor(true)
            .build();
        let sensor_handle = collider_set.insert(sensor);

        Self { local_transform, index, sensor_handle, destroid: false}
    }
    
}
