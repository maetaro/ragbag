fn main() {
    let x = vec![1, 2, 3];
    let y = shuffle(x);
    assert_eq!(y.len(), 3);
}
fn add(a: i32, b: i32) -> i32 {
    a + b
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
    return y;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_with_doing() {
        let x = 10;
        let result = add(x, 20);
        assert_eq!(result, 30);
    }

    #[test]
    fn shuffle1() {
        let x = vec![1, 2, 3];
        let y = shuffle(x);
        assert_eq!(y.len(), 3);
        assert_eq!(y.iter().fold(0, |sum, a| sum + a), 6);
    }

}
