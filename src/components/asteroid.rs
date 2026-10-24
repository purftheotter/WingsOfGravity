use noise::{Fbm, NoiseFn, Perlin};
use rapier2d::dynamics::{RigidBodyHandle, RigidBodySet};
use rapier2d::geometry::*;
use rapier2d::math::*;

use std::collections::HashMap;
use std::f32::consts::PI;

use crate::core::*;
use crate::physics_world::*;
use crate::rendering::pixel_data::PixelData;

#[derive(Debug)]
pub struct Asteroid {
    pub hull_handle: ColliderHandle,
    pub mask: AsteroidMask,
    pub terrain_verts: Vec<AsteroidVert>,
    pub tris: Vec<[usize; 3]>,
    pub intact_tris: Vec<[Vec2; 3]>,
    pub rings: usize,
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
        texel: f32,
    ) -> Self {
        let mut noise = Fbm::<Perlin>::new(seed);
        noise.octaves = 4;
        noise.frequency = 0.5;
        noise.lacunarity = 2.0;
        noise.persistence = 0.5;

        let terrain_verts = Self::make_asteroid_verts(
            collider_set,
            rigid_body_set,
            rigid_body_handle,
            subdivisions,
            radius,
            &noise,
            noise_strength,
        );

        let index_map: HashMap<PolarIndex, usize> = terrain_verts
            .iter()
            .enumerate()
            .map(|(pos, vert)| (vert.index, pos))
            .collect();

        let tris = build_lattice_tris(subdivisions, &index_map);
        let intact_tris = intact_triangles(&tris, &terrain_verts);

        let hull_collider = build_collider(&intact_tris)
            .expect("asteroid had no triangles: need subdivisions >= 2");
        let hull_handle =
            collider_set.insert_with_parent(hull_collider, *rigid_body_handle, rigid_body_set);

        let extent = terrain_verts
            .iter()
            .map(|v| v.local_transform.length())
            .fold(0.0_f32, f32::max);

        let mut mask = AsteroidMask::new(extent, texel);
        mask.rebuild(&intact_tris);

        Self {
            hull_handle,
            mask,
            terrain_verts,
            tris,
            intact_tris,
            rings: subdivisions,
        }
    }

    fn make_asteroid_verts(
        collider_set: &mut ColliderSet,
        rigid_body_set: &mut RigidBodySet,
        rigid_body_handle: &RigidBodyHandle,
        subdivisions: usize,
        base_radius: f32,
        noise: &Fbm<Perlin>,
        noise_strength: f32,
    ) -> Vec<AsteroidVert> {
        let vert_radius = base_radius / subdivisions as f32;
        let mut verts = Vec::new();

        let mut add = |position: Vec2, index: PolarIndex| {
            verts.push(AsteroidVert::new(
                collider_set,
                rigid_body_set,
                rigid_body_handle,
                position,
                index,
                vert_radius,
            ));
        };

        add(Vec2::ZERO, PolarIndex::new(0, 0));

        for ring in 1..subdivisions {
            let ring_radius = 2.0 * ring as f32 * vert_radius;
            let segments = 6 * ring;

            for point in 0..segments {
                let angle = 2.0 * PI * point as f32 / segments as f32;
                let displacement = Self::radius_displacement(noise, angle, noise_strength);
                let radius = ring_radius * (1.0 + displacement);

                add(
                    Vec2::new(angle.cos() * radius, angle.sin() * radius),
                    PolarIndex::new(ring, point),
                );
            }
        }

        verts
    }

    pub fn radius_displacement(noise: &Fbm<Perlin>, angle: f32, wobble_strength: f32) -> f32 {
        let raw = noise.get([angle.cos() as f64, angle.sin() as f64]);
        raw as f32 * wobble_strength
    }

    fn rebuild_collider(
        &mut self,
        rigid_body_handle: RigidBodyHandle,
        physics_world: &mut PhysicsWorld,
    ) -> bool {
        let Some(new_collider) = build_collider(&self.intact_tris) else {
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
            &mut physics_world.rigid_body_set,
        );

        self.mask.rebuild(&self.intact_tris);

        if let Some(body) = physics_world.rigid_body_set.get_mut(rigid_body_handle) {
            body.recompute_mass_properties_from_colliders(&physics_world.collider_set);
        }
        true
    }

    pub fn update_asteroid(
        &mut self,
        rigid_body_handle: RigidBodyHandle,
        physics_world: &mut PhysicsWorld,
    ) -> bool {
        let mut verts_changed = false;
        for vert in self.terrain_verts.iter_mut() {
            verts_changed |=
                vert.update_vert(&physics_world.narrow_phase, &mut physics_world.collider_set);
        }

        if !verts_changed {
            return false;
        }

        self.intact_tris = intact_triangles(&self.tris, &self.terrain_verts);
        !self.rebuild_collider(rigid_body_handle, physics_world)
    }
}

