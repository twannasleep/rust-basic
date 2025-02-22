//! Statistics module
//!
//! This module provides statistical functions for analyzing numerical data.

use std::collections::HashMap;
use crate::{MathError, MathResult, Number};

/// Calculates the mean (average) of a sequence of numbers
///
/// # Examples
///
/// ```
/// use math_utils::statistics::mean;
/// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// assert_eq!(mean(&numbers), Some(3.0));
/// ```
pub fn mean<T>(numbers: &[T]) -> Option<T>
where
    T: Number,
{
    if numbers.is_empty() {
        return None;
    }
    
    let sum = numbers.iter().fold(T::zero(), |acc, &x| acc + x);
    Some(sum / T::from(numbers.len()).unwrap())
}

/// Calculates the median of a sequence of numbers
///
/// # Examples
///
/// ```
/// use math_utils::statistics::median;
/// let mut numbers = vec![1.0, 3.0, 5.0, 2.0, 4.0];
/// assert_eq!(median(&mut numbers), Some(3.0));
/// ```
pub fn median<T>(numbers: &mut [T]) -> Option<T>
where
    T: Number,
{
    if numbers.is_empty() {
        return None;
    }
    
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numbers.len() / 2;
    
    if numbers.len() % 2 == 0 {
        mean(&[numbers[mid - 1], numbers[mid]])
    } else {
        Some(numbers[mid])
    }
}

/// Calculates the mode (most frequent value) of a sequence of numbers
///
/// # Examples
///
/// ```
/// use math_utils::statistics::mode;
/// let numbers = vec![1.0, 2.0, 2.0, 3.0, 2.0, 4.0];
/// assert_eq!(mode(&numbers), Some(2.0));
/// ```
pub fn mode<T>(numbers: &[T]) -> Option<T>
where
    T: Number + std::hash::Hash + Eq,
{
    if numbers.is_empty() {
        return None;
    }
    
    let mut counts = HashMap::new();
    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }
    
    counts.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
}

/// Calculates the variance of a sequence of numbers
///
/// # Examples
///
/// ```
/// use math_utils::statistics::variance;
/// let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
/// assert!((variance(&numbers).unwrap() - 4.0).abs() < 1e-10);
/// ```
pub fn variance<T>(numbers: &[T]) -> MathResult<T>
where
    T: Number + Into<f64> + From<f64>,
{
    if numbers.is_empty() {
        return Err(MathError::EmptyDataSet);
    }
    
    let m = mean(numbers).unwrap();
    let squared_diff_sum = numbers.iter()
        .map(|&x| {
            let diff: T = x - m;
            diff * diff
        })
        .fold(T::zero(), |acc, x| acc + x);
    
    Ok(squared_diff_sum / T::from(numbers.len()).unwrap())
}

/// Calculates the standard deviation of a sequence of numbers
///
/// # Examples
///
/// ```
/// use math_utils::statistics::standard_deviation;
/// let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
/// assert!((standard_deviation(&numbers).unwrap() - 2.0).abs() < 1e-10);
/// ```
pub fn standard_deviation<T>(numbers: &[T]) -> MathResult<T>
where
    T: Number + Into<f64> + From<f64>,
{
    let var = variance(numbers)?;
    let std_dev = (var.into() as f64).sqrt();
    Ok(T::from(std_dev))
}

/// Calculates the correlation coefficient between two sequences of numbers
///
/// # Examples
///
/// ```
/// use math_utils::statistics::correlation;
/// let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let y = vec![2.0, 4.0, 5.0, 4.0, 5.0];
/// assert!((correlation(&x, &y).unwrap() - 0.8366).abs() < 1e-4);
/// ```
pub fn correlation<T>(x: &[T], y: &[T]) -> MathResult<T>
where
    T: Number + Into<f64> + From<f64>,
{
    if x.len() != y.len() {
        return Err(MathError::InvalidInput("sequences must have equal length".to_string()));
    }
    
    if x.is_empty() {
        return Err(MathError::EmptyDataSet);
    }
    
    let mean_x = mean(x).unwrap();
    let mean_y = mean(y).unwrap();
    
    let mut covariance = T::zero();
    let mut var_x = T::zero();
    let mut var_y = T::zero();
    
    for i in 0..x.len() {
        let dx = x[i] - mean_x;
        let dy = y[i] - mean_y;
        covariance = covariance + dx * dy;
        var_x = var_x + dx * dx;
        var_y = var_y + dy * dy;
    }
    
    if var_x == T::zero() || var_y == T::zero() {
        return Err(MathError::InvalidInput("zero variance".to_string()));
    }
    
    let correlation = covariance / (var_x * var_y).into_iter()
        .map(|x: f64| x.sqrt())
        .fold(1.0, |acc, x| acc * x);
    
    Ok(T::from(correlation))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_mean() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&numbers), Some(3.0));
        
        let empty: Vec<f64> = vec![];
        assert_eq!(mean(&empty), None);
    }
    
    #[test]
    fn test_median() {
        let mut numbers = vec![1.0, 3.0, 5.0, 2.0, 4.0];
        assert_eq!(median(&mut numbers), Some(3.0));
        
        let mut even = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(median(&mut even), Some(2.5));
    }
    
    #[test]
    fn test_mode() {
        let numbers = vec![1.0, 2.0, 2.0, 3.0, 2.0, 4.0];
        assert_eq!(mode(&numbers), Some(2.0));
        
        let no_mode = vec![1.0, 2.0, 3.0];
        assert_eq!(mode(&no_mode), Some(1.0)); // Returns first in case of tie
    }
    
    #[test]
    fn test_variance() {
        let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_relative_eq!(variance(&numbers).unwrap(), 4.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_standard_deviation() {
        let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_relative_eq!(standard_deviation(&numbers).unwrap(), 2.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 5.0, 4.0, 5.0];
        assert_relative_eq!(correlation(&x, &y).unwrap(), 0.8366, epsilon = 1e-4);
    }
} 