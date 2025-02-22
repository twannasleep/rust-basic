//! Geometric transformations module
//!
//! This module provides functions for geometric transformations like
//! translation, rotation, and scaling.

use std::f64::consts::PI;
use super::{Point, Vector};

/// A 2D transformation matrix
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    // Row-major order: [a b c; d e f; 0 0 1]
    pub a: f64, pub b: f64, pub c: f64,
    pub d: f64, pub e: f64, pub f: f64,
}

impl Transform {
    /// Creates an identity transformation
    pub fn identity() -> Self {
        Transform {
            a: 1.0, b: 0.0, c: 0.0,
            d: 0.0, e: 1.0, f: 0.0,
        }
    }
    
    /// Creates a translation transformation
    pub fn translation(tx: f64, ty: f64) -> Self {
        Transform {
            a: 1.0, b: 0.0, c: tx,
            d: 0.0, e: 1.0, f: ty,
        }
    }
    
    /// Creates a rotation transformation (angle in radians)
    pub fn rotation(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Transform {
            a: cos, b: -sin, c: 0.0,
            d: sin, e: cos,  f: 0.0,
        }
    }
    
    /// Creates a scaling transformation
    pub fn scaling(sx: f64, sy: f64) -> Self {
        Transform {
            a: sx,  b: 0.0, c: 0.0,
            d: 0.0, e: sy,  f: 0.0,
        }
    }
    
    /// Combines this transformation with another
    pub fn combine(&self, other: &Transform) -> Transform {
        Transform {
            a: self.a * other.a + self.b * other.d,
            b: self.a * other.b + self.b * other.e,
            c: self.a * other.c + self.b * other.f + self.c,
            d: self.d * other.a + self.e * other.d,
            e: self.d * other.b + self.e * other.e,
            f: self.d * other.c + self.e * other.f + self.f,
        }
    }
    
    /// Applies the transformation to a point
    pub fn apply_point(&self, point: &Point) -> Point {
        Point {
            x: self.a * point.x + self.b * point.y + self.c,
            y: self.d * point.x + self.e * point.y + self.f,
        }
    }
    
    /// Applies the transformation to a vector
    pub fn apply_vector(&self, vector: &Vector) -> Vector {
        Vector {
            x: self.a * vector.x + self.b * vector.y,
            y: self.d * vector.x + self.e * vector.y,
        }
    }
}

/// Rotates a point around a center point
pub fn rotate_around(point: &Point, center: &Point, angle: f64) -> Point {
    let translation_to_origin = Transform::translation(-center.x, -center.y);
    let rotation = Transform::rotation(angle);
    let translation_back = Transform::translation(center.x, center.y);
    
    let transform = translation_to_origin
        .combine(&rotation)
        .combine(&translation_back);
    
    transform.apply_point(point)
}

/// Reflects a point across a line defined by two points
pub fn reflect_across_line(point: &Point, line_start: &Point, line_end: &Point) -> Point {
    // Calculate the vector representing the line
    let line_vector = Vector::new(
        line_end.x - line_start.x,
        line_end.y - line_start.y,
    );
    
    // Normalize the line vector
    let normalized = line_vector.normalize();
    
    // Create the reflection matrix
    let transform = Transform {
        a: 2.0 * normalized.x * normalized.x - 1.0,
        b: 2.0 * normalized.x * normalized.y,
        c: 2.0 * normalized.x * (-line_start.x * normalized.x - line_start.y * normalized.y),
        d: 2.0 * normalized.x * normalized.y,
        e: 2.0 * normalized.y * normalized.y - 1.0,
        f: 2.0 * normalized.y * (-line_start.x * normalized.x - line_start.y * normalized.y),
    };
    
    transform.apply_point(point)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_transform_identity() {
        let transform = Transform::identity();
        let point = Point::new(2.0, 3.0);
        let transformed = transform.apply_point(&point);
        assert_relative_eq!(transformed.x, point.x);
        assert_relative_eq!(transformed.y, point.y);
    }
    
    #[test]
    fn test_transform_translation() {
        let transform = Transform::translation(2.0, 3.0);
        let point = Point::new(1.0, 1.0);
        let transformed = transform.apply_point(&point);
        assert_relative_eq!(transformed.x, 3.0);
        assert_relative_eq!(transformed.y, 4.0);
    }
    
    #[test]
    fn test_transform_rotation() {
        let transform = Transform::rotation(PI / 2.0);
        let point = Point::new(1.0, 0.0);
        let transformed = transform.apply_point(&point);
        assert_relative_eq!(transformed.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(transformed.y, 1.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_transform_scaling() {
        let transform = Transform::scaling(2.0, 3.0);
        let point = Point::new(2.0, 2.0);
        let transformed = transform.apply_point(&point);
        assert_relative_eq!(transformed.x, 4.0);
        assert_relative_eq!(transformed.y, 6.0);
    }
    
    #[test]
    fn test_transform_combination() {
        let t1 = Transform::translation(1.0, 0.0);
        let t2 = Transform::rotation(PI);
        let combined = t1.combine(&t2);
        let point = Point::new(1.0, 0.0);
        let transformed = combined.apply_point(&point);
        assert_relative_eq!(transformed.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(transformed.y, 0.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_rotate_around() {
        let point = Point::new(2.0, 0.0);
        let center = Point::new(1.0, 0.0);
        let rotated = rotate_around(&point, &center, PI);
        assert_relative_eq!(rotated.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(rotated.y, 0.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_reflect_across_line() {
        let point = Point::new(1.0, 1.0);
        let line_start = Point::new(0.0, 0.0);
        let line_end = Point::new(1.0, 0.0);
        let reflected = reflect_across_line(&point, &line_start, &line_end);
        assert_relative_eq!(reflected.x, 1.0, epsilon = 1e-10);
        assert_relative_eq!(reflected.y, -1.0, epsilon = 1e-10);
    }
} 