use super::{Arena, Idx};
use std::borrow::Borrow;

pub struct ArenaSplit<'a, T> {
    pub(crate) selected: Idx,
    pub(crate) arena: &'a mut Arena<T>,
    pub(crate) __type: std::marker::PhantomData<T>,
}

impl<T> ArenaSplit<'_, T> {
    pub fn get<I: Borrow<Idx>>(&self, index: I) -> Option<&T> {
        if index.borrow() == &self.selected {
            None
        } else {
            self.arena.get(index)
        }
    }

    pub fn get_mut<I: Borrow<Idx>>(&mut self, index: I) -> Option<&mut T> {
        if index.borrow() == &self.selected {
            None
        } else {
            self.arena.get_mut(index)
        }
    }
}
