use std::fmt::Debug;

#[derive(Eq, PartialEq, Debug)]
pub enum Choice<T, U> {
    First(T),
    Second(U),
}

pub trait Shape {
    const NAME: &'static str;

    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
    fn scale(&mut self, factor: f32);
    fn area_to_perimeter(&self) -> f32 {
        match self.perimeter() {
            0.0 => 0.0,
            _ => self.area() / self.perimeter(),
        }
    }
    fn biggest_shape<'a, 'b, T: Shape>(&'a self, other: &'b T) -> Choice<&'a Self, &'b T> {
        if self.area() > other.area() {
            Choice::First(self)
        } else {
            Choice::Second(other)
        }
    }
    fn print_properties(&self) {
        println!("Name: {}", Self::NAME);
        println!("Area: {}", self.area());
        println!("Perimeter: {}", self.perimeter());
    }
}

#[derive(Clone, Debug, Copy, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Copy, Default, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

#[derive(Clone, Debug, Copy, Default, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

#[derive(Clone, Debug, Copy, Default, PartialEq)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum DynamicShape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

impl Shape for Point {
    const NAME: &'static str = "Point";

    fn perimeter(&self) -> f32 {
        0.0
    }

    fn area(&self) -> f32 {
        0.0
    }

    fn scale(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl Shape for Circle {
    const NAME: &'static str = "Circle";

    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    fn scale(&mut self, factor: f32) {
        self.center.scale(factor);
        self.radius *= factor;
    }
}

impl Shape for Triangle {
    const NAME: &'static str = "Triangle";

    fn perimeter(&self) -> f32 {
        let ab = ((self.a.x - self.b.x).powi(2) + (self.a.y - self.b.y).powi(2)).sqrt();
        let bc = ((self.b.x - self.c.x).powi(2) + (self.b.y - self.c.y).powi(2)).sqrt();
        let ca = ((self.c.x - self.a.x).powi(2) + (self.c.y - self.a.y).powi(2)).sqrt();
        ab + bc + ca
    }

    fn area(&self) -> f32 {
        let ab = ((self.a.x - self.b.x).powi(2) + (self.a.y - self.b.y).powi(2)).sqrt();
        let ac = ((self.a.x - self.c.x).powi(2) + (self.a.y - self.c.y).powi(2)).sqrt();
        let bc = ((self.b.x - self.c.x).powi(2) + (self.b.y - self.c.y).powi(2)).sqrt();
        let s = (ab + ac + bc) / 2.0;
        (s * (s - ab) * (s - ac) * (s - bc)).sqrt()
    }

    fn scale(&mut self, factor: f32) {
        self.a.scale(factor);
        self.b.scale(factor);
        self.c.scale(factor);
    }
}

impl Shape for Rectangle {
    const NAME: &'static str = "Rectangle";

    fn perimeter(&self) -> f32 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.bottom_right.y - self.top_left.y).abs();
        2.0 * (width + height)
    }

    fn area(&self) -> f32 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.bottom_right.y - self.top_left.y).abs();
        width * height
    }

    fn scale(&mut self, factor: f32) {
        self.top_left.scale(factor);
        self.bottom_right.scale(factor);
    }
}

impl Shape for DynamicShape {
    const NAME: &'static str = "DynamicShape";

    fn perimeter(&self) -> f32 {
        match self {
            DynamicShape::Point(point) => point.perimeter(),
            DynamicShape::Circle(circle) => circle.perimeter(),
            DynamicShape::Triangle(triangle) => triangle.perimeter(),
            DynamicShape::Rectangle(rectangle) => rectangle.perimeter(),
        }
    }

    fn area(&self) -> f32 {
        match self {
            DynamicShape::Point(point) => point.area(),
            DynamicShape::Circle(circle) => circle.area(),
            DynamicShape::Triangle(triangle) => triangle.area(),
            DynamicShape::Rectangle(rectangle) => rectangle.area(),
        }
    }

    fn scale(&mut self, factor: f32) {
        match self {
            DynamicShape::Point(point) => point.scale(factor),
            DynamicShape::Circle(circle) => circle.scale(factor),
            DynamicShape::Triangle(triangle) => triangle.scale(factor),
            DynamicShape::Rectangle(rectangle) => rectangle.scale(factor),
        }
    }
}

