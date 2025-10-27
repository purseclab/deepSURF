use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

mod split;

use split::ArenaSplit;

struct IdxInner {
    index: AtomicUsize,
    removed: AtomicBool,
}

impl IdxInner {
    fn index(&self) -> Option<usize> {
        let removed = self.removed.load(Ordering::Relaxed);
        if !removed {
            Some(self.index.load(Ordering::Relaxed))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Idx {
    inner: Arc<IdxInner>,
}

impl std::fmt::Debug for Idx {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        formatter.write_str(&format!(
            "{}Idx ( {} )",
            if self.inner.removed.load(Ordering::Relaxed) {
                "Removed "
            } else {
                ""
            },
            self.inner.index.load(Ordering::Relaxed)
        ))
    }
}

impl Idx {
    pub fn value(&self) -> Option<usize> {
        self.inner.index()
    }
}

impl Eq for Idx {}
impl PartialEq for Idx {
    fn eq(&self, rhs: &Idx) -> bool {
        Arc::ptr_eq(&self.inner, &rhs.inner)
    }
}

impl Hash for Idx {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.inner.as_ref(), state)
    }
}

const DEFAULT_CAPACITY: usize = 4;

pub struct Arena<T> {
    values: Vec<(Arc<IdxInner>, T)>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn choose_second_member_of_tuple_mut<A, B>((_, value): &mut (A, B)) -> &mut B {
    value
}

#[inline]
fn choose_second_member_of_tuple_ref<A, B>((_, value): &(A, B)) -> &B {
    value
}

pub struct IterMut<'a, T> {
    iterator: std::iter::Map<
        std::slice::IterMut<'a, (Arc<IdxInner>, T)>,
        &'a dyn Fn(&mut (Arc<IdxInner>, T)) -> &mut T,
    >,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

pub struct Iter<'a, T> {
    iterator: std::iter::Map<
        std::slice::Iter<'a, (Arc<IdxInner>, T)>,
        &'a dyn Fn(&(Arc<IdxInner>, T)) -> &T,
    >,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

pub struct EntriesMut<'a, T> {
    iterator: std::slice::IterMut<'a, (Arc<IdxInner>, T)>,
}

pub struct Entries<'a, T> {
    iterator: std::slice::Iter<'a, (Arc<IdxInner>, T)>,
}

impl<'a, T> Iterator for EntriesMut<'a, T> {
    type Item = (Idx, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|(inner, value)| {
            (
                Idx {
                    inner: inner.clone(),
                },
                value,
            )
        })
    }
}

impl<'a, T> Iterator for Entries<'a, T> {
    type Item = (Idx, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|(inner, value)| {
            (
                Idx {
                    inner: inner.clone(),
                },
                value,
            )
        })
    }
}

