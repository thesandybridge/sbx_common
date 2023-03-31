use rand::prelude::*;
use rand_distr::{Normal, StandardNormal};

/// Generate a vector of n length with elements incrementing from 0 to (n - 1).
///
/// # Arguments
///
/// * `length` - Length of the vector.
pub fn generate_vec(length: usize) -> Vec<usize> {
    let mut v = Vec::with_capacity(length);
    for i in 0..length {
        v.push(i);
    }
    return v;
}

/// Generate a vector of n length with random elements.
///
/// # Arguments
///
/// * `length` - Length of vector.
pub fn generate_random_vec(length: usize) -> Vec<isize> {
    let mut v = Vec::with_capacity(length);
    for _ in 0..length {
        v.push(random::<isize>());
    }
    return v;
}

/// Generate a vector of random numbers that fall within normal distribution.
///
/// # Arguments
///
/// * `length` - length of the vector
/// * `mean` - mid point for the distribution
/// * `std_dev` - width of the distribution
/// * `min` - min range
/// * `max` - max range
pub fn generate_nordis_vec(length: usize, mean: f64, std_dev: f64, min: f64, max: f64) -> Vec<f64> {
    let mut v = Vec::with_capacity(length);
    let normal = Normal::new(mean, std_dev).unwrap();
    for _ in 0..length {
        let num = normal.sample(&mut thread_rng());
        if num >= min && num <= max {
            v.push(num)
        }
    }
    return v;
}

/// Generate a vector with a standard distribution of elements.
///
/// # Arguments
///
/// * `length` - Length of vector
pub fn generate_std_vec(length: usize) -> Vec<f64>{
    let mut v = Vec::with_capacity(length);
    for _ in 0..length {
        let n: f64 = thread_rng().sample(StandardNormal);
        v.push(n);
    }
    return v;
}

/// Add a percentage to a value.
///
/// # Arguments
///
/// * `value` - value to add to
/// * `percentage` - percentage to add
pub fn add_percent(value: usize, percentage: usize) -> usize {
    return value + (value * percentage / 100);
}

/// Subtract a percentage to a value.
///
/// # Arguments
///
/// * `value` - value to subtract from
/// * `percentage` - percentage to subtract
pub fn sub_percent(value: usize, percentage: usize) -> usize {
    return value - (value * percentage / 100);
}

/// Verify normal distribution
///
/// # Arguments
///
/// * `data` - Vector of numbers that are fit normal distribution
pub fn shapiro_wilk_test(data: &[f64]) -> f64 {
    let n = data.len();
    let sorted_data = {
        let mut temp = data.to_vec();
        temp.sort_by(|a, b| a.partial_cmp(b).unwrap());
        temp
    };

    let mean = sorted_data.iter().sum::<f64>() / n as f64;
    let var = sorted_data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    let mut w = sorted_data.iter().enumerate().map(|(i, &x)| {
        let y = (i as f64 + 1.0 - 0.5) / n as f64;
        (y - normal_cdf((x - mean) / var.sqrt())).powi(2)
    }).sum::<f64>();
    w *= (n as f64).powi(-1) * (var).powi(-1);
    w
}

fn normal_cdf(x: f64) -> f64 {
    (1.0 + erf(x / 2.0f64.sqrt())) / 2.0
}

fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    sign * (1.0 - (a1 * t + a2 * t.powi(2) + a3 * t.powi(3) + a4 * t.powi(4) + a5 * t.powi(5)) * (-x * x).exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector() {
        let vec = generate_vec(5);
        assert_eq!(vec, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn random_vec() {
        let vec_a = generate_random_vec(5);
        let vec_b = generate_random_vec(5);
        assert_ne!(vec_a, vec_b);
    }

    #[test]
    fn add_by_fifteen_percent() {
        let value = add_percent(100, 15);
        assert_eq!(value, 115);
    }

    #[test]
    fn subtract_by_fifteen_percent() {
        let value = sub_percent(100, 15);
        assert_eq!(value, 85);
    }

    #[test]
    fn verify_normal_distribution() {
        let data = generate_nordis_vec(1000, 5000.0, 5000.0, 0.0, 10000.0);
        let std = generate_std_vec(1000);

        let a = shapiro_wilk_test(&data);
        let b = shapiro_wilk_test(&std);

        assert!(a <= b);
    }
}
