use crate::components::transform::Transform;
use crate::components::velocity::Velocity;

pub fn apply_velocity(transform: &mut Transform, velocity: &Velocity, dt: f32) {
    transform.x += velocity.x * dt;
    transform.y += velocity.y * dt;
}

pub fn rotated_velocity(velocity: &mut Velocity, rotation: &f32, distance: f32, dt: f32) {
    velocity.x += distance * rotation.to_radians().sin() * dt;
    velocity.y -= distance * rotation.to_radians().cos() * dt;
}
