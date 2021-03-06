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

    fn triangle_absolute() -> &'static str {
        "a   0.0 0.0
a   4.0 0.0
a   2.0 1.0"
    }

    fn triangle_relative() -> &'static str {
        "a   0.0 0.0
r   4.0 0.0
r   -2.0 1.0"
    }

    #[test]
    fn test_triangle_get_coordinates() {
        assert_eq!(
            get_coordinates(triangle_absolute()),
            vec![
                Point2D::new(0.0, 0.0),
                Point2D::new(4.0, 0.0),
                Point2D::new(2.0, 1.0),
            ]
        );
    }

    #[test]
    fn test_triangle_get_relative_coordinates() {
        assert_eq!(
            get_coordinates(triangle_relative()),
            vec![
                Point2D::new(0.0, 0.0),
                Point2D::new(4.0, 0.0),
                Point2D::new(2.0, 1.0),
            ]
        );
    }

    #[test]
    fn test_read_triangle_absolute() {
        assert_eq!(read_part(triangle_absolute()).area(), 2.0);
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
    let mut last_point: Point2D = Point2D::new(0.0, 0.0);
    for line in script.split('\n') {
        let first_char = line.chars().next().unwrap();
        let next_point = match first_char {
            ' ' | 'a' => read_line(line),
            'r' => read_line(line) + last_point,
            _ => {
                panic!("Invalid line"); //TODO: print offending line as part of panic message
            }
        };
        coordinates.push(next_point);
        last_point = next_point;
    }
    coordinates
}

fn read_part(snippet: &str) -> Polygon {
    return Polygon::new(get_coordinates(snippet));
}
