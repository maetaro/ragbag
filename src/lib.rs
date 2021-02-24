/// Returns a shuffled collection.
///
/// # Arguments
///
/// * `values` - A vector that original collection.
///
/// # Examples
///
/// ```
/// use ragbag::shuffle;
/// let vec = shuffle(vec![1,2,3]);
/// ```
pub fn shuffle(values: Vec<i32>) -> Vec<i32> {
    use rand::Rng;
    use std::collections::HashSet;
    let len = values.len();
    let mut rng = rand::thread_rng();
    let mut y: Vec<i32> = vec![0; len];
    let mut tmp = HashSet::new();
    while tmp.len() != len {
        let i = rng.gen_range(0..len);
        tmp.insert(i);
    }
    for (i, v) in tmp.iter().enumerate() {
        y[i] = values[*v];
    }
    y
}

/// Returns a average of collection.
///
/// # Arguments
///
/// * `values` - A vector that original collection.
///
/// # Examples
///
/// ```
/// use ragbag::average;
/// let result = average(vec![1,2,3]);
/// assert_eq!(result, 2.0);
/// ```
pub fn average(values: Vec<i32>) -> f32 {
  values.iter().sum::<i32>() as f32 / values.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle1() {
        let x = vec![1, 2, 3];
        let y = shuffle(x);
        assert_eq!(y.len(), 3);
        let actual: i32 = y.iter().sum();
        assert_eq!(actual, 6);
    }

    #[test]
    fn average1() {
        let x = vec![1, 2, 3];
        let y = average(x);
        assert_eq!(y, 2.0);
    }
}