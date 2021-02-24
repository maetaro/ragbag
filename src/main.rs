fn main() {
    let x = vec![1, 2, 3,4,5,6,7,8,9,10];
    let y = shuffle(x);
    assert_eq!(y.len(), 3);
}
/// Returns a shuffled collection.
/// 
/// # Arguments
///
/// * `values` - A vector that original collection.
///
/// # Examples
///
/// ```
/// let vec = shuffle(vec![1,2,3]);
/// ```
fn shuffle(values: Vec<i32>) -> Vec<i32> {
    use rand::Rng;
    use std::collections::HashSet;
    let len = values.len();
    let mut rng = rand::thread_rng();
    let mut y: Vec<i32> = vec![0;len];
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
}
