use crate::components::asteroid;
use crate::components::transform::Transform;
use crate::components::velocity::Velocity;
use crate::entity::Entity;
use crate::systems::collision::Collision;
use crate::assets::ship_database::ShipDatabase;

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

pub fn resolve_ship_asteriod_collisoin(
    collision: &Collision,
    asteroid_entity:&mut Entity,
    ship:&mut Entity,
    ship_database: &ShipDatabase,
) {

    let normal = collision.normal.normalize();

    let asteroid = asteroid_entity.asteroid.as_ref().unwrap();

    let ship_component = ship.ship_component.as_ref().unwrap();

    let ship_component_stats = ship_database.get(ship_component.class);

    let restitution = 0.2;

    let relative_velocity = 
        ship.velocity.linear 
        - asteroid_entity.velocity.linear;

    let velocity_along_normal = 
        relative_velocity.dot(normal);
    if velocity_along_normal < 0.0 {
        

        let impulse_magnitude =
            -(1.0 + restitution) * velocity_along_normal
            / (
                1.0 /ship_component_stats.mass 
                + 1.0 /asteroid.root.mass
            );

        let impulse = normal * impulse_magnitude;

        ship.velocity.linear += impulse 
            / ship_component_stats.mass;

        asteroid_entity.velocity.linear -=
            impulse / asteroid.root.mass;

    }

}

pub fn correct_position_ship_astroid(
    collision: &Collision,
    asteroid_entity:&mut Entity,
    ship:&mut Entity,
    ship_database: &ShipDatabase,

) { 
    let normal = collision.normal.normalize();
    let depth = collision.depth;

    let asteroid = asteroid_entity.asteroid.as_ref().unwrap();

    let ship_component = ship.ship_component.as_ref().unwrap();

    let ship_component_stats = ship_database.get(ship_component.class);



    let mtv = normal * depth;
    let total_mass = 
        ship_component_stats.mass + asteroid.root.mass;
    ship.transform.position += 
        mtv * (asteroid.root.mass / total_mass);
    asteroid_entity.transform.position -= 
        mtv * (ship_component_stats.mass / total_mass);

}
