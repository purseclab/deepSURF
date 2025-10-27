use std::cmp::Ord;
use std::cmp::Ordering;
#[derive(Debug)]
pub enum BST<T: Ord> {
    Leaf { value: T, left: Box<BST<T>>, right: Box<BST<T>> },
    Empty,
}
impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST::Empty
    }
    pub fn create(value: T) -> Self {
        BST::Leaf {
            value,
            left: Box::new(BST::Empty),
            right: Box::new(BST::Empty),
        }
    }
    pub fn insert(&mut self, new_value: T) {
        match self {
            BST::Leaf { ref value, ref mut left, ref mut right } => {
                match new_value.cmp(value) {
                    Ordering::Less => left.insert(new_value),
                    Ordering::Greater => right.insert(new_value),
                    _ => return,
                }
            }
            BST::Empty => {
                *self = BST::create(new_value);
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            BST::Empty => true,
            BST::Leaf { .. } => false,
        }
    }
    pub fn find(&self, find_value: T) -> bool {
        match self {
            BST::Leaf { ref value, ref left, ref right } => {
                match find_value.cmp(value) {
                    Ordering::Less => left.find(find_value),
                    Ordering::Greater => right.find(find_value),
                    Ordering::Equal => true,
                }
            }
            BST::Empty => false,
        }
    }
}
impl<T: Ord> Default for BST<T> {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[ignore]
    #[test]
    fn create() {
        let mut t1 = BST::new();
        t1.insert(3);
        t1.insert(1);
        t1.insert(2);
        println!("{:?}", t1)
    }
    #[test]
    fn find() {
        let mut t1 = BST::new();
        t1.insert(3);
        t1.insert(1);
        t1.insert(2);
        assert_eq!(true, t1.find(2));
        assert_eq!(false, t1.find(5));
    }
}
#[cfg(test)]
mod tests_rug_23 {
    use super::*;
    use crate::tree::bst::BST;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_23_rrrruuuugggg_test_rug = 0;
        let result = BST::<i32>::new();
        let _rug_ed_tests_rug_23_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_24 {
    use super::*;
    use crate::search::binary;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v17: i8 = rug_fuzz_0;
        crate::tree::bst::BST::<i8>::create(v17);
             }
});    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::tree::bst::BST;
    use std::cmp::Ordering;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: BST<char> = BST::Empty;
        let mut p1: char = rug_fuzz_0;
        p0.insert(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_26 {
    use super::*;
    use crate::tree::bst::BST;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_26_rrrruuuugggg_test_rug = 0;
        let p0: BST<()> = BST::Empty;
        debug_assert_eq!(p0.is_empty(), true);
        let _rug_ed_tests_rug_26_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_27 {
    use super::*;
    use crate::tree::bst::BST;
    use std::cmp::Ordering;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: BST<i8> = BST::Leaf {
            value: rug_fuzz_0,
            left: Box::new(BST::Leaf {
                value: rug_fuzz_1,
                left: Box::new(BST::Empty),
                right: Box::new(BST::Empty),
            }),
            right: Box::new(BST::Leaf {
                value: rug_fuzz_2,
                left: Box::new(BST::Empty),
                right: Box::new(BST::Empty),
            }),
        };
        let p1: i8 = rug_fuzz_3;
        debug_assert_eq!(p0.find(p1), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_28 {
    use super::*;
    use crate::tree::bst::BST;
    use std::default::Default;
    #[test]
    fn test_default() {
        let _rug_st_tests_rug_28_rrrruuuugggg_test_default = 0;
        let bst_default: BST<i32> = Default::default();
        let _rug_ed_tests_rug_28_rrrruuuugggg_test_default = 0;
    }
}
