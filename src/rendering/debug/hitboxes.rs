use rapier2d::prelude::*;

use crate::render_context::RenderContext;
use crate::rendering::debug::*;

use crate::physics_world::PhysicsWorld;
use crate::entity::Entity;

pub fn render_hitbox(
    entity: &Entity,
    physics_world:&PhysicsWorld,
    render_context: &mut RenderContext,
) ->Result<(), String> {

    let entity_rigid_body = 
        physics_world.rigid_body_set
            .get(entity.rigid_body_handle)
            .unwrap();

    let entity_collider_handles:&[ColliderHandle] = 
        entity_rigid_body.colliders();

    let mut entity_colliders = vec![];

    for collider_handle in entity_collider_handles {
        entity_colliders
            .push(physics_world
                .collider_set
                .get(*collider_handle)
                .unwrap());
    }

    for collider in entity_colliders {
        let shape = collider.shape();
        
        match shape.as_typed_shape() {
            TypedShape::Ball(ball) => {
                render_ball(
                    render_context,
                    entity_rigid_body.position(),
                    ball
                )?
            }

            TypedShape::Triangle(tri) => {
                render_triangle(
                    render_context,
                    entity_rigid_body.position(),
                    tri,
                    )?
            }

            TypedShape::ConvexPolygon(polygon) => {
                render_convex_polygon(
                    render_context,
                    entity_rigid_body.position(),
                    polygon
                    )?
            }

            TypedShape::Compound(compound) =>{
                render_compound(
                    render_context,
                    entity_rigid_body.position(),
                    compound,
                )?
            }

            _ => {}
            
        }

        if let Some(asteroid) = entity.asteroid.as_ref() {
            for vert in asteroid.terrain_verts.iter() {
                if let Some(sensor) = 
                    physics_world.collider_set
                        .get(vert.sensor_handle)
                        .unwrap()
                        .shape()
                        .as_ball(){
                    render_ball(
                        render_context,
                        &entity_rigid_body.position().append_translation(vert.local_transform),
                        sensor
                    )?

                }

            }
        }
    }

    Ok(())
}


