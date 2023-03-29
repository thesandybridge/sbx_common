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

fn swap(vec: &mut [isize], i: usize, j: usize) {
    let temp = vec[i];
    vec[i] = vec[j];
    vec[j] = temp;
}

fn partition(vec: &mut [isize], lo: usize, hi: usize) -> usize {
    let pivot = vec[hi];
    let mut index = lo;
    let mut i = lo;

    while i < hi {
        if vec[i] < pivot {
            swap(vec, i, index);
            index += 1;
        }
        i += 1;
    }
    swap(vec, index, hi);
    return index as usize;
}

pub fn qs(lo: usize, hi: usize, vec: &mut Vec<isize>, ) {
    if lo >= hi {
        return;
    }
    let pivot = partition(vec, lo, hi);
    qs(lo, (pivot - 1) as usize, vec);
    qs((pivot + 1) as usize, hi, vec);
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
    fn quicksort() {
        let mut vec_a = vec![1, 300, 124, 432, 200, 78, 0, -376];
        let vec_b = vec![-376, 0, 1, 78, 124, 200, 300, 432];
        qs(0, vec_a.len() - 1, &mut vec_a);

        assert_eq!(vec_a, vec_b);
    }
}
