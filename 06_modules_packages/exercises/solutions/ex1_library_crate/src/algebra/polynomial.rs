use std::ops::{Add, Mul};

/// A polynomial with real coefficients
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    // Coefficients are stored in ascending order of degree
    // e.g., [1, 2, 3] represents 1 + 2x + 3x²
    coefficients: Vec<f64>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        // Remove trailing zeros
        let mut coeff = coefficients;
        while let Some(0.0) = coeff.last() {
            coeff.pop();
        }
        Polynomial {
            coefficients: if coeff.is_empty() { vec![0.0] } else { coeff },
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len().saturating_sub(1)
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        let mut result = 0.0;
        let mut power = 1.0;
        for coeff in &self.coefficients {
            result += coeff * power;
            power *= x;
        }
        result
    }

    pub fn derivative(&self) -> Polynomial {
        if self.coefficients.len() <= 1 {
            return Polynomial::new(vec![0.0]);
        }

        let mut derivative_coeffs = Vec::with_capacity(self.coefficients.len() - 1);
        for (i, coeff) in self.coefficients.iter().skip(1).enumerate() {
            derivative_coeffs.push(coeff * (i + 1) as f64);
        }
        Polynomial::new(derivative_coeffs)
    }

    pub fn integral(&self) -> Polynomial {
        let mut integral_coeffs = vec![0.0]; // Constant of integration
        for (i, coeff) in self.coefficients.iter().enumerate() {
            integral_coeffs.push(coeff / (i + 1) as f64);
        }
        Polynomial::new(integral_coeffs)
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let max_len = self.coefficients.len().max(other.coefficients.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a = self.coefficients.get(i).copied().unwrap_or(0.0);
            let b = other.coefficients.get(i).copied().unwrap_or(0.0);
            result.push(a + b);
        }

        Polynomial::new(result)
    }

    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let result_degree = self.degree() + other.degree();
        let mut result = vec![0.0; result_degree + 1];

        for (i, a) in self.coefficients.iter().enumerate() {
            for (j, b) in other.coefficients.iter().enumerate() {
                result[i + j] += a * b;
            }
        }

        Polynomial::new(result)
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coefficients.len() == 1 {
            return write!(f, "{}", self.coefficients[0]);
        }

        let mut first = true;
        for (i, coeff) in self.coefficients.iter().enumerate() {
            if *coeff == 0.0 {
                continue;
            }

            if !first && *coeff > 0.0 {
                write!(f, "+")?;
            }
            first = false;

            match i {
                0 => write!(f, "{}", coeff)?,
                1 => {
                    if *coeff == 1.0 {
                        write!(f, "x")?;
                    } else if *coeff == -1.0 {
                        write!(f, "-x")?;
                    } else {
                        write!(f, "{}x", coeff)?;
                    }
                }
                _ => {
                    if *coeff == 1.0 {
                        write!(f, "x^{}", i)?;
                    } else if *coeff == -1.0 {
                        write!(f, "-x^{}", i)?;
                    } else {
                        write!(f, "{}x^{}", coeff, i)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_creation() {
        let p = Polynomial::new(vec![1.0, -2.0, 1.0]); // 1 - 2x + x²
        assert_eq!(p.degree(), 2);
        assert_eq!(p.evaluate(2.0), 1.0);
    }

    #[test]
    fn test_polynomial_operations() {
        let p1 = Polynomial::new(vec![1.0, 1.0]); // 1 + x
        let p2 = Polynomial::new(vec![0.0, 1.0]); // x
        
        let sum = p1.add(&p2);
        assert_eq!(sum.coefficients, vec![1.0, 2.0]);
        
        let product = p1.multiply(&p2);
        assert_eq!(product.coefficients, vec![0.0, 1.0, 1.0]);
    }

    #[test]
    fn test_polynomial_calculus() {
        let p = Polynomial::new(vec![1.0, 2.0, 1.0]); // 1 + 2x + x²
        
        let derivative = p.derivative();
        assert_eq!(derivative.coefficients, vec![2.0, 2.0]);
        
        let integral = p.integral();
        assert_eq!(integral.coefficients, vec![0.0, 1.0, 1.0, 1.0/3.0]);
    }

    #[test]
    fn test_polynomial_display() {
        let p = Polynomial::new(vec![1.0, -2.0, 1.0]);
        assert_eq!(p.to_string(), "1-2x+x^2");
    }
}
