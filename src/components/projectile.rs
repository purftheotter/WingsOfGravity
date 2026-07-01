use crate::components::hitbox::Hitbox;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProjectileType {
    Bullet,
    Missle,
    Laser,
}

pub struct Projectile {
    pub projectile_type: ProjectileType,
    pub damage: f32,
    pub hitbox: Hitbox,
}
