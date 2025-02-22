//! Geometric shapes module
//!
//! This module provides implementations for various geometric shapes.

use std::f64::consts::PI;
use super::{Point, Vector};
use crate::MathError;

/// A trait for shapes that can calculate their area and perimeter
pub trait Shape {
    /// Calculates the area of the shape
    fn area(&self) -> f64;
    
    /// Calculates the perimeter of the shape
    fn perimeter(&self) -> f64;
    
    /// Checks if a point is inside the shape
    fn contains(&self, point: Point) -> bool;
}

/// A circle defined by its center and radius
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    /// Creates a new circle
    ///
    /// # Errors
    ///
    /// Returns `MathError::InvalidInput` if radius is negative
    pub fn new(center: Point, radius: f64) -> Result<Self, MathError> {
        if radius < 0.0 {
            Err(MathError::InvalidInput("radius must be non-negative".to_string()))
        } else {
            Ok(Circle { center, radius })
        }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
    
    fn contains(&self, point: Point) -> bool {
        self.center.distance_to(&point) <= self.radius
    }
}

/// A rectangle defined by its top-left corner and dimensions
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub origin: Point,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    /// Creates a new rectangle
    ///
    /// # Errors
    ///
    /// Returns `MathError::InvalidInput` if width or height is negative
    pub fn new(origin: Point, width: f64, height: f64) -> Result<Self, MathError> {
        if width < 0.0 || height < 0.0 {
            Err(MathError::InvalidInput("dimensions must be non-negative".to_string()))
        } else {
            Ok(Rectangle { origin, width, height })
        }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn contains(&self, point: Point) -> bool {
        point.x >= self.origin.x
            && point.x <= self.origin.x + self.width
            && point.y >= self.origin.y
            && point.y <= self.origin.y + self.height
    }
}

/// A triangle defined by its three vertices
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    /// Creates a new triangle
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Triangle { a, b, c }
    }
    
    /// Calculates the signed area of the triangle
    fn signed_area(&self) -> f64 {
        let v1 = Vector::new(self.b.x - self.a.x, self.b.y - self.a.y);
        let v2 = Vector::new(self.c.x - self.a.x, self.c.y - self.a.y);
        v1.cross(&v2) / 2.0
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.signed_area().abs()
    }
    
    fn perimeter(&self) -> f64 {
        self.a.distance_to(&self.b)
            + self.b.distance_to(&self.c)
            + self.c.distance_to(&self.a)
    }
    
    fn contains(&self, point: Point) -> bool {
        let total_area = self.area();
        let t1 = Triangle::new(point, self.b, self.c);
        let t2 = Triangle::new(self.a, point, self.c);
        let t3 = Triangle::new(self.a, self.b, point);
        
        let sum_areas = t1.area() + t2.area() + t3.area();
        (sum_areas - total_area).abs() < 1e-10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_circle() {
        let circle = Circle::new(Point::new(0.0, 0.0), 2.0).unwrap();
        assert_relative_eq!(circle.area(), 4.0 * PI);
        assert_relative_eq!(circle.perimeter(), 4.0 * PI);
        
        assert!(circle.contains(Point::new(1.0, 1.0)));
        assert!(!circle.contains(Point::new(2.0, 2.0)));
        
        assert!(Circle::new(Point::new(0.0, 0.0), -1.0).is_err());
    }
    
    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(Point::new(0.0, 0.0), 3.0, 4.0).unwrap();
        assert_relative_eq!(rect.area(), 12.0);
        assert_relative_eq!(rect.perimeter(), 14.0);
        
        assert!(rect.contains(Point::new(1.0, 1.0)));
        assert!(!rect.contains(Point::new(4.0, 4.0)));
        
        assert!(Rectangle::new(Point::new(0.0, 0.0), -1.0, 1.0).is_err());
    }
    
    #[test]
    fn test_triangle() {
        let triangle = Triangle::new(
            Point::new(0.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(0.0, 4.0),
        );
        
        assert_relative_eq!(triangle.area(), 6.0);
        assert_relative_eq!(triangle.perimeter(), 12.0);
        
        assert!(triangle.contains(Point::new(1.0, 1.0)));
        assert!(!triangle.contains(Point::new(2.0, 3.0)));
    }
} 