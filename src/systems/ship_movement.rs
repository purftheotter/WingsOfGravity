use rapier2d::math::*;

use crate::components::ShipStats;
use crate::entity::Entity;
use crate::physics_world::PhysicsWorld;


pub fn apply_ship_movements(
    physics_world: &mut PhysicsWorld,
    entity: &Entity,
    ship_stats: &ShipStats,
    ){
    let ship = entity.ship_component.as_ref().unwrap();
    let rigid_body = physics_world.rigid_body_set
        .get_mut(entity.rigid_body_handle)
        .unwrap();

    rigid_body.reset_forces(true);
    rigid_body.reset_torques(true);

    let local_forward = Vec2::new(0.0, -ship_stats.thrust);
    let world_impulse = rigid_body.rotation()
        .transform_vector(local_forward);

    let torque = ship_stats.torque;

    rigid_body.add_force(world_impulse * ship.input.thrust, true);
    
    rigid_body.add_torque(torque * ship.input.turn, true);


}
