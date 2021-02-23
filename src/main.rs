fn main() {
    let x = vec![1, 2, 3,4,5,6,7,8,9,10];
    let y = shuffle(x);
    assert_eq!(y.len(), 3);
}
fn shuffle(x: Vec<i32>) -> Vec<i32> {
    use rand::Rng;
    use std::collections::HashSet;
    let len = x.len();
    let mut rng = rand::thread_rng();
    let mut y: Vec<i32> = vec![0;len];
    let mut tmp = HashSet::new();
    while tmp.len() != len {
        let i = rng.gen_range(0..len);
        tmp.insert(i);
    }
    for (i, v) in tmp.iter().enumerate() {
        y[i] = x[*v];
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
