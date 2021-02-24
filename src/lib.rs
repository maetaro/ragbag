use rand::Rng;
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

/// Returns a sort of collection.
///
/// # Arguments
///
/// * `list` - A vector that original collection.
///
/// # Examples
///
/// ```
/// use ragbag::order_by;
/// let result = order_by(vec![10, 8, 3, 2], &|a: i32, b: i32| return a < b);
/// assert_eq!(result, vec![2, 3, 8, 10]);
/// ```
pub fn order_by<F: Fn(i32, i32) -> bool>(list: Vec<i32>, f: &F) -> Vec<i32> {
	let len = list.len();
	if len <= 1 {
		return list;
	}
	let mut rng = rand::thread_rng();
	let piv_i = rng.gen_range(0..len);
	let piv = list[piv_i];
	let mut left: Vec<i32> = Vec::new();
	let mut right: Vec<i32> = Vec::new();

	for i in list {
		if f(piv, i) {
			right.push(i);
		}else{
			left.push(i);
		}
	}

	let mut concat: Vec<i32> = Vec::new();
	let sorted_left = &mut  order_by(left, f);
	let sorted_right = &mut order_by(right, f);
	concat.append(sorted_left);
	concat.append(sorted_right);

	concat
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
        let error_margin = f32::EPSILON;
        assert!((y - 2.0f32).abs() < error_margin);
    }

	#[test]
	fn order_by1(){
		let expected = vec![2,3,4,5,6];
		let actual = vec![4,3,2,5,6];
		assert_eq!(order_by(actual, &|a: i32, b: i32| return a < b), expected);
	}
}
