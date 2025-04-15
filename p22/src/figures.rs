#[derive(Clone, Debug, Copy, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Copy, Default)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

#[derive(Clone, Debug, Copy, Default)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

#[derive(Clone, Debug, Copy, Default)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

#[derive(Clone, Debug, Copy)]
pub enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

/// Area of a point is 0.0
/// ```
/// use p22::figures::Point;
/// use p22::figures::point_area;
/// let point = Point { x: 0.0, y: 0.0 };
/// assert_eq!(point_area(point), 0.0);
/// ```
pub fn point_area(_point: Point) -> f32 {
    0.0
}

/// Area of a circle is π * r^2
/// ```
/// use p22::figures::Point;
/// use p22::figures::Circle;
/// use p22::figures::circle_area;
/// let circle = Circle {
///     center: Point { x: 0.0, y: 0.0 },
///     radius: 1.0,
/// };
/// assert_eq!(circle_area(circle), std::f32::consts::PI);
/// ```
pub fn circle_area(circle: Circle) -> f32 {
    std::f32::consts::PI * circle.radius * circle.radius
}

/// Area of a triangle is 1/2 * base * height
/// ```
/// use p22::figures::Point;
/// use p22::figures::Triangle;
/// use p22::figures::triangle_area;
/// let triangle = Triangle {
///   a: Point { x: 0.0, y: 0.0 },
///   b: Point { x: 3.0, y: 0.0 },
///   c: Point { x: 3.0, y: 4.0 },
/// };
/// assert_eq!(triangle_area(triangle), 6.0);
/// ```
pub fn triangle_area(triangle: Triangle) -> f32 {
    let a = triangle.a;
    let b = triangle.b;
    let c = triangle.c;

    let ab = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
    let ac = ((c.x - a.x).powi(2) + (c.y - a.y).powi(2)).sqrt();
    let bc = ((c.x - b.x).powi(2) + (c.y - b.y).powi(2)).sqrt();

    let s = (ab + ac + bc) / 2.0;
    (s * (s - ab) * (s - ac) * (s - bc)).sqrt()
}

/// Area of a rectangle is width * height
/// ```
/// use p22::figures::Point;
/// use p22::figures::Rectangle;
/// use p22::figures::rectangle_area;
/// let rectangle = Rectangle {
///     top_left: Point { x: 0.0, y: 0.0 },
///     bottom_right: Point { x: 2.0, y: 3.0 },
/// };
/// assert_eq!(rectangle_area(rectangle), 6.0);
/// ```
pub fn rectangle_area(rectangle: Rectangle) -> f32 {
    let width = rectangle.bottom_right.x - rectangle.top_left.x;
    let height = rectangle.bottom_right.y - rectangle.top_left.y;
    width * height
}

/// Area of a shape is the area of the shape
/// ```
/// use p22::figures::Point;
/// use p22::figures::Circle;
/// use p22::figures::Triangle;
/// use p22::figures::Rectangle;
/// use p22::figures::Shape;
/// use p22::figures::area;
/// let point = Shape::Point(Point { x: 0.0, y: 0.0 });
/// let circle = Shape::Circle(Circle {
///    center: Point { x: 0.0, y: 0.0 },
///    radius: 1.0,
/// });
/// let triangle = Shape::Triangle(Triangle {
///   a: Point { x: 0.0, y: 0.0 },
///   b: Point { x: 3.0, y: 0.0 },
///   c: Point { x: 3.0, y: 4.0 },
/// });
/// let rectangle = Shape::Rectangle(Rectangle {
///   top_left: Point { x: 0.0, y: 0.0 },
///   bottom_right: Point { x: 2.0, y: 3.0 },
/// });
/// assert_eq!(area(point), 0.0);
/// assert_eq!(area(circle), std::f32::consts::PI);
/// assert_eq!(area(triangle), 6.0);
/// assert_eq!(area(rectangle), 6.0);
/// ```
pub fn area(shape: Shape) -> f32 {
    match shape {
        Shape::Point(point) => point_area(point),
        Shape::Circle(circle) => circle_area(circle),
        Shape::Triangle(triangle) => triangle_area(triangle),
        Shape::Rectangle(rectangle) => rectangle_area(rectangle),
    }
}

/// Perimeter of a point is 0.0
/// ```
/// use p22::figures::Point;
/// use p22::figures::point_perimeter;
/// let point = Point { x: 0.0, y: 0.0 };
/// assert_eq!(point_perimeter(point), 0.0);
/// ```
pub fn point_perimeter(_point: Point) -> f32 {
    0.0
}

