use crate::components::transform::Transform;
use crate::math::vec2::Vec2; 
use crate::math::vec2::project_points;
use crate::math::vec2::clip_x;
use crate::math::vec2::clip_y;

const EPS: f32 = 1e-6;

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        Self {radius:r}
    }
}

#[derive(Clone, serde::Deserialize)]
pub struct Polygon {
    pub points: Vec<Vec2>,
}

impl Polygon {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Vec2> {
        self.points.iter()
    }

    pub fn apply_transformation(
        &self,
        transform: &Transform,
    ) -> Polygon {
        let rotation = transform.rotation.to_radians();
        let (sin,cos) = rotation.sin_cos();

        let points = self.points
            .iter()
            .map(|p| {

                let x = p.x * transform.scale.x;
                let y = p.y * transform.scale.y;


                let rotated_x = x * cos - y * sin;
                let rotated_y = x * sin + y * cos;
                Vec2::new(
                    rotated_x,
                    rotated_y,
                ) + transform.position
            })
            .collect();

        Polygon::new(points)

    }

    pub fn project_polygon(&self, axis: Vec2) -> (f32,f32) {
        project_points(&self.points, axis)
    }

    

    pub fn subdivide4(&self) -> [Polygon;4] {
        let (min_x,max_x) = project_points(
            &self.points,
            Vec2::new(1.0, 0.0)
        );
        let (min_y,max_y) = project_points(
            &self.points,
            Vec2::new(0.0, 1.0)
        );

        let center = Vec2::new(
            (min_x + max_x) / 2.0, (min_y+ max_y) / 2.0 );

        let left = clip_polygon_left(&self.points, center.x);
        let right = clip_polygon_right(&self.points, center.x);

        //top_left 
        let mut top_left = clip_polygon_bottom(&right, center.y);
        insert_center(&mut top_left, &center);
        let top_left_polygon = Polygon::new(top_left);

        //top_right 
        let mut top_right = clip_polygon_bottom(&left, center.y);
        insert_center(&mut top_right, &center);
        let top_right_polygon = Polygon::new(top_right);

        //bottom_left 
        let mut bottom_left = clip_polygon_top(&right, center.y);
        insert_center(&mut bottom_left, &center);
        let bottom_left_polygon = Polygon::new(bottom_left);

        //bottom_right 
        let mut bottom_right = clip_polygon_top(&left, center.y);
        insert_center(&mut bottom_right, &center);
        let bottom_right_polygon = Polygon::new(bottom_right);

        for point in top_left_polygon.iter() {
            println!("x:{}, y:{}",point.x,point.y);
        }


        [
            top_left_polygon,
            top_right_polygon,
            bottom_left_polygon,
            bottom_right_polygon,
        ]

    }
}


pub fn clip_polygon_left(
    points: &[Vec2],
    min_x: f32,
) -> Vec<Vec2> {
    let mut output = Vec::new();

    for (p1,p2) in points.iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
        {

        let p1_inside = p1.x >= min_x;
        let p2_inside = p2.x >= min_x;

        match (p1_inside,p2_inside) {
            (true, true) => {
                output.push(*p2);
            }
            (true,false) => {
                if let Some(intersection) = clip_x(p1, p2, min_x) {
                    output.push(intersection);
                }
            }
            (false,true) => {
                if let Some(intersection) = clip_x(p1, p2, min_x) {
                    output.push(intersection);
                }
                output.push(*p2);
            }
            (false, false) => {

            }
            
        }

    }

    output
}


pub fn clip_polygon_right(
    points: &[Vec2],
    max_x: f32,
) -> Vec<Vec2> {
    let mut output = Vec::new();

    for (p1,p2) in points.iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
        {
        let p1_inside = p1.x <= max_x;
        let p2_inside = p2.x <= max_x;

        match (p1_inside,p2_inside) {
            (true, true) => {
                output.push(*p2);
            }
            (true,false) => {
                if let Some(intersection) = clip_x(p1, p2, max_x) {
                    output.push(intersection);
                }
            }
            (false,true) => {
                if let Some(intersection) = clip_x(p1, p2, max_x) {
                    output.push(intersection);
                }
                output.push(*p2);
            }
            (false, false) => {

            }
            
        }

    }

    output
}


pub fn clip_polygon_top(
    points: &[Vec2],
    min_y: f32,
) -> Vec<Vec2> {
    let mut output = Vec::new();

    for (p1,p2) in points.iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
    {
        let p1_inside = p1.y >= min_y;
        let p2_inside = p2.y >= min_y;

        match (p1_inside,p2_inside) {
            (true, true) => {
                output.push(*p2);
            }
            (true,false) => {
                if let Some(intersection) = clip_y(p1, p2, min_y) {
                    output.push(intersection);
                }
            }
            (false,true) => {
                if let Some(intersection) = clip_y(p1, p2, min_y) {
                    output.push(intersection);
                }
                output.push(*p2);

            }
            (false, false) => {

            }
            
        }

    }

    output
}


pub fn clip_polygon_bottom(
    points: &[Vec2],
    max_y: f32,
) -> Vec<Vec2> {
    let mut output = Vec::new();

    for (p1,p2) in points.iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
    {
        let p1_inside = p1.y <= max_y;
        let p2_inside = p2.y <= max_y;

        match (p1_inside,p2_inside) {
            (true, true) => {
                output.push(*p2);
            }
            (true,false) => {
                if let Some(intersection) = clip_y(p1, p2, max_y) {
                    output.push(intersection);
                }
            }
            (false,true) => {
                if let Some(intersection) = clip_y(p1, p2, max_y) {
                    output.push(intersection);
                }
                output.push(*p2);

            }
            (false, false) => {

            }
            
        }

    }

    output
}

fn insert_center(points: &mut Vec<Vec2>, center: &Vec2) {
    if points.contains(center) {
        return
    }

    let mut vx = None;
    let mut hy = None;

    for (i,p) in points.iter().enumerate() {
        if is_vertical(*p, center.x) {
            vx = Some(i);
        }
        if is_horizontal(*p, center.y) {
            hy = Some(i);
        }
    }

    if let (Some(a),Some(b)) = (vx, hy) {
        let insert_pos = a.min(b) + 1;
        points.insert(insert_pos, *center);
    }
}

fn is_vertical(p: Vec2, cx: f32) -> bool {
    (p.x - cx).abs() < EPS
}

fn is_horizontal(p: Vec2, cy: f32) -> bool {
    (p.y - cy).abs() < EPS
}