#[derive(Debug)]
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
    ) -> Self {
        let sensor = ColliderBuilder::ball(sensor_radius)
            .sensor(true)
            .translation(local_transform)
            .collision_groups(InteractionGroups::new(
                ASTEROID_VERT,
                BULLET,
                InteractionTestMode::And,
            ))
            .density(0.0)
            .build();
        let sensor_handle =
            collider_set.insert_with_parent(sensor, *rigid_body_handle, rigid_body_set);

        Self {
            local_transform,
            index,
            sensor_handle,
            destroyed: false,
        }
    }

    pub fn update_vert(&mut self, narrow_phase: &NarrowPhase, collider_set: &mut ColliderSet) -> bool {
        if self.destroyed {
            return false;
        }

        let hit = narrow_phase
            .intersection_pairs_with(self.sensor_handle)
            .any(|(_, _, intersecting)| intersecting);

        if hit {
            self.destroyed = true;
            if let Some(sensor) = collider_set.get_mut(self.sensor_handle) {
                sensor.set_enabled(false);
            }
        }
        hit
    }
}

#[derive(Debug)]
pub struct AsteroidMask {
    pub size: usize,
    pub texel: f32,
    alpha: Vec<u8>,
    pub dirty: bool,
}

impl AsteroidMask {
    pub fn new(max_extent: f32, texel: f32) -> Self {
        let size = ((2.0 * max_extent) / texel).ceil() as usize + 2;
        Self {
            size,
            texel,
            alpha: vec![0; size * size],
            dirty: true,
        }
    }

    fn to_texel(&self, p: Vec2) -> Vec2 {
        p / self.texel + Vec2::splat(self.size as f32 * 0.5)
    }

    pub fn rebuild(&mut self, triangles: &[[Vec2; 3]]) {
        self.alpha.fill(0);
        let max = Vec2::splat((self.size - 1) as f32);

        for &[a, b, c] in triangles {
            let (a, b, c) = (self.to_texel(a), self.to_texel(b), self.to_texel(c));

            let lo = a.min(b).min(c).floor().max(Vec2::ZERO);
            let hi = a.max(b).max(c).ceil().min(max);

            for y in lo.y as usize..=hi.y as usize {
                for x in lo.x as usize..=hi.x as usize {
                    let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                    if point_in_triangle(p, a, b, c) {
                        self.alpha[y * self.size + x] = 255;
                    }
                }
            }
        }
        self.dirty = true;
    }

    pub fn to_rgba(&self, rock: &PixelData) -> Vec<u8> {
        let mut out = vec![0u8; self.size * self.size * 4];
        for y in 0..self.size {
            for x in 0..self.size {
                let i = y * self.size + x;
                if self.alpha[i] == 0 {
                    continue;
                }
                let r = ((y % rock.h) * rock.w + (x % rock.w)) * 4;
                out[i * 4..i * 4 + 3].copy_from_slice(&rock.rgba[r..r + 3]);
                out[i * 4 + 3] = 255;
            }
        }
        out
    }
}

pub fn build_collider(triangles: &[[Vec2; 3]]) -> Option<Collider> {
    if triangles.is_empty() {
        return None;
    }
    let shapes: Vec<(Pose, SharedShape)> = triangles
        .iter()
        .map(|&[a, b, c]| (Pose::IDENTITY, SharedShape::triangle(a, b, c)))
        .collect();
    Some(ColliderBuilder::compound(shapes).restitution(0.2).build())
}

pub fn build_lattice_tris(rings: usize, index_map: &HashMap<PolarIndex, usize>) -> Vec<[usize; 3]> {
    let vert = |ring: usize, point: usize| index_map[&PolarIndex::new(ring, point)];

    let mut tris = Vec::new();

    for r in 1..rings {
        let outer_n = 6 * r;
        let inner_n = if r == 1 { 1 } else { 6 * (r - 1) };

        for s in 0..6 {
            for k in 0..r {
                let outer0 = vert(r, (s * r + k) % outer_n);
                let outer1 = vert(r, (s * r + k + 1) % outer_n);
                let inner0 = vert(r - 1, (s * (r - 1) + k) % inner_n);
                tris.push([outer0, outer1, inner0]);

                if k < r - 1 {
                    let inner1 = vert(r - 1, (s * (r - 1) + k + 1) % inner_n);
                    tris.push([inner0, outer1, inner1]);
                }
            }
        }
    }
    tris
}

pub fn intact_triangles(tris: &[[usize; 3]], verts: &[AsteroidVert]) -> Vec<[Vec2; 3]> {
    tris.iter()
        .filter(|tri| tri.iter().all(|&i| !verts[i].destroyed))
        .map(|tri| tri.map(|i| verts[i].local_transform))
        .collect()
}
