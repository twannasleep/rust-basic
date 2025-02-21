use std::f64::consts::PI;
use super::transformations::Point;

/// A circle defined by its center point and radius
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    center: Point,
    radius: f64,
}

/// A rectangle defined by its top-left corner and dimensions
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

/// A triangle defined by its three vertices
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    vertices: [Point; 3],
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        assert!(radius > 0.0, "Radius must be positive");
        Circle { center, radius }
    }

    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.center.distance_to(point) <= self.radius
    }
}

impl Rectangle {
    pub fn new(top_left: Point, width: f64, height: f64) -> Self {
        assert!(width > 0.0 && height > 0.0, "Width and height must be positive");
        Rectangle {
            top_left,
            width,
            height,
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.top_left.x
            && point.x <= self.top_left.x + self.width
            && point.y >= self.top_left.y
            && point.y <= self.top_left.y + self.height
    }
}

impl Triangle {
    pub fn new(v1: Point, v2: Point, v3: Point) -> Self {
        Triangle {
            vertices: [v1, v2, v3],
        }
    }

    pub fn area(&self) -> f64 {
        // Using Heron's formula
        let a = self.vertices[0].distance_to(&self.vertices[1]);
        let b = self.vertices[1].distance_to(&self.vertices[2]);
        let c = self.vertices[2].distance_to(&self.vertices[0]);
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }

    pub fn perimeter(&self) -> f64 {
        let a = self.vertices[0].distance_to(&self.vertices[1]);
        let b = self.vertices[1].distance_to(&self.vertices[2]);
        let c = self.vertices[2].distance_to(&self.vertices[0]);
        a + b + c
    }

    pub fn contains(&self, point: &Point) -> bool {
        // Using barycentric coordinates
        let v0 = self.vertices[1] - self.vertices[0];
        let v1 = self.vertices[2] - self.vertices[0];
        let v2 = *point - self.vertices[0];

        let d00 = v0.dot(&v0);
        let d01 = v0.dot(&v1);
        let d11 = v1.dot(&v1);
        let d20 = v2.dot(&v0);
        let d21 = v2.dot(&v1);

        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        v >= 0.0 && w >= 0.0 && u >= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = Circle::new(Point::new(0.0, 0.0), 1.0);
        assert_eq!(circle.area(), PI);
        assert_eq!(circle.perimeter(), 2.0 * PI);
        assert!(circle.contains(&Point::new(0.5, 0.5)));
        assert!(!circle.contains(&Point::new(1.5, 1.5)));
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(Point::new(0.0, 0.0), 2.0, 3.0);
        assert_eq!(rect.area(), 6.0);
        assert_eq!(rect.perimeter(), 10.0);
        assert!(rect.contains(&Point::new(1.0, 1.0)));
        assert!(!rect.contains(&Point::new(2.5, 2.5)));
    }

    #[test]
    fn test_triangle() {
        let triangle = Triangle::new(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0),
        );
        assert_eq!(triangle.area(), 0.5);
        assert!(triangle.contains(&Point::new(0.2, 0.2)));
        assert!(!triangle.contains(&Point::new(0.8, 0.8)));
    }
}
