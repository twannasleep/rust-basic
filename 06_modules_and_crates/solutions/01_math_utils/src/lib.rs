//! Math Utilities Library
//!
//! This library provides a comprehensive set of mathematical utilities
//! organized into modules for arithmetic, statistics, and geometry.
//!
//! # Features
//!
//! - Basic arithmetic operations (GCD, LCM, etc.)
//! - Statistical functions (mean, median, mode, etc.)
//! - Geometric calculations (areas, perimeters, transformations)
//!
//! # Examples
//!
//! ```
//! use math_utils::arithmetic::gcd;
//! use math_utils::statistics::mean;
//! use math_utils::geometry::shapes::Rectangle;
//!
//! // Calculate GCD
//! assert_eq!(gcd(48, 18), 6);
//!
//! // Calculate mean
//! let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! assert_eq!(mean(&numbers), Some(3.0));
//!
//! // Create and use geometric shapes
//! let rect = Rectangle::new(5.0, 3.0);
//! assert_eq!(rect.area(), 15.0);
//! ```

use std::error::Error;
use std::fmt;
use thiserror::Error;

pub mod arithmetic;
pub mod statistics;
pub mod geometry;

/// Common error type for math operations
#[derive(Error, Debug)]
pub enum MathError {
    #[error("division by zero")]
    DivisionByZero,
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("value out of range: {0}")]
    OutOfRange(String),
    #[error("empty data set")]
    EmptyDataSet,
}

/// Result type for math operations
pub type MathResult<T> = Result<T, MathError>;

/// Trait for types that can be used in mathematical operations
pub trait Number: num_traits::Num + Copy + PartialOrd + fmt::Debug {}

impl<T> Number for T where T: num_traits::Num + Copy + PartialOrd + fmt::Debug {}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_math_error() {
        let err = MathError::DivisionByZero;
        assert_eq!(err.to_string(), "division by zero");

        let err = MathError::InvalidInput("negative number".to_string());
        assert_eq!(err.to_string(), "invalid input: negative number");
    }

    #[test]
    fn test_number_trait() {
        fn accepts_number<T: Number>(x: T, y: T) -> T {
            x + y
        }

        assert_eq!(accepts_number(1, 2), 3);
        assert_eq!(accepts_number(1.5, 2.5), 4.0);
    }
} 