use rand::prelude::*;
use rand_distr::Normal;

/// Generate a vector of n length with elements incrementing from 0 to (n - 1).
///
/// # Arguments
///
/// * `length` - Length of the vector.
pub fn generate_vec(length: usize) -> Vec<usize> {
    let mut v = vec![0; length];
    for i in 0..length {
        v[i] = i;
    }
    return v;
}

/// Generate a vector of n length with random elements.
///
/// # Arguments
///
/// * `length` - Length of vector.
pub fn generate_random_vec(length: usize) -> Vec<isize> {
    let mut v = Vec::new();
    for _ in 0..length {
        v.push(random::<isize>());
    }
    return v;
}

pub fn generate_nordis_vec(length: usize) -> Vec<f64> {
    let mut v = Vec::new();
    let normal = Normal::new(5000.0, 5000.0).unwrap();
    for _ in 0..length {
        let val = normal.sample(&mut thread_rng());
        v.push(val);
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
}
