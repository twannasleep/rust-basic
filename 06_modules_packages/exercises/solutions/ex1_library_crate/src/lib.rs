pub mod algebra;
pub mod geometry;
pub mod statistics;

// Re-export commonly used items
pub use algebra::linear::{Matrix, Vector};
pub use geometry::shapes::{Circle, Rectangle, Triangle};
pub use statistics::descriptive::{mean, median, mode};

/// Math utilities library
/// 
/// This library provides various mathematical utilities organized into three main modules:
/// - algebra: Linear algebra and polynomial operations
/// - geometry: Geometric shapes and transformations
/// - statistics: Statistical analysis and probability calculations
#[cfg(test)]
mod tests {
    use super::*;
    use algebra::linear::Vector;
    use geometry::shapes::Circle;
    use statistics::descriptive::mean;

    #[test]
    fn test_vector_operations() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
        let sum = v1.add(&v2);
        assert_eq!(sum.data(), &vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle::new(2.0);
        assert!((circle.area() - 12.566).abs() < 0.001);
    }

    #[test]
    fn test_mean_calculation() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&numbers), 3.0);
    }
}
