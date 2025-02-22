use std::f64::consts::{E, PI};

/// Calculate the factorial of a number
fn factorial(n: u32) -> f64 {
    (1..=n).fold(1.0, |acc, x| acc * x as f64)
}

/// Calculate the binomial coefficient (n choose k)
fn binomial_coefficient(n: u32, k: u32) -> f64 {
    if k > n {
        return 0.0;
    }
    factorial(n) / (factorial(k) * factorial(n - k))
}

/// Calculate the probability of k successes in n trials with probability p
pub fn binomial_probability(n: u32, k: u32, p: f64) -> f64 {
    if p < 0.0 || p > 1.0 {
        panic!("Probability must be between 0 and 1");
    }
    binomial_coefficient(n, k) * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32)
}

/// Calculate the probability density function of the normal distribution
pub fn normal_distribution(x: f64, mean: f64, std_dev: f64) -> f64 {
    if std_dev <= 0.0 {
        panic!("Standard deviation must be positive");
    }
    let exponent = -(x - mean).powi(2) / (2.0 * std_dev.powi(2));
    1.0 / (std_dev * (2.0 * PI).sqrt()) * E.powf(exponent)
}

/// Calculate the cumulative distribution function of the normal distribution
pub fn normal_cdf(x: f64, mean: f64, std_dev: f64) -> f64 {
    if std_dev <= 0.0 {
        panic!("Standard deviation must be positive");
    }
    // Using error function approximation
    let z = (x - mean) / (std_dev * 2.0_f64.sqrt());
    0.5 * (1.0 + erf(z))
}

/// Calculate the error function (erf)
fn erf(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let t = 1.0 / (1.0 + 0.3275911 * x.abs());
    let sum = t * (0.254829592 + 
                   t * (-0.284496736 +
                   t * (1.421413741 +
                   t * (-1.453152027 +
                   t * 1.061405429))));
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    sign * (1.0 - sum * E.powf(-x * x))
}

/// Calculate the Poisson probability mass function
pub fn poisson_pmf(k: u32, lambda: f64) -> f64 {
    if lambda <= 0.0 {
        panic!("Lambda must be positive");
    }
    (lambda.powi(k as i32) * E.powf(-lambda)) / factorial(k)
}

/// Calculate the geometric probability mass function
pub fn geometric_pmf(k: u32, p: f64) -> f64 {
    if p <= 0.0 || p > 1.0 {
        panic!("Probability must be between 0 and 1");
    }
    p * (1.0 - p).powi((k - 1) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_probability() {
        // Probability of exactly 2 heads in 3 fair coin flips
        let p = binomial_probability(3, 2, 0.5);
        assert!((p - 0.375).abs() < 1e-10);
    }

    #[test]
    fn test_normal_distribution() {
        // Standard normal distribution at x = 0
        let p = normal_distribution(0.0, 0.0, 1.0);
        assert!((p - 1.0 / (2.0 * PI).sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_normal_cdf() {
        // CDF of standard normal at x = 0 should be approximately 0.5
        let p = normal_cdf(0.0, 0.0, 1.0);
        assert!((p - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_poisson_pmf() {
        // Probability of exactly 0 events when lambda = 1
        let p = poisson_pmf(0, 1.0);
        assert!((p - 1.0 / E).abs() < 1e-10);
    }

    #[test]
    fn test_geometric_pmf() {
        // Probability of success on first try with p = 0.5
        let p = geometric_pmf(1, 0.5);
        assert!((p - 0.5).abs() < 1e-10);
    }

    #[test]
    #[should_panic(expected = "Probability must be between 0 and 1")]
    fn test_invalid_probability() {
        binomial_probability(5, 2, 1.5);
    }

    #[test]
    #[should_panic(expected = "Standard deviation must be positive")]
    fn test_invalid_std_dev() {
        normal_distribution(0.0, 0.0, -1.0);
    }
}
