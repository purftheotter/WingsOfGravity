use crate::components::velocity::Velocity;

pub struct RigidBody {
    pub mass:f32,
    pub inertia:f32,
    pub velocity:Velocity,
}
