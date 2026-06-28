use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::math::shapes::Polygon;
use crate::math::vec2::{cross, perpendicular};
use crate::entity::Entity;
use crate::systems::collision::Collision;

const RESTITUTION:f32 = 0.2;

const RESTITUTION_THRESHOLD: f32 = 2.0;

pub fn apply_forward_thrust(
    velocity: &mut Velocity,
    rotation: &f32,
    force: &f32,
    mass: &f32,
    dt: &f32,
)
{
    let acceleration = force / mass;

    velocity.linear.x += acceleration * rotation.sin() * dt;
    velocity.linear.y -= acceleration * rotation.cos() * dt;
}

pub fn apply_torque(
    velocity: &mut Velocity,
    torque: &f32,
    moment_of_inertia: &f32,
    dt: &f32,
)
{
    velocity.angular += torque.to_radians() / moment_of_inertia * dt;
}

pub fn apply_velocity(transform: &mut Transform, velocity: &Velocity, dt: &f32) {
    transform.position += velocity.linear * *dt;
    transform.rotation = 
        (transform.rotation + velocity.angular * dt) % (2.0 * std::f32::consts::PI);
}

pub fn resolve_impulse(collision: &Collision, entity1:&mut Entity, entity2:&mut Entity) {
    let normal = collision.normal.normalize();

    let point = match collision.points.first() {
        Some(&p) => p,
        None => return,
    };
    
    let omeg1 = entity1.rigidbody.velocity.angular;

    let omeg2 = entity2.rigidbody.velocity.angular;

    let r1 = 
        point - entity1.transform.position;

    let r2 =
        point - entity2.transform.position;

    let v1 = entity1.rigidbody.velocity.linear + perpendicular(r1) * omeg1;
    let v2 = entity2.rigidbody.velocity.linear + perpendicular(r2) * omeg2;

    let relative_velocity = v2 - v1;

    let velocity_along_normal = 
        relative_velocity.dot(normal);

    if velocity_along_normal > 0.0 {
        return;
    }

    let rn1 = cross(r1, normal);

    let rn2 = cross(r2, normal);

    let inv_mass1 = 1.0 / entity1.rigidbody.mass;
    let inv_mass2 = 1.0 / entity2.rigidbody.mass;

    let demon =
        (inv_mass1 + inv_mass2)
        + (rn1 * rn1) / entity1.rigidbody.inertia
        + (rn2 * rn2) / entity2.rigidbody.inertia;

    let effect_restitution = if velocity_along_normal.abs() < RESTITUTION_THRESHOLD {
        0.0
    }else {
        RESTITUTION
    };

    let j =
        (-(1.0 + effect_restitution) * velocity_along_normal /demon).max(0.0);

    let impulse = normal * j;

    //apply_linear_velocity
    entity1.rigidbody.velocity.linear -=
        impulse / entity1.rigidbody.mass;

    entity2.rigidbody.velocity.linear +=
        impulse / entity2.rigidbody.mass;
    
    //apply_angular_velocity
    entity1.rigidbody.velocity.angular -= cross(r1, impulse)
        / entity1.rigidbody.inertia;

    entity2.rigidbody.velocity.angular += cross(r2, impulse)
        / entity2.rigidbody.inertia;
    
}

pub fn correct_position(
    collision: &Collision,
    entity1:&mut Entity,
    entity2:&mut Entity,
) { 

    const BAUMGARTE: f32 = 0.4;
    
    const SLOP: f32 = 1.0;

    let correction_depth = (collision.depth - SLOP).max(0.0);

    if correction_depth == 0.0 {
        return;
    }

    let normal = collision.normal.normalize();

    let inv_mass1 = 1.0 / entity1.rigidbody.mass;
    let inv_mass2 = 1.0 / entity2.rigidbody.mass;
    let total_inv_mass = inv_mass1 + inv_mass2;

    let correction_mag = BAUMGARTE * correction_depth / total_inv_mass;
    let correction = normal* correction_mag;

    entity1.transform.position -= correction * inv_mass1;
    entity2.transform.position += correction * inv_mass2;

    /*
    let point = match collision.points.first() {
        Some(&p) => p,
        None => return,
    };

    let r1 = point - entity1.transform.position;
    let r2 = point - entity2.transform.position;

    entity1.transform.rotation -= cross(r1, correction) * inv_mass1 / entity1.rigidbody.inertia;
    entity2.transform.rotation += cross(r2, correction) * inv_mass2 / entity2.rigidbody.inertia;
    */
}

pub fn polygon_inertia(mass: &f32, polygon: &Polygon) -> f32 {
    let mut numerator:f32 = 0.0;
    let mut demoninator:f32 = 0.0;

    for i in 0..polygon.points.len() {
        let point1 = polygon.points[i];
        let point2 = polygon.points[(i+1) % polygon.points.len()];

        let cross = cross(point1, point2).abs();

        numerator += cross * (point1.dot(point1) + point1.dot(point2) + point2.dot(point2));

        demoninator += cross;
    }

    if demoninator == 0.0 {
        return 0.0;
    }
    (mass / 6.0) * (numerator / demoninator)
}
