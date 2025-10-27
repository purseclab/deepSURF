use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum BST<T: Ord> {
    Leaf {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
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
            BST::Leaf {
                ref value,
                ref mut left,
                ref mut right,
            } => match new_value.cmp(value) {
                Ordering::Less => left.insert(new_value),
                Ordering::Greater => right.insert(new_value),
                _ => return,
            },
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
            BST::Leaf {
                ref value,
                ref left,
                ref right,
            } => match find_value.cmp(value) {
                Ordering::Less => left.find(find_value),
                Ordering::Greater => right.find(find_value),
                Ordering::Equal => true,
            },
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
