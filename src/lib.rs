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

/// Returns a sorted of collection.
/// using quick sort.
///
/// # Arguments
///
/// * `list` - A vector that original collection.
///
/// # Examples
///
/// ```
/// use ragbag::qsort;
/// let result = vec![3,2,1];
/// assert_eq!(qsort(&result, &|a, b| a < b), vec![1,2,3]);
/// ```
pub fn qsort<F: Fn(i32, i32) -> bool>(list: &[i32], f: &F) -> Vec<i32> {
	let len = list.len();
	if len <= 1 {
		return list.to_owned();
	}
	let mut rng = rand::thread_rng();
	let piv_i = rng.gen_range(0..len);
	let piv = list[piv_i];
	let mut left: Vec<i32> = Vec::new();
	let mut right: Vec<i32> = Vec::new();

	for i in list.to_owned() {
		if f(piv, i) {
			right.push(i);
		}else{
			left.push(i);
		}
	}

	let mut concat: Vec<i32> = Vec::new();
	let sorted_left =  &mut qsort(&left, f);
	let sorted_right = &mut qsort(&right, f);
	concat.append(sorted_left);
	concat.append(sorted_right);

	concat
}

trait OrderExt {
    fn order_by<F: Fn(i32, i32) -> bool>(&mut self, f: &F);
}
impl OrderExt for Vec<i32> {
	fn order_by<F: Fn(i32, i32) -> bool>(&mut self, f: &F) {
		*self = qsort(self, f);
	}
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
		let mut actual = vec![4,3,2,5,6];
		actual.order_by(&|a: i32, b: i32| a < b);
		assert_eq!(actual, expected);
	}
}
