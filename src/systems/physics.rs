use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::math::shapes::Polygon;
use crate::math::vec2::{cross, perpendicular};
use crate::entity::Entity;

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
