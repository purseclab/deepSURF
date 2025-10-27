use std::mem::{drop, forget, replace, uninitialized, ManuallyDrop};

use arrayvec::Array;

use super::error::CapacityError;

#[derive(Debug)]
pub struct ArrayQueue<A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> {
    array: ManuallyDrop<A>,
    start: usize,
    length: usize,
}

impl<A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> ArrayQueue<A> {
    pub fn new() -> Self {
        ArrayQueue {
            array: unsafe { uninitialized() },
            start: 0,
            length: 0,
        }
    }

    pub fn first(&self) -> Option<&<A as Array>::Item> {
        self.element(0)
    }

    pub fn first_mut(&mut self) -> Option<&mut <A as Array>::Item> {
        self.element_mut(0)
    }

    pub fn last(&self) -> Option<&<A as Array>::Item> {
        if self.is_empty() {
            return None;
        }

        self.element(self.length - 1)
    }

    pub fn last_mut(&mut self) -> Option<&mut <A as Array>::Item> {
        if self.is_empty() {
            return None;
        }

        let i = self.length - 1;
        self.element_mut(i)
    }

    fn element(&self, i: usize) -> Option<&<A as Array>::Item> {
        if self.is_empty() {
            None
        } else {
            Some(&self.array.as_ref()[self.index(i)])
        }
    }

    fn element_mut(&mut self, i: usize) -> Option<&mut <A as Array>::Item> {
        if self.is_empty() {
            None
        } else {
            let i = self.index(i);
            Some(&mut self.array.as_mut()[i])
        }
    }

    pub fn push_back(&mut self, x: &<A as Array>::Item) -> Result<(), CapacityError>
    where
        <A as Array>::Item: Clone,
    {
        if self.is_full() {
            return Err(CapacityError);
        }

        let i = self.index(self.length);
        forget(replace(&mut self.array.as_mut()[i], x.clone()));
        self.length += 1;
        Ok(())
    }

    pub fn push_front(&mut self, x: &<A as Array>::Item) -> Result<(), CapacityError>
    where
        <A as Array>::Item: Clone,
    {
        //println!("push_front1");
        if self.is_full() {
            return Err(CapacityError);
        }

        //println!("push_front2");
        //let tmp = self.index(Self::capacity() - 1);
        self.start = self.index(Self::capacity() - 1);
        //println!("push_front3");
        //println!("self.start = {:?}", self.start);
        forget(replace(&mut self.array.as_mut()[self.start], x.clone()));
        //self.start = self.index(Self::capacity() - 1);
        self.length += 1;
        //println!("push_front4");
        Ok(())
    }

    pub fn pop_back(&mut self) -> Option<<A as Array>::Item> {
        if self.is_empty() {
            return None;
        }

        let x = replace(&mut self.array.as_mut()[self.length - 1], unsafe {
            uninitialized()
        });
        self.length -= 1;
        Some(x)
    }

    pub fn pop_front(&mut self) -> Option<<A as Array>::Item> {
        if self.is_empty() {
            return None;
        }

        let x = replace(&mut self.array.as_mut()[self.start], unsafe {
            uninitialized()
        });
        self.start = self.index(1);
        self.length -= 1;
        Some(x)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == Self::capacity()
    }

    fn index(&self, i: usize) -> usize {
        //println!("self.start = {:?}", (self.start + i) % Self::capacity());
        (self.start + i) % Self::capacity()
    }

    fn capacity() -> usize {
        A::capacity()
    }
}

impl<A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> Clone for ArrayQueue<A>
where
    <A as Array>::Item: Clone,
{
    fn clone(&self) -> Self {
        //println!("0");
        let mut a = Self::new();
        //println!("1");
        for x in self {
            //println!("2");
            a.push_back(x).unwrap();
            //println!("3");
        }

        a
    }
}

impl<A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> Default
    for ArrayQueue<A>
{
    fn default() -> Self {
        ArrayQueue::new()
    }
}

impl<A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> Drop for ArrayQueue<A> {
    fn drop(&mut self) {
        for x in self {
            //println!("before drop");
            drop(replace(x, unsafe { uninitialized() }));
            //println!("after drop");
        }
    }
}

impl<'a, A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> IntoIterator
    for &'a ArrayQueue<A>
{
    type Item = &'a <A as Array>::Item;
    type IntoIter = ArrayQueueIterator<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        let l = self.len();

        ArrayQueueIterator {
            queue: self,
            first: 0,
            last: l,
        }
    }
}

impl<'a, A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> IntoIterator
    for &'a mut ArrayQueue<A>
{
    type Item = &'a mut <A as Array>::Item;
    type IntoIter = ArrayQueueMutIterator<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        let l = self.len();

        ArrayQueueMutIterator {
            queue: self,
            first: 0,
            last: l,
        }
    }
}

