use noise::Fbm;
use noise::NoiseFn;
use noise::Perlin;
use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::dynamics::RigidBodySet;
use rapier2d::math::*;
use rapier2d::geometry::*;
use rapier2d::parry::transformation::vhacd::VHACDParameters;
use rapier2d::parry::transformation::voxelization::*;

use std::collections::HashMap;
use std::f32::consts::PI;

use crate::math::math::PolarIndex;
use crate::physics_world::*;

pub struct Asteroid {
    pub terrain_verts: Vec<AsteroidVert>,
    pub boundary: Vec<PolarIndex>,
    pub hull_handle: ColliderHandle,
    pub index_map: HashMap<PolarIndex, usize>,
    pub sprite_id: Option<String>,
}

pub struct AsteroidVert {
    pub local_transform: Vec2,
    pub index: PolarIndex,
    pub sensor_handle: ColliderHandle,
    pub destroyed: bool,
}


impl Asteroid {
    pub fn new(
        collider_set: &mut ColliderSet,
        rigid_body_set: &mut RigidBodySet,
        rigid_body_handle: RigidBodyHandle,
        subdivisions: usize,
        radius: f32,
        seed: u32,
        noise_strength: f32,
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
            &noise,
            noise_strength,
        );
        let index_map = terrain_verts.iter()
            .enumerate()
            .map(|(array_pos, vert)| (vert.index, array_pos))
            .collect();

        let hull_handle = collider_set.insert_with_parent(
            build_asteroid_collider(
                &boundary,
                &index_map,
                &terrain_verts
            ),
            rigid_body_handle,
            rigid_body_set
            );

