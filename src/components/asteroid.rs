use noise::Fbm;
use noise::NoiseFn;
use noise::Perlin;
use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::dynamics::RigidBodySet;
use rapier2d::math::*;
use rapier2d::geometry::*;

use std::collections::HashMap;
use std::f32::consts::PI;

use crate::components::sprite::Sprite;
use crate::core::*;
use crate::physics_world::*;

pub struct Asteroid {
    pub terrain_verts: Vec<AsteroidVert>,
    pub hull_handle: ColliderHandle,
    pub index_map: HashMap<PolarIndex, usize>,
    pub rings: usize,
    pub sprite: Sprite,

}


impl Asteroid {
    pub fn new(
        collider_set: &mut ColliderSet,
        rigid_body_set: &mut RigidBodySet,
        rigid_body_handle: &RigidBodyHandle,
        subdivisions: usize,
        radius: f32,
        seed: u32,
        noise_strength: f32,
        sprite: Sprite,
        ) -> Self {

        let mut noise = Fbm::<Perlin>::new(seed);
        noise.octaves = 4;
        noise.frequency = 0.5;
        noise.lacunarity = 2.0;
        noise.persistence = 0.5;

        let terrain_verts = Asteroid::make_astroid_verts(
            collider_set,
            rigid_body_set,
            &rigid_body_handle,
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
                subdivisions,
                &index_map,
                &terrain_verts,
            ).expect("asteroid had not triangles: need subdiv >= 2"),
            *rigid_body_handle,
            rigid_body_set
            );

        Self {
            terrain_verts,
            hull_handle,
            index_map,
            rings: subdivisions,
            sprite,
        }
    }

    pub fn make_astroid_verts(
        collider_set: &mut ColliderSet,
        rigid_body_set: &mut RigidBodySet,
        rigid_body_handle: &RigidBodyHandle,
        subdivisions: &usize,
        base_radius:f32,
        noise: &Fbm<Perlin>,
        noise_strength: f32,
    ) -> Vec<AsteroidVert>{
        let mut verticies:Vec<AsteroidVert> =  vec![];

        let subdivisions = *subdivisions;

        let vert_radius =  base_radius / subdivisions as f32;

        let center_index = PolarIndex::new(0, 0);
        verticies.push(
            AsteroidVert::new(
                collider_set,
                rigid_body_set,
                rigid_body_handle,
                Vec2::new(0.0, 0.0),
                center_index,
                vert_radius,
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
                        rigid_body_set,
                        rigid_body_handle,
                        local_transform,
                        polar_idx,
                        vert_radius,
                    )
                );

            }
        }

        verticies
    }

    pub fn build_asteroid_collider(
        &mut self,
        rigid_body_handle: RigidBodyHandle,
        physics_world: &mut PhysicsWorld,
    ) -> bool {
 
        let Some(new_collider) = 
            build_asteroid_collider(self.rings, &self.index_map, &self.terrain_verts) else {
                return false;
            };

        physics_world.collider_set.remove(
            self.hull_handle,
            &mut physics_world.island_manager,
            &mut physics_world.rigid_body_set,
            true,
            );

        self.hull_handle = physics_world.collider_set.insert_with_parent(
                new_collider,
                rigid_body_handle,
                &mut physics_world.rigid_body_set);

        true

    }

    pub fn radius_displacement(noise:&Fbm<Perlin>, angle: f32, wobble_strength: f32) -> f32 {
        let sample_x = angle.cos() as f64;
        let sample_y = angle.sin() as f64;

        let raw_dis = noise.get([sample_x, sample_y]);
        raw_dis as f32 * wobble_strength
    }

    pub fn update_asteroid(
        &mut self,
        rigid_body_handle: RigidBodyHandle,
        physics_world: &mut PhysicsWorld,
    ) -> bool {
        let mut verts_changed: bool = false;
        for vert in self.terrain_verts.iter_mut() {
            if vert.update_vert(&physics_world.narrow_phase, &mut physics_world.collider_set) {
                verts_changed = true;
            }
        }
        
        if verts_changed{
            if !self.build_asteroid_collider(
                rigid_body_handle,
                physics_world
            ) {
                return true;
            }
        }

        false
    }
}

pub struct AsteroidVert {
    pub local_transform: Vec2,
    pub index: PolarIndex,
    pub sensor_handle: ColliderHandle,
    pub destroyed: bool,
}

impl AsteroidVert {
    pub fn new(
        collider_set: &mut ColliderSet,
        rigid_body_set: &mut RigidBodySet,
        rigid_body_handle: &RigidBodyHandle,
        local_transform: Vec2,
        index: PolarIndex,
        sensor_radius: f32,
        ) -> Self{
        let sensor = ColliderBuilder::ball(sensor_radius)
            .sensor(true)
            .translation(local_transform)
            .collision_groups(InteractionGroups::new(
                    ASTEROID_VERT,
                    BULLET,
                    InteractionTestMode::And,
                ))
            .build();
        let sensor_handle = collider_set.insert_with_parent(sensor, *rigid_body_handle, rigid_body_set);

        Self { local_transform, index, sensor_handle, destroyed: false}
    }

    pub fn update_vert(&mut self, narrow_phase: &NarrowPhase, collider_set: &mut ColliderSet) -> bool{
        let is_intersecting = narrow_phase
            .intersection_pairs_with(self.sensor_handle)
            .any(|(_,_, intersecting)| intersecting);

        if is_intersecting {
            self.destroyed = true;
            let collider = collider_set.get_mut(self.sensor_handle).unwrap();
            collider.set_enabled(false);
            true
        }else {
            false
        }
    }
}

pub fn build_asteroid_collider(
    rings: usize,
    index_map: &HashMap<PolarIndex, usize>,
    terrain_verts: &[AsteroidVert],
) -> Option<Collider> {
    let shapes = build_triangle_shapes(rings, index_map, terrain_verts);
    if shapes.is_empty() {
        return None;
    }
    Some(ColliderBuilder::compound(shapes).restitution(0.2).build())
}

fn build_triangle_shapes(
    rings: usize,
    index_map: &HashMap<PolarIndex, usize>,
    terrain_verts: &[AsteroidVert],
) -> Vec<(Pose, SharedShape)> {
    let vert = |ring: usize, point: usize| &terrain_verts[index_map[&PolarIndex::new(ring, point)]];

    let mut shapes = Vec::new();
    let mut add = |a: &AsteroidVert, b: &AsteroidVert, c: &AsteroidVert| {
        if a.destroyed || b.destroyed || c.destroyed {
            return;
        }
        shapes.push((
            Pose::IDENTITY,
            SharedShape::triangle(a.local_transform, b.local_transform, c.local_transform),
        ));
    };

    for r in 1..rings {
        let outer_ring_n = 6 * r;
        let inner_ring_n = if r == 1 { 1 } else { 6 * (r - 1) };

        for s in 0..6 {
            for k in 0..r {
                let outer0 = vert(r, (s * r + k) % outer_ring_n);
                let outer1 = vert(r, (s * r + k + 1) % outer_ring_n);
                let inner0 = vert(r - 1, (s * (r - 1) + k) % inner_ring_n);

                add(outer0, outer1, inner0);

                if k < r - 1 {
                    let inner1 = vert(r - 1, (s * (r - 1) + k + 1) % inner_ring_n);
                    add(inner0, outer1, inner1);
                }
            }
        }
    }

    shapes
}