/// Perimeter of a circle is 2 * π * radius
/// ```
/// use p22::figures::Point;
/// use p22::figures::Circle;
/// use p22::figures::circle_perimeter;
/// let circle = Circle {
///     center: Point { x: 0.0, y: 0.0 },
///     radius: 1.0,
/// };
/// assert_eq!(circle_perimeter(circle), 2.0 * std::f32::consts::PI);
/// ```
pub fn circle_perimeter(circle: Circle) -> f32 {
    2.0 * std::f32::consts::PI * circle.radius
}

/// Perimeter of a triangle is the sum of the lengths of its sides
/// ```
/// use p22::figures::Point;
/// use p22::figures::Triangle;
/// use p22::figures::triangle_perimeter;
/// let triangle = Triangle {
///    a: Point { x: 0.0, y: 0.0 },
///    b: Point { x: 3.0, y: 0.0 },
///    c: Point { x: 3.0, y: 4.0 },
/// };
/// assert_eq!(triangle_perimeter(triangle), 12.0);
/// ```
pub fn triangle_perimeter(triangle: Triangle) -> f32 {
    let a = triangle.a;
    let b = triangle.b;
    let c = triangle.c;

    let ab = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
    let ac = ((c.x - a.x).powi(2) + (c.y - a.y).powi(2)).sqrt();
    let bc = ((c.x - b.x).powi(2) + (c.y - b.y).powi(2)).sqrt();

    ab + ac + bc
}

/// Perimeter of a rectangle is 2 * (width + height)
/// ```
/// use p22::figures::Point;
/// use p22::figures::Rectangle;
/// use p22::figures::rectangle_perimeter;
/// let rectangle = Rectangle {
///     top_left: Point { x: 0.0, y: 0.0 },
///     bottom_right: Point { x: 2.0, y: 3.0 },
/// };
/// assert_eq!(rectangle_perimeter(rectangle), 10.0);
/// ```
pub fn rectangle_perimeter(rectangle: Rectangle) -> f32 {
    let width = rectangle.bottom_right.x - rectangle.top_left.x;
    let height = rectangle.bottom_right.y - rectangle.top_left.y;
    2.0 * (width + height)
}

/// Perimeter of a shape is the perimeter of the shape
/// ```
/// use p22::figures::Point;
/// use p22::figures::Circle;
/// use p22::figures::Triangle;
/// use p22::figures::Rectangle;
/// use p22::figures::Shape;
/// use p22::figures::perimeter;
/// let point = Shape::Point(Point { x: 0.0, y: 0.0 });
/// let circle = Shape::Circle(Circle {
///   center: Point { x: 0.0, y: 0.0 },
///   radius: 1.0,
/// });
/// let triangle = Shape::Triangle(Triangle {
///   a: Point { x: 0.0, y: 0.0 },
///   b: Point { x: 3.0, y: 0.0 },
///   c: Point { x: 3.0, y: 4.0 },
/// });
/// let rectangle = Shape::Rectangle(Rectangle {
///   top_left: Point { x: 0.0, y: 0.0 },
///   bottom_right: Point { x: 2.0, y: 3.0 },
/// });
/// assert_eq!(perimeter(point), 0.0);
/// assert_eq!(perimeter(circle), 2.0 * std::f32::consts::PI);
/// assert_eq!(perimeter(triangle), 12.0);
/// assert_eq!(perimeter(rectangle), 10.0);
/// ```
pub fn perimeter(shape: Shape) -> f32 {
    match shape {
        Shape::Point(point) => point_perimeter(point),
        Shape::Circle(circle) => circle_perimeter(circle),
        Shape::Triangle(triangle) => triangle_perimeter(triangle),
        Shape::Rectangle(rectangle) => rectangle_perimeter(rectangle),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_area() {
        let point = Shape::Point(Point { x: 0.0, y: 0.0 });
        assert_eq!(area(point), 0.0);
    }

    #[test]
    fn test_circle_area() {
        let circle = Shape::Circle(Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 1.0,
        });
        assert_eq!(area(circle), std::f32::consts::PI);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Shape::Triangle(Triangle {
            a: Point { x: 0.0, y: 0.0 },
            b: Point { x: 3.0, y: 0.0 },
            c: Point { x: 3.0, y: 4.0 },
        });
        assert_eq!(area(triangle), 6.0);
    }

    #[test]
    fn test_rectangle_area() {
        let rectangle = Shape::Rectangle(Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 2.0, y: 3.0 },
        });
        assert_eq!(area(rectangle), 6.0);
    }

    #[test]
    fn test_area() {
        let point = Shape::Point(Point { x: 0.0, y: 0.0 });
        let circle = Shape::Circle(Circle {
            center: Point { x: 0.0, y: 0.0 },
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

        assert_eq!(area(point), 0.0);
        assert_eq!(area(circle), std::f32::consts::PI);
        assert_eq!(area(triangle), 6.0);
        assert_eq!(area(rectangle), 6.0);
    }
}
