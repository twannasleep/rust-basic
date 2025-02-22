//! Geometry module
//!
//! This module provides geometric calculations and transformations.

pub mod shapes;
pub mod transformations;

use std::f64::consts::PI;

/// Common geometric constants
pub const TAU: f64 = 2.0 * PI;
pub const HALF_PI: f64 = PI / 2.0;

/// A point in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new point
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    /// Calculates the distance to another point
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A vector in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    /// Creates a new vector
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }
    
    /// Calculates the magnitude (length) of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    /// Normalizes the vector (makes it unit length)
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        } else {
            Vector {
                x: self.x / mag,
                y: self.y / mag,
            }
        }
    }
    
    /// Calculates the dot product with another vector
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }
    
    /// Calculates the cross product with another vector
    pub fn cross(&self, other: &Vector) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_relative_eq!(p1.distance_to(&p2), 5.0);
    }
    
    #[test]
    fn test_vector_operations() {
        let v1 = Vector::new(3.0, 4.0);
        assert_relative_eq!(v1.magnitude(), 5.0);
        
        let normalized = v1.normalize();
        assert_relative_eq!(normalized.magnitude(), 1.0);
        
        let v2 = Vector::new(1.0, 2.0);
        assert_relative_eq!(v1.dot(&v2), 11.0);
        assert_relative_eq!(v1.cross(&v2), 2.0);
    }
} 