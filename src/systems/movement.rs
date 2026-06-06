use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::math::vec2math::Vec2;

pub fn apply_velocity(transform: &mut Transform, velocity: &Velocity, dt: f32) {
    transform.position += velocity.linear * dt;
    transform.rotation = (transform.rotation + velocity.angular * dt) % 360.0;
}

pub fn apply_acceleration(velocity: &mut Velocity, rotation: &f32, acceleration: &f32, dt: f32) {
    
    let dir = rotation.to_radians();
    let force = Vec2::new(dir.sin(), -dir.cos()) * *acceleration * dt;

    velocity.linear += force;
}
