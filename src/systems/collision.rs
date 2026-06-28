use crate::components::transform::Transform;
use crate::math::shapes::circle::project_circle;
use crate::math::shapes::{Circle, Polygon};
use crate::math::shapes::polygon::{
    find_cloesest_point,
    project_polygon,
    find_reference_edge_and_incident, 
    clip_incident_to_reference,
    remove_duplicates,
};
use crate::math::vec2::Vec2;
use crate::entity::Entity;
use crate::entity::EntityType;
use crate::assets::ship_database::ShipDatabase;
use crate::components::hitbox::Hitbox;
use crate::systems::physics::{correct_position, resolve_impulse};

pub struct Collision {
    pub normal: Vec2,
    pub depth: f32,
    pub points: Vec<Vec2>,
}

pub fn sat_collision_pg_pg(
    t1: &Transform,
    pg1: &Polygon,
    t2: &Transform,
    pg2: &Polygon,
    ) -> Option<Collision>{

    let wspg1 = pg1.apply_transformation(t1);
    let wspg2 = pg2.apply_transformation(t2);

    let mut normal = Vec2::new(0.0, 0.0);
    let mut depth = f32::MAX;

    // Polygon 1 axes
    for i in 0..wspg1.points.len() {
        let va = wspg1.points[i];
        let vb = wspg1.points[(i + 1) % wspg1.points.len()];

        let axis = (va - vb).normal().normalize();

        let (mina, maxa) = project_polygon(&wspg1, axis);
        let (minb, maxb) = project_polygon(&wspg2, axis);

        if maxa < minb || maxb < mina {
            return None;
        }

        let axis_depth = f32::min(maxb - mina,maxa - minb);

        if axis_depth < depth {
            depth = axis_depth;
            normal = axis;
        }
    }

    //Polygon 2 axes
    for i in 0..wspg2.points.len() {
        let va = wspg2.points[i];
        let vb = wspg2.points[(i + 1) % wspg2.points.len()];

        let axis = (vb-va).normal().normalize();

        let (mina, maxa) = project_polygon(&wspg1, axis);
        let (minb, maxb) = project_polygon(&wspg2, axis);

        if maxa < minb || maxb < mina {
            return None;
        }

        let axis_depth = f32::min(maxb - mina,maxa - minb);

        if axis_depth < depth {
            depth = axis_depth;
            normal = axis;
        }
    }

    if normal.dot(t2.position - t1.position) < 0.0 {
        normal = -normal;
    }

    let points = find_contact_points_pg_pg(&wspg1, &wspg2, normal);

    return Some(Collision { normal, depth, points })
}

pub fn sat_collision_pg_cir(
    polygon: &Polygon,
    polygon_transform: &Transform,
    circle: &Circle,
    circle_transform: &Transform,
) ->Option<Collision> {
 
    let ws_polygon = polygon.apply_transformation(polygon_transform);

    let mut normal = Vec2::new(0.0, 0.0);
    let mut depth = f32::MAX;

    // Polygon axes
    for i in 0..ws_polygon.points.len() {
        let va = ws_polygon.points[i];
        let vb = ws_polygon.points[(i + 1) % ws_polygon.points.len()];

        let axis = (va - vb).normal().normalize();

        let (min_polygon, max_polygon) = project_polygon(&ws_polygon, axis);
        let (min_circle, max_circle) = project_circle(circle, circle_transform, axis);

        if max_polygon < min_circle || max_circle < min_polygon {
            return None;
        }

        let axis_depth = f32::min(max_circle - min_polygon, max_polygon - min_circle);

        if axis_depth < depth {
            depth = axis_depth;
            normal = axis;
        }
    }

    let closest_point_index = find_cloesest_point(circle_transform.position, &polygon);

    let closest_point = polygon.points[closest_point_index];

    let axis = (closest_point - circle_transform.position).normalize();

    let (min_polygon, max_polygon) = project_polygon(&ws_polygon, axis);
    let (min_circle, max_circle) = project_circle(circle, circle_transform, axis);

    if max_polygon < min_circle || max_circle < min_polygon {
        return None;
    }

    let axis_depth = f32::min(max_circle - min_polygon, max_polygon - min_circle);

    if axis_depth < depth {
        depth = axis_depth;
        normal = axis;
    }

    if normal.dot(polygon_transform.position - circle_transform.position) < 0.0 {
        normal = -normal;
    }

    let contact_point = circle_transform.position - normal * circle.radius;

    return Some(Collision { normal, depth, points: vec![contact_point] })
   
}

pub fn two_circle_collision(c1: &Circle,t1: &Transform, c2: &Circle, t2: &Transform) -> bool {
    let dx = t1.position.x - t2.position.x;
    let dy = t1.position.y - t2.position.y;

    let distance_sqaured = dx * dx + dy * dy;

    let radius_sum = c1.radius + c2.radius; 
    
    if distance_sqaured <= radius_sum * radius_sum{
        true
    }else {
        false
    }
}


pub fn update_player_collisions(
    entities: &mut Vec<Entity>,
    player_index: usize,
    ship_database: &ShipDatabase,
) {
    
        for i in 0..entities.len() {
 
            if i == player_index {
                continue;
            }
           

            let (mut player, mut asteroid) =
                if i > player_index {
                    let (left, right) = entities.split_at_mut(i);

                    (
                        &mut left[player_index],
                        &mut right[0],
                    )
                } else {
                    let (left, right) =
                        entities.split_at_mut(player_index);

                    (
                        &mut right[0],
                        &mut left[i],
                    )
                };

                if asteroid.entity_type != EntityType::Asteroid {
                    continue;
                }

                if let Some(collision) = ship_asteroid_collision(
                    &mut player,
                    &mut asteroid,
                    ship_database,
                ){

                    for &point in &collision.points {
                        let single = Collision {
                            normal: collision.normal,
                            depth: collision.depth,
                            points: vec![point],
                        };
                        resolve_impulse(&single, asteroid, player);
                    }
                    correct_position(&collision, asteroid, player);
                }

                

            }
}


pub fn ship_asteroid_collision(
    ship: &mut Entity,
    asteroid_entity: &mut Entity,
    ship_database: &ShipDatabase,
) -> Option<Collision> {

    let ship_component = ship
        .ship_component
        .as_ref()
        .unwrap();
    let ship_component_stats = ship_database.get(ship_component.class);

    let ship_hitbox = match &ship_component_stats.hitbox {Hitbox::Polygon { polygon } => polygon,_ => return None,};


    let asteroid = asteroid_entity.asteroid.as_ref().unwrap();

    asteroid.collides(
        ship_hitbox,
        &asteroid_entity.transform,
        &ship.transform,
    )
}

pub fn find_contact_points_pg_pg(pg1: &Polygon, pg2:&Polygon, normal: Vec2,) -> Vec<Vec2> {
    let (ref_edge, inc_verts) = 
        find_reference_edge_and_incident(pg1, pg2, normal);
    let mut points = clip_incident_to_reference(ref_edge, inc_verts);

    points = remove_duplicates(points);

    points
}

