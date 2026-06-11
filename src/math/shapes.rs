use crate::{components::transform::Transform, math::vec2::{Vec2, project_points}};

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        Self {radius:r}
    }
}

#[derive(Clone,serde::Deserialize)]
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

    pub fn project_polygon(self, axis: Vec2) -> (f32,f32) {
        project_points(&self.points, axis)
    }

    pub fn clip_polygon_left(
        points: Vec<Vec2>,
        min_x:f32,
    )

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

        //pg1 is the topleft ploygon
        let mut pg1:Vec<Vec2> = vec![];

        //pg2 is the topright polygon
        let mut pg2:Vec<Vec2> = vec![];

        //pg3 is the bottomleft ploygon
        let mut pg3:Vec<Vec2> = vec![];

        //pg4 is the bottomright polygon
        let mut pg4:Vec<Vec2> = vec![];

        for point in &self.points {
            if point.x <= 0.0 && point.y <= 0.0 {
                pg1.push(*point);
            }
            if point.x >= 0.0 && point.y <= 0.0 {
                pg2.push(*point);
            }
            if point.x <= 0.0 && point.y >= 0.0 {
                pg3.push(*point);
            }
            if point.x >= 0.0 && point.y >= 0.0 {
                pg4.push(*point);
            }
        }

        let pg1:Polygon = Polygon::new(pg1);
        let pg2:Polygon = Polygon::new(pg2);
        let pg3:Polygon = Polygon::new(pg3);
        let pg4:Polygon = Polygon::new(pg4);
        return [pg1,pg2,pg3,pg4];
    }
}
