use crate::components::transform::Transform;
use crate::components::velocity::Velocity;

pub fn apply_forward_thrust(
    velocity: &mut Velocity,
    rotation: &f32,
    force: &f32,
    mass: &f32,
    dt: &f32,
)
{
    let acceleration = force / mass;

    velocity.linear.x += acceleration * rotation.to_radians().sin() * dt;
    velocity.linear.y -= acceleration * rotation.to_radians().cos() * dt;
}

pub fn apply_torque(
    velocity: &mut Velocity,
    torque: &f32,
    moment_of_inertia: &f32,
    dt: &f32,
)
{
    velocity.angular += torque / moment_of_inertia * dt;
}

pub fn apply_velocity(transform: &mut Transform, velocity: &Velocity, dt: &f32) {
    transform.position += velocity.linear * *dt;
    transform.rotation = (transform.rotation + velocity.angular * dt) % 360.0;
}