impl<T> FromIterator<T> for Arena<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Arena {
            values: iter
                .into_iter()
                .enumerate()
                .map(|(index, value)| (create_idx(index), value))
                .collect(),
        }
    }
}
#[inline]
fn create_idx(index: usize) -> Arc<IdxInner> {
    Arc::new(IdxInner {
        index: AtomicUsize::new(index),
        removed: AtomicBool::new(false),
    })
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Arena<T> {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    #[inline]
    pub fn alloc_with_idx<F: FnOnce(Idx) -> T>(&mut self, func: F) -> Idx {
        let len = self.values.len();
        let inner = create_idx(len);
        let idx = Idx {
            inner: inner.clone(),
        };
        self.values.push((inner.clone(), func(idx)));
        Idx { inner }
    }

    #[inline]
    pub fn alloc_with<F: FnOnce() -> T>(&mut self, func: F) -> Idx {
        self.alloc_with_idx(|_| func())
    }

    #[inline]
    pub fn insert(&mut self, value: T) -> Idx {
        self.alloc(value)
    }

    #[inline]
    pub fn alloc(&mut self, value: T) -> Idx {
        self.alloc_with(|| value)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get_idx_at_index(&self, index: usize) -> Option<Idx> {
        self.values.get(index).map(|(inner, _)| Idx {
            inner: Arc::clone(&inner),
        })
    }

    pub fn split_at<'a, I: Borrow<Idx>>(
        &'a mut self,
        selected: I,
    ) -> Option<(&mut T, ArenaSplit<'a, T>)> {
        let selected = selected.borrow();

        if let Some(value) = self.get_mut(selected) {
            Some((
                unsafe { (value as *mut T).as_mut().unwrap() },
                ArenaSplit {
                    selected: selected.clone(),
                    arena: self,
                    __type: Default::default(),
                },
            ))
        } else {
            None
        }
    }

    pub fn truncate(&mut self, len: usize) {
        let end = self.values.len();
        let start = end - (end - len);

        for i in (start..end).rev() {
            self.remove_index(i);
        }
    }

    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        let len = self.values.len();
        let mut del = 0;

        for i in 0..len {
            if !f(&self.values[i].1) {
                del += 1;
            } else {
                self.swap_index(i - del, i);
            }
        }

        if del > 0 {
            self.truncate(len - del);
        }
    }

    pub fn entries<'a>(&'a self) -> Entries<'a, T> {
        Entries {
            iterator: self.values.iter(),
        }
    }

    pub fn entries_mut<'a>(&'a mut self) -> EntriesMut<'a, T> {
        EntriesMut {
            iterator: self.values.iter_mut(),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            iterator: self
                .values
                .iter_mut()
                .map(&choose_second_member_of_tuple_mut),
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            iterator: self.values.iter().map(&choose_second_member_of_tuple_ref),
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        self.into()
    }

    fn remove_index(&mut self, index: usize) -> T {
        let (removed_index, value) = self.values.remove(index);

        for (index, (idx, _)) in self.values.iter().enumerate().skip(index) {
            idx.index.store(index, Ordering::Relaxed);
        }

        removed_index.removed.store(true, Ordering::Relaxed);

        value
    }

    pub fn remove<I: Borrow<Idx>>(&mut self, index: I) -> T {
        if let Some(index) = index.borrow().value() {
            self.remove_index(index)
        } else {
            panic!("Trying to remove index that has already been removed!");
        }
    }

    fn swap_index(&mut self, a: usize, b: usize) {
        self.values.swap(a, b);
        self.values[a].0.index.store(a, Ordering::Relaxed);
        self.values[b].0.index.store(b, Ordering::Relaxed);
    }

    pub fn swap<A: Borrow<Idx>, B: Borrow<Idx>>(&mut self, a: A, b: B) {
        if let Some((a_index, b_index)) = a
            .borrow()
            .value()
            .and_then(|a| b.borrow().value().map(|b| (a, b)))
        {
            self.swap_index(a_index, b_index);
        }
    }

    pub fn position<F: Fn(&T) -> bool>(&self, func: F) -> Option<Idx> {
        for (inner, value) in self.values.iter() {
            if func(value) {
                return Some(Idx {
                    inner: Arc::clone(&inner),
                });
            }
        }

        None
    }

    pub fn apply_ordering<I: Borrow<Idx>>(&mut self, ordering: &Vec<I>) {
        assert!(ordering.len() == self.values.len());

        let mut old_arena = Arena::<T>::with_capacity(self.capacity());
        std::mem::swap(&mut old_arena.values, &mut self.values);

        for idx in ordering.iter() {
            let new_index = self.values.len();
            let old_index = idx.borrow().value().unwrap();

            let (inner, value) = old_arena.swap_remove_index(old_index);

            inner.index.store(new_index, Ordering::Relaxed);

            self.values.push((inner, value));

            idx.borrow().inner.index.store(new_index, Ordering::Relaxed);
        }
    }

    fn swap_remove_index(&mut self, index: usize) -> (Arc<IdxInner>, T) {
        let (removed_index, value) = self.values.swap_remove(index);

        if self.values.len() > 0 && index != self.values.len() {
            self.values[index].0.index.store(index, Ordering::Relaxed);
        }

        (removed_index, value)
    }

    #[cfg(test)]
    fn get_index(&mut self, index: usize) -> &mut T {
        &mut self.values[index].1
    }

    pub fn swap_remove<I: Borrow<Idx>>(&mut self, index: I) -> T {
        if let Some(index) = index.borrow().value() {
            let (removed_index, value) = self.swap_remove_index(index);
            removed_index.removed.store(true, Ordering::Relaxed);
            value
        } else {
            panic!("Trying to remove index that has already been removed!");
        }
    }

    pub fn get<I: Borrow<Idx>>(&self, index: I) -> Option<&T> {
        index
            .borrow()
            .value()
            .and_then(|index| self.values.get(index).and_then(|(_, value)| Some(value)))
    }

    pub fn get_mut<I: Borrow<Idx>>(&mut self, index: I) -> Option<&mut T> {
        if let Some(index) = index.borrow().value() {
            self.values
                .get_mut(index)
                .and_then(|(_, value)| Some(value))
        } else {
            None
        }
    }
}

