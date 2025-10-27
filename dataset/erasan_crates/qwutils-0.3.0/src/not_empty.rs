use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use super::*;

pub trait NotEmpty: Sized {
    fn _is_empty(&self) -> bool;
    #[inline]
    fn not_empty(self) -> Option<Self> {
        self._is_empty().map(|| self)
    }
}

impl<T> NotEmpty for &Vec<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<T> NotEmpty for &mut Vec<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<T> NotEmpty for Vec<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl NotEmpty for &str {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl NotEmpty for &mut str {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl NotEmpty for &String {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl NotEmpty for &mut String {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl NotEmpty for String {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> NotEmpty for &HashSet<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<T> NotEmpty for &mut HashSet<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<T> NotEmpty for HashSet<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K,V> NotEmpty for &HashMap<K,V> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<K,V> NotEmpty for &mut HashMap<K,V> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<K,V> NotEmpty for HashMap<K,V> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
