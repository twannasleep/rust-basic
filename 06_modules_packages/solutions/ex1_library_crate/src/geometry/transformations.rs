use std::ops::{Add, Sub};

/// A 2D point or vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// A 2D transformation matrix
#[derive(Debug, Clone, PartialEq)]
pub struct Transform2D {
    // 3x3 matrix in row-major order
    // [0, 1, 2]
    // [3, 4, 5]
    // [6, 7, 8]
    matrix: [f64; 9],
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn origin() -> Self {
        Point::new(0.0, 0.0)
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f64 {
        self.distance_to(&Point::origin())
    }

    pub fn normalize(&self) -> Point {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        } else {
            Point::new(self.x / mag, self.y / mag)
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Transform2D {
    pub fn identity() -> Self {
        Transform2D {
            matrix: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            ],
        }
    }

    pub fn translation(dx: f64, dy: f64) -> Self {
        Transform2D {
            matrix: [
                1.0, 0.0, dx,
                0.0, 1.0, dy,
                0.0, 0.0, 1.0
            ],
        }
    }

    pub fn rotation(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Transform2D {
            matrix: [
                cos, -sin, 0.0,
                sin,  cos, 0.0,
                0.0,  0.0, 1.0
            ],
        }
    }

    pub fn scale(sx: f64, sy: f64) -> Self {
        Transform2D {
            matrix: [
                sx,  0.0, 0.0,
                0.0, sy,  0.0,
                0.0, 0.0, 1.0
            ],
        }
    }

    pub fn compose(&self, other: &Transform2D) -> Transform2D {
        let mut result = [0.0; 9];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i * 3 + j] +=
                        self.matrix[i * 3 + k] * other.matrix[k * 3 + j];
                }
            }
        }
        Transform2D { matrix: result }
    }

    pub fn apply(&self, point: &Point) -> Point {
        let x = point.x * self.matrix[0] + point.y * self.matrix[1] + self.matrix[2];
        let y = point.x * self.matrix[3] + point.y * self.matrix[4] + self.matrix[5];
        let w = point.x * self.matrix[6] + point.y * self.matrix[7] + self.matrix[8];
        Point::new(x / w, y / w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_point_operations() {
        let p1 = Point::new(3.0, 4.0);
        let p2 = Point::new(0.0, 0.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
        
        let p3 = Point::new(1.0, 0.0);
        let p4 = Point::new(0.0, 1.0);
        assert_eq!(p3.dot(&p4), 0.0);
    }

    #[test]
    fn test_point_arithmetic() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1 + p2, Point::new(4.0, 6.0));
        assert_eq!(p2 - p1, Point::new(2.0, 2.0));
    }

    #[test]
    fn test_transformations() {
        let p = Point::new(1.0, 0.0);
        
        // Test translation
        let t = Transform2D::translation(2.0, 3.0);
        assert_eq!(t.apply(&p), Point::new(3.0, 3.0));
        
        // Test rotation
        let r = Transform2D::rotation(PI / 2.0);
        let rotated = r.apply(&p);
        assert!((rotated.x - 0.0).abs() < 1e-10);
        assert!((rotated.y - 1.0).abs() < 1e-10);
        
        // Test scale
        let s = Transform2D::scale(2.0, 3.0);
        assert_eq!(s.apply(&p), Point::new(2.0, 0.0));
        
        // Test composition
        let combined = t.compose(&r);
        let result = combined.apply(&p);
        assert!((result.x - 2.0).abs() < 1e-10);
        assert!((result.y - 4.0).abs() < 1e-10);
    }
}
