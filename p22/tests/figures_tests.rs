use p22::figures;
use p22::figures::{Circle, Point, Rectangle, Shape, Triangle};
use std::f32::consts::PI;

#[test]
fn test_point_perimeter() {
    // Test the perimeter of a point
    let point = Point { x: 0.0, y: 0.0 };
    let perimeter = figures::point_perimeter(point);
    assert_eq!(perimeter, 0.0);
}

#[test]
fn test_circle_perimeter() {
    // Test the perimeter of a circle
    let circle = Circle {
        _center: Point { x: 0.0, y: 0.0 },
        radius: 1.0,
    };
    let perimeter = figures::circle_perimeter(circle);
    assert_eq!(perimeter, 2.0 * PI);
}

#[test]
fn test_triangle_perimeter() {
    // Test the perimeter of a triangle
    let triangle = Triangle {
        a: Point { x: 0.0, y: 0.0 },
        b: Point { x: 3.0, y: 0.0 },
        c: Point { x: 3.0, y: 4.0 },
    };
    let perimeter = figures::triangle_perimeter(triangle);
    assert_eq!(perimeter, 12.0);
}

#[test]
fn test_rectangle_perimeter() {
    // Test the perimeter of a rectangle
    let rectangle = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 2.0, y: 3.0 },
    };
    let perimeter = figures::rectangle_perimeter(rectangle);
    assert_eq!(perimeter, 10.0);
}

#[test]
fn test_perimeter() {
    // Test the perimeter function with different shapes
    let point = Shape::Point(Point { x: 0.0, y: 0.0 });
    let circle = Shape::Circle(Circle {
        _center: Point { x: 0.0, y: 0.0 },
        radius: 1.0,
    });
    let triangle = Shape::Triangle(Triangle {
        a: Point { x: 0.0, y: 0.0 },
        b: Point { x: 3.0, y: 0.0 },
        c: Point { x: 3.0, y: 4.0 },
    });
    let rectangle = Shape::Rectangle(Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 2.0, y: 3.0 },
    });

    assert_eq!(figures::perimeter(point), 0.0);
    assert_eq!(figures::perimeter(circle), 2.0 * PI);
    assert_eq!(figures::perimeter(triangle), 12.0);
    assert_eq!(figures::perimeter(rectangle), 10.0);
}
