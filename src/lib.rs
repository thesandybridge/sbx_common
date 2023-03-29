/// Helper function to generate a vector.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_vector() {
        let vec = generate_vec(5);
        assert_eq!(vec, vec![0, 1, 2, 3, 4]);
    }
}
