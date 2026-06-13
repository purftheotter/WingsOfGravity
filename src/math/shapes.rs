use crate::components::transform::Transform;
use crate::math::vec2::Vec2; 
use crate::math::vec2::project_points;
use crate::math::vec2::clip_x;
use crate::math::vec2::clip_y;

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
        let (min_y,max_y) = project_points
            (&self.points,
            Vec2::new(0.0, 1.0)
        );

        let center = Vec2::new(
            (min_x + max_x) / 2.0, (min_y+ max_y) / 2.0 );

        let left = 
            Polygon::new(clip_polygon_left(&self.points, center.x));
        let right = 
            Polygon::new(clip_polygon_right(&self.points, center.x));

        let top_left = 
            Polygon::new(clip_polygon_top(&left.points, center.y));
        let top_right = 
            Polygon::new(clip_polygon_top(&right.points, center.y));
        let bottom_left = 
            Polygon::new(clip_polygon_bottom(&left.points, center.y));
        let bottom_right = 
            Polygon::new(clip_polygon_bottom(&right.points, center.y));
    
        [
            top_left,
            top_right,
            bottom_left,
            bottom_right,
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
