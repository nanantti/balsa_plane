#[cfg(test)]
mod tests {
    use super::Point2D;
    use super::Polygon;

    #[test]
    fn point_init_floats() {
        let point = Point2D::new(0.0, 1.0);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 1.0);
    }

    #[test]
    fn point_init_string_float() {
        let point = Point2D::new(-0.3, 1.2);
        assert_eq!(point.x, -0.3);
        assert_eq!(point.y, 1.2);
    }

    #[test]
    fn point_equality() {
        let p1 = Point2D::new(0.0, 1.0);
        let p2 = Point2D::new(0.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);
        assert!(p1 == p2);
        assert!(p1 != p3);
    }

    fn simple_triangle() -> Polygon {
        Polygon::new(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(2.0, 0.0),
            Point2D::new(1.0, 1.0),
        ])
    }

    #[test]
    fn area_simple() {
        assert_eq!(simple_triangle().area(), 1.0);
    }

    #[test]
    fn centroid_simple() {
        let triang = simple_triangle();
        assert_eq!(triang.centroid().x, 1.0);
        assert_eq!(triang.centroid().y, 1.0 / 3.0);
    }

    #[test]
    fn centroid_square() {
        let triang = Polygon::new(vec![
            Point2D::new(-1.0, -1.0),
            Point2D::new(1.0, -1.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(-1.0, 1.0),
        ]);
        assert_eq!(triang.centroid().x, 0.0);
        assert_eq!(triang.centroid().y, 0.0);
    }

    #[test]
    fn polygon_equality() {
        let p1 = simple_triangle();
        let p2 = simple_triangle();
        let p3 = Polygon::new(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(2.0, 0.0),
            Point2D::new(1.0, 4.0),
        ]);
        assert!(p1 == p2);
        assert!(p1 != p3);
    }

    #[test]
    #[should_panic]
    fn zero_points() {
        let _zerang = Polygon::new(vec![]);
    }

    #[test]
    #[should_panic]
    fn one_point() {
        let _oneang = Polygon::new(vec![Point2D::new(0.0, 0.0)]);
    }

    #[test]
    #[should_panic]
    fn two_points() {
        let _twoang = Polygon::new(vec![Point2D::new(0.0, 0.0), Point2D::new(0.0, 0.0)]);
    }

    #[test]
    #[should_panic]
    fn area_zero() {
        let _zeroarea = Polygon::new(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(2.0, 0.0),
        ]);
    }
}

#[derive(Debug, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x_coord: f32, y_coord: f32) -> Point2D {
        Point2D {
            x: x_coord,
            y: y_coord,
        }
    }
}

#[derive(PartialEq)]
pub struct Polygon {
    point_list: Vec<Point2D>,
}

impl Polygon {
    pub fn new(points: Vec<Point2D>) -> Polygon {
        let poly = Polygon { point_list: points };
        poly.sanity_check();
        poly
    }

    pub fn area(&self) -> f32 {
        let mut area = 0.0;
        let num_points = self.point_list.len();
        let mut j = num_points - 1;

        for i in 0..num_points {
            area += (self.point_list[j].x + self.point_list[i].x)
                * (self.point_list[j].y - self.point_list[i].y);
            j = i;
        }
        area.abs() / 2.0
    }

    pub fn centroid(&self) -> Point2D {
        let mut cent = Point2D { x: 0.0, y: 0.0 };
        let mut count: i32 = 0;
        for point in &self.point_list {
            count += 1;
            cent.x += point.x;
            cent.y += point.y;
        }
        cent.x = cent.x / (count as f32);
        cent.y = cent.y / (count as f32);
        cent
    }

    fn sanity_check(&self) {
        if self.point_list.len() < 3 {
            panic!("Invalid number of points");
        }
        if self.area() <= 0.0 {
            panic!("Invalid area!")
        }
    }
}
