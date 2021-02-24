fn average(values: Vec<i32>) -> f32 {
  values.iter().sum::<i32>() as f32 / values.len() as f32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average1() {
        let x = vec![1, 2, 3];
        let y = average(x);
        assert_eq!(y, 2.0);
    }
}