#[derive(Debug)]
pub struct ArrayQueueIterator<
    'a,
    A: 'a + Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>,
> {
    queue: &'a ArrayQueue<A>,
    first: usize,
    last: usize,
}

impl<'a, A: 'a + Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>>
    ArrayQueueIterator<'a, A>
{
    fn exhausted(&self) -> bool {
        self.first >= self.last
    }
}

impl<'a, A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> Iterator
    for ArrayQueueIterator<'a, A>
{
    type Item = &'a <A as Array>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted() {
            return None;
        }

        let x = &self.queue.array.as_ref()[self.queue.index(self.first)];
        self.first += 1;
        Some(x)
    }
}

impl<'a, A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> DoubleEndedIterator
    for ArrayQueueIterator<'a, A>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.exhausted() {
            return None;
        }

        self.last -= 1;
        let x = &self.queue.array.as_ref()[self.queue.index(self.last)];
        Some(x)
    }
}

#[derive(Debug)]
pub struct ArrayQueueMutIterator<
    'a,
    A: 'a + Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>,
> {
    queue: &'a mut ArrayQueue<A>,
    first: usize,
    last: usize,
}

impl<'a, A: 'a + Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>>
    ArrayQueueMutIterator<'a, A>
{
    fn exhausted(&self) -> bool {
        self.first >= self.last
    }
}

impl<'a, A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> Iterator
    for ArrayQueueMutIterator<'a, A>
{
    type Item = &'a mut <A as Array>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted() {
            return None;
        }

        let i = self.queue.index(self.first);
        let x = &mut self.queue.array.as_mut()[i] as *mut <A as Array>::Item;
        self.first += 1;
        Some(unsafe { &mut *x })
    }
}

