mod polygon;

use polygon::Point2D;
use polygon::Polygon;

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::get_coordinates;
    use super::polygon::Point2D;
    use super::read_line;
    use super::read_part;

    fn triangle_raw() -> &'static str {
        "a   0.0 0.0
r   4.0 0.0
r   -2.0    1.0"
    }

    #[test]
    fn test_triangle_get_coordinates() {
        let triangle_raw: &str = triangle_raw();
        assert_eq!(
            get_coordinates(triangle_raw),
            vec![
                Point2D::new(0.0, 0.0),
                Point2D::new(4.0, 0.0),
                Point2D::new(-2.0, 1.0),
            ]
        );
    }

    #[test]
    fn test_read_triangle_raw() {
        let triangle_raw: &str = triangle_raw();
        assert_eq!(read_part(triangle_raw).area(), 2.0);
    }

    #[test]
    fn test_read_line_absolute() {
        assert_eq!(read_line("a   0.0 0.0"), Point2D::new(0.0, 0.0));
    }

    #[test]
    fn test_read_line_negative_float() {
        assert_eq!(read_line("a   0.2 -5.6"), Point2D::new(0.2, -5.6));
    }

    #[test]
    fn test_read_line_int() {
        assert_eq!(read_line("a   1 -2"), Point2D::new(1.0, -2.0));
    }

    #[test]
    fn test_read_line_absolute_whitespace() {
        assert_eq!(read_line("a   0.0  0.0"), Point2D::new(0.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn test_invalid_line() {
        assert_eq!(read_line("a   0.0  "), Point2D::new(0.0, 0.0));
    }
}

fn read_line(line_text: &str) -> Point2D {
    let re = Regex::new(r"[ra] +(-*[0-9]*.[0-9]*) +(-*[0-9]*.[0-9]*)").unwrap();
    let captures = re.captures(line_text).unwrap();
    let x_coord: f32 = captures.get(1).unwrap().as_str().parse().unwrap();
    let y_coord: f32 = captures.get(2).unwrap().as_str().parse().unwrap();
    Point2D::new(x_coord, y_coord)
}

fn get_coordinates(script: &str) -> Vec<Point2D> {
    let mut coordinates: Vec<Point2D> = Vec::new();
    for line in script.split('\n') {
        coordinates.push(read_line(line));
    }
    coordinates
}

fn read_part(snippet: &str) -> Polygon {
    return Polygon::new(get_coordinates(snippet));
}
