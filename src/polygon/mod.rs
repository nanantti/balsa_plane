#[cfg(test)]
mod tests {
    use super::Point2D;
    use super::Polygon;

    #[test]
    fn test_point_init_string() {
        let point = Point2D::init_str("0,1");
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 1.0);
    }

    #[test]
    fn test_area_simple() {
        let triang = Polygon::init("0,0 2,0 1,1");
        assert_eq!(triang.area(), 1.0);
    }

    #[test]
    fn test_read_command() {
        let triang = Polygon::init("0,0 2,0 1,1");
        assert_eq!(triang.point_list[0].x, 0.0);
        assert_eq!(triang.point_list[0].y, 0.0);
        assert_eq!(triang.point_list[1].x, 2.0);
        assert_eq!(triang.point_list[1].y, 0.0);
        assert_eq!(triang.point_list[2].x, 1.0);
        assert_eq!(triang.point_list[2].y, 1.0);
    }
}

pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    pub fn init_str(cmd: &str) -> Point2D {
        let cmt_it: Vec<&str> = cmd.split(',').collect();
        let x_coord: f32 = cmt_it[0].parse().unwrap();
        let y_coord: f32 = cmt_it[1].parse().unwrap();
        Point2D {
            x: x_coord,
            y: y_coord,
        }
    }
}

pub struct Polygon {
    point_list: Vec<Point2D>,
}

impl Polygon {
    pub fn init(command: &str) -> Polygon {
        let cmd_it = command.split(' ');
        let mut points = Vec::new();
        for s in cmd_it {
            points.push(Point2D::init_str(s));
        }
        Polygon {
            point_list: points,
        }
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
}
