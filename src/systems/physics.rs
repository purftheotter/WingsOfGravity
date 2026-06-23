use sdl2::hint::Hint::Normal;

use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::entity::Entity;
use crate::systems::collision::Collision;
use crate::assets::ship_database::ShipDatabase;

const RESTITUTION:f32 = 0.2;


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

pub fn resolve_collisoin(
    collision: &Collision,
    entity1:&mut Entity,
    entity2:&mut Entity,
) {

    resolve_linear_impulse(
        collision,
        entity1,
        entity2,
    );

}

pub fn resolve_linear_impulse(
    collision: &Collision,
    entity1:&mut Entity,
    entity2:&mut Entity,
){
 
    let normal = collision.normal.normalize();


    let relative_velocity = 
        entity2.rigidbody.velocity.linear 
        - entity1.rigidbody.velocity.linear;

    let velocity_along_normal = 
        relative_velocity.dot(normal);
    if velocity_along_normal < 0.0 {
        

        let impulse_magnitude =
            -(1.0 + RESTITUTION) * velocity_along_normal
            / (
                1.0 /entity2.rigidbody.mass
                + 1.0 /entity1.rigidbody.mass
            );

        let impulse = normal * impulse_magnitude;

        entity2.rigidbody.velocity.linear += impulse 
            / entity2.rigidbody.mass;

        entity1.rigidbody.velocity.linear -=
            impulse / entity1.rigidbody.mass;

    }

   
}

pub fn correct_position_ship_astroid(
    collision: &Collision,
    asteroid_entity:&mut Entity,
    ship:&mut Entity,

) { 
    let normal = collision.normal.normalize();
    let depth = collision.depth;

    let mtv = normal * depth;
    let total_mass = 
        ship.rigidbody.mass + asteroid_entity.rigidbody.mass;
    ship.transform.position += 
        mtv * (asteroid_entity.rigidbody.mass / total_mass);
    asteroid_entity.transform.position -= 
        mtv * (ship.rigidbody.mass / total_mass);

}
