use std::ops::{Add, Mul, Sub};

/// A vector of floating-point numbers
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    data: Vec<f64>,
}

impl Vector {
    pub fn new(data: Vec<f64>) -> Self {
        Vector { data }
    }

    pub fn zeros(size: usize) -> Self {
        Vector {
            data: vec![0.0; size],
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &Vec<f64> {
        &self.data
    }

    pub fn dot(&self, other: &Vector) -> Option<f64> {
        if self.len() != other.len() {
            return None;
        }
        Some(self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum())
    }

    pub fn magnitude(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        Vector::new(self.data.iter().map(|x| x / mag).collect())
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector::new(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

/// A matrix of floating-point numbers
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Option<Self> {
        if data.is_empty() || data[0].is_empty() {
            return None;
        }

        let rows = data.len();
        let cols = data[0].len();

        // Verify all rows have the same length
        if !data.iter().all(|row| row.len() == cols) {
            return None;
        }

        Some(Matrix { data, rows, cols })
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Matrix {
            data,
            rows: size,
            cols: size,
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Option<Matrix> {
        if self.cols != other.rows {
            return None;
        }

        let mut result = vec![vec![0.0; other.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        Some(Matrix::new(result).unwrap())
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = vec![vec![0.0; self.rows]; self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[j][i] = self.data[i][j];
            }
        }
        Matrix {
            data: result,
            rows: self.cols,
            cols: self.rows,
        }
    }

    pub fn determinant(&self) -> Option<f64> {
        if self.rows != self.cols {
            return None;
        }

        match self.rows {
            1 => Some(self.data[0][0]),
            2 => Some(
                self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
            ),
            _ => None, // For simplicity, we're not implementing larger determinants
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
        
        assert_eq!(v1.len(), 3);
        assert_eq!(v1.dot(&v2), Some(32.0));
        
        let sum = v1.add(&v2);
        assert_eq!(sum.data, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_matrix_operations() {
        let m1 = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let m2 = Matrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
        
        let product = m1.multiply(&m2).unwrap();
        assert_eq!(product.data, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);
        
        let transpose = m1.transpose();
        assert_eq!(transpose.data, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);
    }
}
