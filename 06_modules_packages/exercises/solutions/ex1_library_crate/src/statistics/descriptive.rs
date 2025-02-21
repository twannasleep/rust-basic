use std::collections::HashMap;

/// Calculate the arithmetic mean of a slice of numbers
pub fn mean(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    Some(data.iter().sum::<f64>() / data.len() as f64)
}

/// Calculate the median of a slice of numbers
pub fn median(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

/// Calculate the mode (most frequent value) of a slice of numbers
pub fn mode(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }

    let mut frequencies = HashMap::new();
    for &value in data {
        *frequencies.entry(value).or_insert(0) += 1;
    }

    frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
}

/// Calculate the variance of a slice of numbers
pub fn variance(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }

    let m = mean(data)?;
    let squared_diff_sum = data.iter()
        .map(|x| (x - m).powi(2))
        .sum::<f64>();
    
    Some(squared_diff_sum / data.len() as f64)
}

/// Calculate the standard deviation of a slice of numbers
pub fn standard_deviation(data: &[f64]) -> Option<f64> {
    variance(data).map(|v| v.sqrt())
}

/// Calculate the range (difference between max and min values)
pub fn range(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    
    let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    Some(max - min)
}

/// Calculate the quartiles (Q1, Q2, Q3) of a slice of numbers
pub fn quartiles(data: &[f64]) -> Option<(f64, f64, f64)> {
    if data.is_empty() {
        return None;
    }

    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let q2 = median(&sorted)?;
    let (lower, upper) = sorted.split_at(sorted.len() / 2);
    let q1 = median(lower)?;
    let q3 = median(if sorted.len() % 2 == 0 { upper } else { &upper[1..] })?;

    Some((q1, q2, q3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&data), Some(3.0));
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn test_median() {
        let data1 = vec![1.0, 3.0, 5.0, 7.0, 9.0];
        let data2 = vec![1.0, 3.0, 5.0, 7.0];
        assert_eq!(median(&data1), Some(5.0));
        assert_eq!(median(&data2), Some(4.0));
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn test_mode() {
        let data = vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0];
        assert_eq!(mode(&data), Some(3.0));
        assert_eq!(mode(&[]), None);
    }

    #[test]
    fn test_variance_and_std_dev() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert!((variance(&data).unwrap() - 4.0).abs() < 1e-10);
        assert!((standard_deviation(&data).unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_range_and_quartiles() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        assert_eq!(range(&data), Some(6.0));
        
        if let Some((q1, q2, q3)) = quartiles(&data) {
            assert_eq!(q1, 2.0);
            assert_eq!(q2, 4.0);
            assert_eq!(q3, 6.0);
        }
    }
}
