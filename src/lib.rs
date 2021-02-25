pub struct Collection<'a> {
    source: std::slice::Iter<'a, i32>
}

impl<'b, 'a> Collection<'a> {
    fn new(values: &'a Vec<i32>) -> Collection<'a> {
        let iter = values.iter().clone();
        Collection { source: iter }
    }
    pub fn first(&mut self) -> Option<&i32> {
        self.source.
        self.source.next()
    }
}

// trait Trait1 {
//     fn fn1() -> i32;
// }

// impl Vec for Trait1 {
//     fn fn1() -> i32 {
//         1
//     }
// }

#[cfg(test)]
mod tests {
    use crate::Collection;

    #[test]
    fn from1() {
        let x = vec![1,2,3];
        let mut result = Collection::new(&x);
        assert_eq!(result.first().unwrap().clone(), 1);
    }

}
