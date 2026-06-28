use crate::components::transform::Transform;
use crate::math::shapes::{Circle, Polygon};
use crate::math::vec2::{Vec2, project_points};
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

pub fn sat_collision(
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

        let (mina, maxa) = project_points(&wspg1.points, axis);
        let (minb, maxb) = project_points(&wspg2.points, axis);

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

        let (mina, maxa) = project_points(&wspg1.points, axis);
        let (minb, maxb) = project_points(&wspg2.points, axis);

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

    let points = find_contact_points(&wspg1, &wspg2, normal);

    return Some(Collision { normal, depth, points })
}

pub fn find_contact_points(pg1: &Polygon, pg2:&Polygon, normal: Vec2,) -> Vec<Vec2> {
    let (ref_edge, inc_verts) = 
        find_reference_edge_and_incident(pg1, pg2, normal);
    let mut points = clip_incident_to_reference(ref_edge, inc_verts);

    points = remove_duplicates(points);

    points
}

pub fn remove_duplicates(points: Vec<Vec2>) -> Vec<Vec2> {
    const DIST_THRESHOLD: f32 = 0.1;
    let mut result: Vec<Vec2> = Vec::new();

    for point in points {
        let is_duplicate = result
            .iter()
            .any(|&q| (point-q).length_sq() < DIST_THRESHOLD * DIST_THRESHOLD);
        if !is_duplicate {
            result.push(point);
        }
    }
    result
}

pub fn find_reference_edge_and_incident(
    pg1: &Polygon,
    pg2: &Polygon,
    normal: Vec2,
) -> ([Vec2; 2], Vec<Vec2>) {
    let ref_edge = best_edge(pg1, normal);

    let inc_edge = best_edge(pg2, -normal);

    (ref_edge, inc_edge.to_vec())
}

fn best_edge(pg: &Polygon, dir: Vec2) -> [Vec2; 2] {
    let mut best_dot = f32::NEG_INFINITY;
    let mut best_i = 0;
    for i in 0..pg.points.len() {
        let va = pg.points[i];
        let vb = pg.points[(i + 1) % pg.points.len()];
        let edge_normal = (vb - va).normal().normalize();
        let d = edge_normal.dot(dir);
        if d > best_dot {
            best_dot = d;
            best_i = i;
        }
    }
    [pg.points[best_i], pg.points[(best_i + 1) % pg.points.len()]]
}

fn clip_incident_to_reference(ref_edge: [Vec2; 2], incident: Vec<Vec2>) -> Vec<Vec2> {
    let ref_dir = (ref_edge[1] - ref_edge[0]).normalize();
    let ref_normal = -ref_dir.normal();
 
    // Clip against the two side planes (perpendicular to the edge at each endpoint)
    let output = clip_to_halfplane(incident, ref_edge[0], ref_dir);
    let output = clip_to_halfplane(output, ref_edge[1], -ref_dir);
 
    // Keep only points on the penetrating side of the reference face
    output
        .into_iter()
        .filter(|&p| (p - ref_edge[0]).dot(ref_normal) <= 0.0)
        .collect()
}

fn clip_to_halfplane(points: Vec<Vec2>, plane_point: Vec2, plane_normal: Vec2) -> Vec<Vec2> {
    let mut output = Vec::new();
    let n = points.len();
    if n < 2 {
        return output;
    }

    for i in 0..n -1{
        let a = points[i];
        let b = points[i + 1];
        let da = (a - plane_point).dot(plane_normal);
        let db = (b - plane_point).dot(plane_normal);
        if da >= 0.0 {
            output.push(a);
        }
        if (da >= 0.0) != (db >= 0.0) {
            let t = da / (da - db);
            output.push(a + (b - a) * t);
        }
    }

    if let Some(&last) = points.last() {
        let d = (last - plane_point).dot(plane_normal);
        if d >= 0.0 {
            output.push(last);
        }
    }
    output
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


