//! Arithmetic operations module
//!
//! This module provides basic arithmetic operations like GCD, LCM,
//! and other number theory functions.

use crate::{MathError, MathResult, Number};

/// Calculates the Greatest Common Divisor (GCD) of two numbers
///
/// # Examples
///
/// ```
/// use math_utils::arithmetic::gcd;
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(35, 10), 5);
/// ```
pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Number,
{
    while b != T::zero() {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Calculates the Least Common Multiple (LCM) of two numbers
///
/// # Examples
///
/// ```
/// use math_utils::arithmetic::lcm;
/// assert_eq!(lcm(4, 6), 12);
/// assert_eq!(lcm(15, 25), 75);
/// ```
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Number,
{
    if a == T::zero() || b == T::zero() {
        T::zero()
    } else {
        (a / gcd(a, b)) * b
    }
}

/// Checks if a number is prime
///
/// # Examples
///
/// ```
/// use math_utils::arithmetic::is_prime;
/// assert!(is_prime(17));
/// assert!(!is_prime(4));
/// ```
pub fn is_prime<T>(n: T) -> bool
where
    T: Number,
{
    if n <= T::one() {
        return false;
    }
    let mut i = T::from(2).unwrap();
    while i * i <= n {
        if n % i == T::zero() {
            return false;
        }
        i = i + T::one();
    }
    true
}

/// Calculates the factorial of a number
///
/// # Examples
///
/// ```
/// use math_utils::arithmetic::factorial;
/// assert_eq!(factorial(5).unwrap(), 120);
/// ```
///
/// # Errors
///
/// Returns `MathError::InvalidInput` if the input is negative
/// Returns `MathError::OutOfRange` if the result would overflow
pub fn factorial<T>(n: T) -> MathResult<T>
where
    T: Number,
{
    if n < T::zero() {
        return Err(MathError::InvalidInput("negative number".to_string()));
    }
    
    let mut result = T::one();
    let mut i = T::one();
    
    while i <= n {
        // Check for overflow
        if let Some(new_result) = result.checked_mul(&i) {
            result = new_result;
        } else {
            return Err(MathError::OutOfRange("factorial overflow".to_string()));
        }
        i = i + T::one();
    }
    
    Ok(result)
}

/// Calculates the binomial coefficient C(n,k)
///
/// # Examples
///
/// ```
/// use math_utils::arithmetic::binomial;
/// assert_eq!(binomial(5, 2).unwrap(), 10);
/// ```
pub fn binomial<T>(n: T, k: T) -> MathResult<T>
where
    T: Number,
{
    if k > n {
        return Err(MathError::InvalidInput("k cannot be greater than n".to_string()));
    }
    
    if k < T::zero() || n < T::zero() {
        return Err(MathError::InvalidInput("negative input".to_string()));
    }
    
    let k = if k > n - k { n - k } else { k };
    
    let mut result = T::one();
    for i in T::zero()..k {
        result = result * (n - i) / (i + T::one());
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(7, 13), 1);
    }
    
    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(15, 25), 75);
        assert_eq!(lcm(8, 12), 24);
    }
    
    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(is_prime(101));
        assert!(!is_prime(4));
        assert!(!is_prime(100));
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0).unwrap(), 1);
        assert_eq!(factorial(1).unwrap(), 1);
        assert_eq!(factorial(5).unwrap(), 120);
        assert!(factorial(-1).is_err());
    }
    
    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 2).unwrap(), 10);
        assert_eq!(binomial(10, 5).unwrap(), 252);
        assert!(binomial(5, 6).is_err());
        assert!(binomial(-1, 2).is_err());
    }
} 