pub enum SliceChoice<'a, 'b, T: Shape, U: Shape> {
    First(&'a [T]),
    Second(&'b [U]),
}
/// Return slice with bigger perimeter to area ratio.
pub fn bigger_slice<'a, 'b, T: Shape + Debug, U: Shape + Debug>(
    slice1: &'a [T],
    slice2: &'b [U],
) -> SliceChoice<'a, 'b, T, U> {
    let ratio1 = slice1
        .iter()
        .map(|shape| shape.area_to_perimeter())
        .sum::<f32>()
        / slice1.len() as f32;
    let ratio2 = slice2
        .iter()
        .map(|shape| shape.area_to_perimeter())
        .sum::<f32>()
        / slice2.len() as f32;

    if ratio1 > ratio2 {
        print!("{:?}", slice1);
        SliceChoice::First(slice1)
    } else {
        print!("{:?}", slice2);
        SliceChoice::Second(slice2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let point = Point { x: 0.0, y: 0.0 };
        let circle = Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 1.0,
        };
        let triangle = Triangle {
            a: Point { x: 0.0, y: 0.0 },
            b: Point { x: 3.0, y: 0.0 },
            c: Point { x: 3.0, y: 4.0 },
        };
        let rectangle = Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 2.0, y: 3.0 },
        };
        assert_eq!(point.area(), 0.0);
        assert_eq!(circle.area(), std::f32::consts::PI);
        assert_eq!(triangle.area(), 6.0);
        assert_eq!(rectangle.area(), 6.0);
    }

    #[test]
    fn test_perimeter() {
        let point = Point { x: 0.0, y: 0.0 };
        let circle = Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 1.0,
        };
        let triangle = Triangle {
            a: Point { x: 0.0, y: 0.0 },
            b: Point { x: 3.0, y: 0.0 },
            c: Point { x: 3.0, y: 4.0 },
        };
        let rectangle = Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 2.0, y: 3.0 },
        };
        assert_eq!(point.perimeter(), 0.0);
        assert_eq!(circle.perimeter(), 2.0 * std::f32::consts::PI);
        assert_eq!(triangle.perimeter(), 12.0);
        assert_eq!(rectangle.perimeter(), 10.0);
    }

    #[test]
    fn test_scale() {
        let mut point = Point { x: 1.0, y: 1.0 };
        let mut circle = Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 1.0,
        };
        let mut triangle = Triangle {
            a: Point { x: 0.0, y: 0.0 },
            b: Point { x: 3.0, y: 0.0 },
            c: Point { x: 3.0, y: 4.0 },
        };
        let mut rectangle = Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 2.0, y: 3.0 },
        };
        point.scale(2.0);
        circle.scale(2.0);
        triangle.scale(2.0);
        rectangle.scale(2.0);
        assert_eq!(point.x, 2.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(circle.radius, 2.0);
        assert_eq!(triangle.a.x, 0.0);
        assert_eq!(triangle.a.y, 0.0);
        assert_eq!(triangle.b.x, 6.0);
        assert_eq!(triangle.b.y, 0.0);
        assert_eq!(triangle.c.x, 6.0);
        assert_eq!(triangle.c.y, 8.0);
        assert_eq!(rectangle.top_left.x, 0.0);
        assert_eq!(rectangle.top_left.y, 0.0);
        assert_eq!(rectangle.bottom_right.x, 4.0);
        assert_eq!(rectangle.bottom_right.y, 6.0);
    }

    #[test]
    fn test_area_to_perimeter() {
        let point = Point { x: 0.0, y: 0.0 };
        let circle = Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 1.0,
        };
        let triangle = Triangle {
            a: Point { x: 0.0, y: 0.0 },
            b: Point { x: 3.0, y: 0.0 },
            c: Point { x: 3.0, y: 4.0 },
        };
        let rectangle = Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 2.0, y: 3.0 },
        };
        assert_eq!(point.area_to_perimeter(), 0.0);
        assert_eq!(
            circle.area_to_perimeter(),
            std::f32::consts::PI / (2.0 * std::f32::consts::PI)
        );
        assert_eq!(triangle.area_to_perimeter(), 6.0 / 12.0);
        assert_eq!(rectangle.area_to_perimeter(), 6.0 / 10.0);
    }

    #[test]
    fn test_biggest_shape() {
        let circle = Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 1.0,
        };
        let triangle = Triangle {
            a: Point { x: 0.0, y: 0.0 },
            b: Point { x: 3.0, y: 0.0 },
            c: Point { x: 3.0, y: 4.0 },
        };
        assert_eq!(circle.biggest_shape(&triangle), Choice::Second(&triangle));
    }
}
