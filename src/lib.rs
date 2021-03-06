use rand::Rng;

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
        } else {
            left.push(i);
        }
    }

    let mut concat: Vec<i32> = Vec::new();
    let sorted_left = &mut qsort(&left, f);
    let sorted_right = &mut qsort(&right, f);
    concat.append(sorted_left);
    concat.append(sorted_right);

    concat
}

pub trait OrderExt {
    fn shuffle(&self) -> Vec<i32>;
    fn average(&self) -> f32;
    fn order_by<F: Fn(i32, i32) -> bool>(&self, f: &F) -> Self;
    fn filter<F: Fn(i32) -> bool>(&self, f: &F) -> Self;
    fn map<T, F: Fn(i32) -> T>(&self, f: &F) -> Vec<T>;
}
impl OrderExt for Vec<i32> {
    /// Returns a shuffled collection.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector that original collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use ragbag::OrderExt;
    /// let vec = vec![1,2,3].shuffle();
    /// ```
    fn shuffle(&self) -> Vec<i32> {
        use std::collections::HashSet;
        let len = self.len();
        let mut rng = rand::thread_rng();
        let mut y: Vec<i32> = vec![0; len];
        let mut tmp = HashSet::new();
        while tmp.len() != len {
            let i = rng.gen_range(0..len);
            tmp.insert(i);
        }
        for (i, v) in tmp.iter().enumerate() {
            y[i] = self[*v];
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
    /// use ragbag::OrderExt;
    /// let result = vec![1,2,3].average();
    /// assert_eq!(result, 2.0);
    /// ```
    fn average(&self) -> f32 {
        self.iter().sum::<i32>() as f32 / self.len() as f32
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
    /// use ragbag::OrderExt;
    /// let result = vec![4,5,1,2,3].order_by(&|a,b| a < b);
    /// assert_eq!(result, vec![1,2,3,4,5]);
    /// ```
    fn order_by<F: Fn(i32, i32) -> bool>(&self, f: &F) -> Vec<i32> {
        qsort(self, f)
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
    /// use ragbag::OrderExt;
    /// let result = vec![1,2,3,4,5].filter(&|a| a < 3);
    /// assert_eq!(result, vec![1,2]);
    /// ```
    fn filter<F: Fn(i32) -> bool>(&self, f: &F) -> Vec<i32> {
        let mut list: Vec<i32> = Vec::new();
        for x in self.iter() {
            if f(*x) {
                list.push(*x)
            }
        }
        list
    }
    /// Returns a collection with f.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector that original collection.
    /// * `f` - Function that is called for every element of values.
    ///
    /// # Examples
    ///
    /// ```
    /// use ragbag::OrderExt;
    /// let x = vec![1, 2, 3];
    /// let y = x.map(&|z: i32| { z.to_string() });
    /// assert_eq!(y, vec!["1", "2", "3"]);
    /// ```
    fn map<T, F: Fn(i32) -> T>(&self, f: &F) -> Vec<T> {
        let mut b: Vec<T> = Vec::new();
        for elem in self.to_owned() {
            b.push(f(elem));
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle1() {
        let x = vec![1, 2, 3];
        let y = x.shuffle();
        assert_eq!(y.len(), 3);
        let actual: i32 = y.iter().sum();
        assert_eq!(actual, 6);
    }

    #[test]
    fn average1() {
        let x = vec![1, 2, 3];
        let y = x.average();
        let error_margin = f32::EPSILON;
        assert!((y - 2.0f32).abs() < error_margin);
    }

    #[test]
    fn order_by1() {
        let expected = vec![2, 3, 4, 5, 6];
        let actual = vec![4, 3, 2, 5, 6];
        assert_eq!(actual.order_by(&|a: i32, b: i32| a < b), expected);
    }

    #[test]
    fn map1() {
        let x = vec![1, 2, 3];
        let y = x.map(&|z: i32| z.to_string());
        assert_eq!(y[0], "1");
        assert_eq!(y[1], "2");
        assert_eq!(y[2], "3");
    }
    #[test]
    fn filter1() {
        let actual = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 2];
        assert_eq!(actual.filter(&|a| a < 3), expected);
    }
}