impl<'a, A: Array + AsRef<[<A as Array>::Item]> + AsMut<[<A as Array>::Item]>> DoubleEndedIterator
    for ArrayQueueMutIterator<'a, A>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.exhausted() {
            return None;
        }

        self.last -= 1;
        let i = self.queue.index(self.last);
        let x = &mut self.queue.array.as_mut()[i] as *mut <A as Array>::Item;
        Some(unsafe { &mut *x })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        ArrayQueue::<[usize; 1]>::new();
        ArrayQueue::<[usize; 2]>::new();
    }

    #[test]
    fn first_and_last() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert_eq!(a.first(), None);
        assert_eq!(a.first_mut(), None);
        assert_eq!(a.last(), None);
        assert_eq!(a.last_mut(), None);

        assert!(a.push_back(&1).is_ok());

        assert_eq!(a.first(), Some(&1));
        assert_eq!(a.first_mut(), Some(&mut 1));
        assert_eq!(a.last(), Some(&1));
        assert_eq!(a.last_mut(), Some(&mut 1));

        assert!(a.push_back(&2).is_ok());

        assert_eq!(a.first(), Some(&1));
        assert_eq!(a.first_mut(), Some(&mut 1));
        assert_eq!(a.last(), Some(&2));
        assert_eq!(a.last_mut(), Some(&mut 2));
    }

    #[test]
    fn push_back() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();

        assert_eq!(a.len(), 0);
        assert!(a.push_back(&42).is_ok());
        assert_eq!(a.len(), 1);
        assert_eq!(a.push_back(&42), Err(CapacityError));
        assert_eq!(a.len(), 1);

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert_eq!(a.len(), 0);
        assert!(a.push_back(&42).is_ok());
        assert_eq!(a.len(), 1);
        assert!(a.push_back(&42).is_ok());
        assert_eq!(a.len(), 2);
        assert_eq!(a.push_back(&42), Err(CapacityError));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn push_front() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();

        assert_eq!(a.len(), 0);
        assert!(a.push_front(&42).is_ok());
        assert_eq!(a.len(), 1);
        assert_eq!(a.push_front(&42), Err(CapacityError));
        assert_eq!(a.len(), 1);

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert_eq!(a.len(), 0);
        assert!(a.push_front(&1).is_ok());
        assert_eq!(a.first(), Some(&1));
        assert_eq!(a.last(), Some(&1));
        assert_eq!(a.len(), 1);
        assert!(a.push_front(&2).is_ok());
        assert_eq!(a.first(), Some(&2));
        assert_eq!(a.last(), Some(&1));
        assert_eq!(a.len(), 2);
        assert_eq!(a.push_front(&3), Err(CapacityError));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn pop_back() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();

        assert!(a.push_back(&42).is_ok());

        assert_eq!(a.pop_back(), Some(42));
        assert_eq!(a.len(), 0);

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&123).is_ok());
        assert!(a.push_back(&42).is_ok());

        assert_eq!(a.pop_back(), Some(42));
        assert_eq!(a.first(), Some(&123));
        assert_eq!(a.last(), Some(&123));
        assert_eq!(a.len(), 1);
        assert_eq!(a.pop_back(), Some(123));
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn pop_front() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();

        assert!(a.push_back(&42).is_ok());

        assert_eq!(a.pop_front(), Some(42));
        assert_eq!(a.len(), 0);

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&123).is_ok());
        assert!(a.push_back(&42).is_ok());

        assert_eq!(a.pop_front(), Some(123));
        assert_eq!(a.first(), Some(&42));
        assert_eq!(a.last(), Some(&42));
        assert_eq!(a.len(), 1);
        assert_eq!(a.pop_front(), Some(42));
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn push_and_pop_across_edges() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&1).is_ok());
        assert!(a.push_back(&2).is_ok());

        for i in 3..64 {
            assert_eq!(a.pop_front(), Some(i - 2));
            assert_eq!(a.len(), 1);
            assert!(a.push_back(&i).is_ok());
            assert_eq!(a.len(), 2);
        }
    }

    #[test]
    fn is_empty() {
        let a: ArrayQueue<[usize; 1]> = ArrayQueue::new();
        assert!(a.is_empty());

        let a: ArrayQueue<[usize; 2]> = ArrayQueue::new();
        assert!(a.is_empty());
    }

    #[test]
    fn is_full() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();
        assert!(a.push_back(&0).is_ok());
        assert!(a.is_full());

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();
        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&0).is_ok());
        assert!(a.is_full());
    }

    #[test]
    fn iterator() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&1).is_ok());

        for (i, e) in a.into_iter().enumerate() {
            assert_eq!(*e, i);
        }
    }

    #[test]
    fn iterator_across_edges() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&42).is_ok());
        a.pop_front();
        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&1).is_ok());

        for (i, e) in a.into_iter().enumerate() {
            assert_eq!(*e, i);
        }
    }

    #[test]
    fn iterate_forward_and_backward() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&1).is_ok());

        let mut i = a.into_iter();

        assert_eq!(i.next(), Some(&0));
        assert_eq!(i.next_back(), Some(&1));
        assert_eq!(i.next(), None);
        assert_eq!(i.next_back(), None);
    }

    #[test]
    fn iterate_forward_and_backward_mutablly() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&1).is_ok());

        let mut i = (&mut a).into_iter();

        assert_eq!(i.next(), Some(&mut 0));
        assert_eq!(i.next_back(), Some(&mut 1));
        assert_eq!(i.next(), None);
        assert_eq!(i.next_back(), None);
    }

    #[test]
    fn iterate_empty_queue() {
        let a = ArrayQueue::<[usize; 0]>::new();

        for _ in a.into_iter() {}
    }

    #[test]
    fn iterator_mut() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&1).is_ok());

        for (i, e) in (&mut a).into_iter().enumerate() {
            assert_eq!(*e, i);
            *e = 42;
        }
    }

    #[test]
    fn reference_elements() {
        let mut a: ArrayQueue<[Box<usize>; 2]> = ArrayQueue::new();
        assert!(a.push_back(&Box::new(42)).is_ok());
        assert!(a.push_front(&Box::new(42)).is_ok());
    }

    #[test]
    fn clone() {
        let mut a: ArrayQueue<[Box<usize>; 32]> = ArrayQueue::new();

        for _ in 0..32 {
            assert!(a.push_back(&Box::new(42)).is_ok());
        }

        a.clone();
    }

    static mut FOO_SUM: usize = 0;

    #[derive(Clone)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            unsafe {
                FOO_SUM += 1;
            }
        }
    }

    #[test]
    fn no_drops_of_elements_on_push_back() {
        assert_eq!(unsafe { FOO_SUM }, 0);

        let mut a: ArrayQueue<[Foo; 32]> = ArrayQueue::new();

        for _ in 0..32 {
            assert!(a.push_back(&Foo).is_ok());
        }

        assert_eq!(unsafe { FOO_SUM }, 32); // drops of arguments `&Foo`

        drop(a);

        assert_eq!(unsafe { FOO_SUM }, 64); // drops of elements
    }

    static mut BAR_SUM: usize = 0;

    #[derive(Clone)]
    struct Bar;

    impl Drop for Bar {
        fn drop(&mut self) {
            unsafe {
                BAR_SUM += 1;
            }
        }
    }

    #[test]
    fn drops_of_elements_on_pop_back() {
        assert_eq!(unsafe { BAR_SUM }, 0);

        let mut a: ArrayQueue<[Bar; 32]> = ArrayQueue::new();

        for _ in 0..32 {
            assert!(a.push_back(&Bar).is_ok());
        }

        assert_eq!(unsafe { BAR_SUM }, 32); // drops of arguments `&Bar`

        for _ in 0..32 {
            assert!(a.pop_back().is_some());
        }

        assert_eq!(unsafe { BAR_SUM }, 64); // drops of elements

        drop(a);

        assert_eq!(unsafe { BAR_SUM }, 64);
    }
}
