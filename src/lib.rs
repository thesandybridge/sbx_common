/// Generate a vector of n length with elements incrementing from 0 to (n - 1).
///
/// # Arguments
///
/// * `length` - Length of the vector.
///
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
        v.push(rand::random::<isize>());
    }
    return v;
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
}