        Self {
            terrain_verts,
            boundary,
            hull_handle,
            index_map,
            sprite_id: Some(sprite_id),
        }
    }

    pub fn make_astroid_verts(
        collider_set: &mut ColliderSet,
        subdivisions: &usize,
        base_radius:f32,
        noise: &Fbm<Perlin>,
        noise_strength: f32,
    ) -> (Vec<AsteroidVert>, Vec<PolarIndex>){
        let mut verticies:Vec<AsteroidVert> =  vec![];
        let mut boundary:Vec<PolarIndex> = vec![];

        let subdivisions = *subdivisions;

        let center_index = PolarIndex::new(0, 0);
        verticies.push(
            AsteroidVert::new(
                collider_set,
                Vec2::new(0.0, 0.0),
                center_index
            )
        );

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
                    noise_strength
                );

                let radius = ring_radius * (1.0 + displacement);

                let local_transform = Vec2::new(
                    angle.cos() * radius,
                    angle.sin() * radius
                );

                let polar_idx = PolarIndex::new(i, j);

                verticies.push(
                    AsteroidVert::new(
                        collider_set,
                        local_transform,
                        polar_idx
                    )
                );

                if i == subdivisions - 1 {
                    boundary.push(polar_idx)
                }
            }
        }

        (verticies, boundary)
    }

    pub fn rebuild_asteroid_collider(
        &mut self,
        rigid_body_handle: RigidBodyHandle,
        physics_world: &mut PhysicsWorld,
    ){
        physics_world.collider_set.remove(
            self.hull_handle,
            &mut physics_world.island_manager,
            &mut physics_world.rigid_body_set,
            true,
            );

        let outline: Vec<Vec2> = self.boundary.iter()
            .map(|polar_index| {
                let array_pos = self.index_map[polar_index];
                self.terrain_verts[array_pos].local_transform
            })
            .collect();

        let new_collider = 
            ColliderBuilder::convex_decomposition(
            &outline,
            &Asteroid::build_polyline_indices(outline.len()),
        ).restitution(0.2).build();

        let new_hull_handle = physics_world.collider_set
            .insert_with_parent(
                new_collider,
                rigid_body_handle,
                &mut physics_world.rigid_body_set);

        self.hull_handle = new_hull_handle;

    }

    pub fn build_polyline_indices(outline_len: usize) -> Vec<[u32; 2]> {
        (0..outline_len)
            .map(|i| [i as u32, ((i + 1) % outline_len) as u32])
            .collect()
    }

    pub fn build_boundary(&mut self){
        let mut new_boundary:Vec<PolarIndex> = vec![];
        for polar_index in self.boundary.iter() {
            let array_pos = self.index_map[polar_index];
            if self.terrain_verts[array_pos].destroyed {
                new_boundary.append(
                    &mut self.find_intact_vert(&polar_index)
                );
            }else {
                new_boundary.push(*polar_index);
            }
        }
        new_boundary.dedup();
        self.boundary = new_boundary;
        println!("{:?}", self.boundary.len());
    }

    pub fn radius_displacement(noise:&Fbm<Perlin>, angle: f32, wobble_strength: f32) -> f32 {
        let sample_x = angle.cos() as f64;
        let sample_y = angle.sin() as f64;

        let raw_dis = noise.get([sample_x, sample_y]);
        raw_dis as f32 * wobble_strength
    }

    pub fn find_intact_vert(
        &self,
        polar_index: &PolarIndex
    ) -> Vec<PolarIndex> {
    let array_pos = self.index_map[polar_index];

    if !self.terrain_verts[array_pos].destroyed {
        return vec![*polar_index];
    }

    if polar_index.ring == 0 {
        return vec![*polar_index];
    }

    let from_segments = 6 * polar_index.ring;
    let to_ring = polar_index.ring - 1;
    let to_segments = if to_ring == 0 { 1 } else { 6 * to_ring };

    let candidates = Asteroid::remap_point(
        polar_index.point,
        from_segments,
        to_segments,
    );

    let mut intact_verts = vec![];

    for point in candidates {
        let point = if to_ring == 0 { 0 } else { point };
        let next_index = PolarIndex::new(to_ring, point);
        let resolved_verts = self.find_intact_vert(&next_index);

        for vert in resolved_verts.iter() {
            let resolved_pos = self.index_map[&vert];
            if !self.terrain_verts[resolved_pos].destroyed {
                intact_verts.push(*vert); 
            }
        }

    }

    intact_verts
}
    
    pub fn remap_point(point: usize, from_segment:usize, to_segment:usize) -> Vec<usize> {
        if to_segment == 0 {
            return vec![0];
        }

        let scaled = point as f32 * to_segment as f32 / from_segment as f32;
        let lower = (scaled.floor() as usize) % to_segment;
        let upper = (scaled.ceil() as usize) % to_segment;

        if lower == upper {
            vec![lower]
        }else {
            vec![lower, upper]
        }
    }

    pub fn update_asteroid(
        &mut self,
        rigid_body_handle: RigidBodyHandle,
        physics_world: &mut PhysicsWorld,
    ){
        let mut verts_changed: bool = false;
        for vert in self.terrain_verts.iter_mut() {
            if vert.update_vert(&physics_world.narrow_phase) {
                verts_changed = true;
            }
        }
        
        if verts_changed{
            self.build_boundary();
            self.rebuild_asteroid_collider(
                rigid_body_handle,
                physics_world
            );
        }
    }

}

impl AsteroidVert {
    pub fn new(
        collider_set: &mut ColliderSet,
        local_transform: Vec2,
        index: PolarIndex,
        ) -> Self{
        let sensor = ColliderBuilder::ball(1.0)
            .sensor(true)
            .collision_groups(InteractionGroups::new(
                    ASTEROID_VERT,
                    ASTEROID_VERT|BULLET,
                    InteractionTestMode::And,
                ))
            .build();
        let sensor_handle = collider_set.insert(sensor);

        Self { local_transform, index, sensor_handle, destroyed: false}

    }

    pub fn update_vert(&mut self, narrow_phase: &NarrowPhase) -> bool{
        let is_intersecting = narrow_phase
            .intersection_pairs_with(self.sensor_handle)
            .any(|(_,_, intersecting)| intersecting);

        if is_intersecting {
            self.destroyed = true;
            true
        }else {
            false
        }
    }
    
}


pub fn build_asteroid_collider(
    boundary: &Vec<PolarIndex>,
    index_map: &HashMap<PolarIndex, usize>,
    terrain_verts: &Vec<AsteroidVert>,
) -> Collider {
    let outline: Vec<Vec2> = boundary.iter()
        .map(|polar_index| {
            let array_pos = index_map[polar_index];
            terrain_verts[array_pos].local_transform
        })
        .collect();
    ColliderBuilder::convex_decomposition(
        &outline,
        &Asteroid::build_polyline_indices(outline.len()),
        ).restitution(0.2).build()
}