impl<T> Into<Vec<T>> for Arena<T> {
    fn into(self) -> Vec<T> {
        // Set all the indexes to removed, since we can't use them anymore
        for (idx, _) in self.values.iter() {
            idx.removed.store(true, Ordering::Relaxed);
        }

        // Grab all the values and turn them into an array
        self.values.into_iter().map(|(_, value)| value).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn setup_arena() -> (Arena<String>, Idx, Idx, Idx, Idx) {
        let mut arena = Arena::new();

        let john = arena.alloc("John".into());
        let julia = arena.alloc("Julia".into());
        let jane = arena.alloc("Jane".into());
        let jake = arena.alloc("Jake".into());

        (arena, john, julia, jane, jake)
    }

    #[test]
    fn should_construct_default() {
        let arena: Arena<()> = Default::default();
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.capacity(), DEFAULT_CAPACITY);
    }

    #[test]
    fn should_construct_with_capacity() {
        let arena: Arena<()> = Arena::with_capacity(100);
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.capacity(), 100);
    }

    #[test]
    fn getting_by_index() {
        let (mut arena, john, julia, jane, jake) = setup_arena();

        assert_eq!(arena.get_index(0), "John");
        assert_eq!(arena.get_index(1), "Julia");
        assert_eq!(arena.get_index(2), "Jane");
        assert_eq!(arena.get_index(3), "Jake");

        assert_eq!(arena.get(john).unwrap(), "John");
        assert_eq!(arena.get(julia).unwrap(), "Julia");
        assert_eq!(arena.get(jane).unwrap(), "Jane");
        assert_eq!(arena.get(jake).unwrap(), "Jake");
    }

    #[test]
    fn arena_length() {
        let (mut arena, _, _, _, _) = setup_arena();
        assert_eq!(arena.len(), 4);
        arena.alloc("Wow".into());
        assert_eq!(arena.len(), 5);
    }

    #[test]
    fn swap_indexes() {
        let (mut arena, john, julia, jane, jake) = setup_arena();

        assert_eq!(arena.get_index(0), "John");
        assert_eq!(arena.get_index(1), "Julia");
        assert_eq!(arena.get_index(2), "Jane");
        assert_eq!(arena.get_index(3), "Jake");

        assert_eq!(arena.get(&john).unwrap(), "John");
        assert_eq!(arena.get(&julia).unwrap(), "Julia");
        assert_eq!(arena.get(&jane).unwrap(), "Jane");
        assert_eq!(arena.get(&jake).unwrap(), "Jake");

        arena.swap(&john, &julia);
        arena.swap(&jane, &jake);

        assert_eq!(arena.get_index(0), "Julia");
        assert_eq!(arena.get_index(1), "John");
        assert_eq!(arena.get_index(2), "Jake");
        assert_eq!(arena.get_index(3), "Jane");

        assert_eq!(arena.get(&john).unwrap(), "John");
        assert_eq!(arena.get(&julia).unwrap(), "Julia");
        assert_eq!(arena.get(&jane).unwrap(), "Jane");
        assert_eq!(arena.get(&jake).unwrap(), "Jake");
    }

    #[test]
    fn remove() {
        let (mut arena, john, julia, jane, jake) = setup_arena();

        assert_eq!(arena.len(), 4);

        arena.remove(&john);

        assert!(arena.get(&john).is_none());
        assert_eq!(arena.get(&julia).unwrap(), "Julia");
        assert_eq!(arena.get(&jane).unwrap(), "Jane");
        assert_eq!(arena.get(&jake).unwrap(), "Jake");

        assert_eq!(arena.get_index(0), "Julia");
        assert_eq!(arena.get_index(1), "Jane");
        assert_eq!(arena.get_index(2), "Jake");
    }

    #[test]
    fn swap_remove() {
        let (mut arena, john, julia, jane, jake) = setup_arena();

        assert_eq!(arena.len(), 4);

        arena.swap_remove(&john);

        assert!(arena.get(&john).is_none());
        assert_eq!(arena.get(&julia).unwrap(), "Julia");
        assert_eq!(arena.get(&jane).unwrap(), "Jane");
        assert_eq!(arena.get(&jake).unwrap(), "Jake");

        assert_eq!(arena.get_index(0), "Jake");
        assert_eq!(arena.get_index(1), "Julia");
        assert_eq!(arena.get_index(2), "Jane");
    }

    #[test]
    fn remove_should_remove_last_value() {
        let (mut arena, john, julia, jane, jake) = setup_arena();

        assert_eq!(arena.len(), 4);

        arena.swap_remove(&jake);
        arena.remove(&jane);
        arena.swap_remove(&julia);
        arena.remove(&john);

        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn convert_to_vec() {
        let (arena, _, _, _, _) = setup_arena();
        assert_eq!(arena.to_vec(), vec!["John", "Julia", "Jane", "Jake"]);
    }

    #[test]
    fn index_should_be_hashable() {
        let (mut arena, john, julia, jane, jake) = setup_arena();

        let mut seen = std::collections::HashSet::<Idx>::new();

        seen.insert(jake.clone());
        assert!(seen.contains(&jake));
        assert_eq!(jake.value().unwrap(), 3);

        arena.remove(john);
        arena.remove(julia);
        arena.remove(jane);

        assert_eq!(jake.value().unwrap(), 0);
        assert!(seen.contains(&jake));
    }

    #[test]
    fn cloned_index_should_equal() {
        let (_, john, _, _, _) = setup_arena();

        let a = john.clone();
        let b = john;

        assert!(a == b);
    }

    #[test]
    fn apply_ordering() {
        let (mut arena, john, julia, jane, jake) = setup_arena();

        let ordering = vec![&jake, &julia, &john, &jane];

        assert_eq!(arena.get_index(0), "John");
        assert_eq!(arena.get_index(1), "Julia");
        assert_eq!(arena.get_index(2), "Jane");
        assert_eq!(arena.get_index(3), "Jake");

        arena.apply_ordering(&ordering);

        assert_eq!(arena.get_index(0), "Jake");
        assert_eq!(arena.get_index(1), "Julia");
        assert_eq!(arena.get_index(2), "John");
        assert_eq!(arena.get_index(3), "Jane");

        assert_eq!(arena.get(&john).unwrap(), "John");
        assert_eq!(arena.get(&julia).unwrap(), "Julia");
        assert_eq!(arena.get(&jane).unwrap(), "Jane");
        assert_eq!(arena.get(&jake).unwrap(), "Jake");
    }

    #[test]
    fn position() {
        let (arena, _, julia, _, _) = setup_arena();

        let j = arena.position(|v| v == "Julia").unwrap();

        assert!(j == julia);
    }

    #[test]
    fn truncate() {
        let (mut arena, _, _, _, _) = setup_arena();
        arena.truncate(0);
        assert_eq!(arena.to_vec(), Vec::<String>::new());
    }

    #[test]
    fn retain() {
        let (mut arena, _, _, _, _) = setup_arena();

        arena.retain(|v| v == "Julia" || v == "Jane");

        assert_eq!(arena.to_vec(), vec!["Julia", "Jane"]);
    }

    #[test]
    fn mut_iter() {
        let (mut arena, _, _, _, _) = setup_arena();

        for val in arena.iter_mut() {
            *val = "Wow".into();
        }

        assert_eq!(arena.to_vec(), vec!["Wow"; 4])
    }

    #[test]
    fn iter() {
        let (arena, _, _, _, _) = setup_arena();

        let names = vec!["John", "Julia", "Jane", "Jake"];

        for (a, b) in arena.iter().zip(names.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn turn_iterator_into_vector() {
        let names = vec!["John", "Julia", "Jane", "Jake"];
        let other_names = vec!["John", "Julia", "Jane", "Jake"];

        let arena = names.into_iter().collect::<Arena<_>>();

        for (a, b) in arena.iter().zip(other_names.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn get_mut() {
        let (mut arena, john, _, _, _) = setup_arena();
        *(arena.get_mut(&john).unwrap()) = "Not John".into();
        assert_eq!(arena.get(&john).unwrap(), "Not John");
    }

    struct Node {
        id: Idx,
    }

    #[test]
    fn alloc_with_idx() {
        let mut arena = Arena::new();
        let idx = arena.alloc_with_idx(|id| Node { id });
        assert_eq!(arena.get(&idx).unwrap().id.value(), idx.value());
    }

    #[test]
    fn split_at() {
        let (mut arena, john, julia, jane, jake) = setup_arena();

        let (j, mut arena) = arena.split_at(&julia).unwrap();

        assert_eq!(j, "Julia");
        assert!(arena.get_mut(john).is_some());
        assert!(arena.get_mut(jane).is_some());
        assert!(arena.get_mut(jake).is_some());

        assert!(arena.get_mut(julia).is_none());
    }

    #[test]
    fn debug_printing() {
        let (mut arena, john, _, _, _) = setup_arena();

        assert_eq!(format!("{:?}", john), "Idx ( 0 )");

        arena.swap_remove(&john);

        assert_eq!(format!("{:?}", john), "Removed Idx ( 0 )");
    }

    #[test]
    fn compare_should_work_for_idx() {
        let mut arena = Arena::new();
        let left = arena.alloc(1);
        let right = left.clone();

        assert!(left.borrow() == right.borrow());
    }

    #[test]
    fn splitting_should_be_safe() {
        // A bug described here (https://github.com/bennetthardwick/nano-arena/issues/1) meant that
        // multiple mutable references could be handed out.

        struct ToggleIdx {
            first: Idx,
            second: Idx,
            state: Cell<bool>,
        }

        impl Borrow<Idx> for ToggleIdx {
            fn borrow(&self) -> &Idx {
                self.state.set(!self.state.get());

                if self.state.get() {
                    &self.first
                } else {
                    &self.second
                }
            }
        }

        let mut arena = Arena::new();

        let first = arena.alloc(1);
        let second = arena.alloc(2);

        let toggle_idx = ToggleIdx {
            first: first.clone(),
            second: second.clone(),
            state: Cell::new(false),
        };

        let (first_mut_ref, mut split_arena) = arena.split_at(toggle_idx).unwrap();
        let second_mut_ref = split_arena.get_mut(&first);

        drop(first_mut_ref);
        assert!(second_mut_ref.is_none());
    }
}
