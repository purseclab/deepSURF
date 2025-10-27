use std::sync::Arc;
use std::rc::Rc;
use std::sync::RwLock;
use std::cell::RefCell;
use super::*;

impl<S,T> Interior for RefCell<S> where S: ScopedMut<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        let b = self.try_borrow().expect("ContainerMut access failed: borrowed RefCell");
        <S as ScopedMut>::access(&b,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        let mut b = self.try_borrow_mut().expect("ContainerMut access failed: borrowed RefCell");
        <S as ScopedMut>::access_mut(&mut b,f)
    }
}

impl<S,T> Interior for RwLock<S> where S: ScopedMut<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        let b = self.read().expect("ContainerMut access failed: poisoned RwLock");
        <S as ScopedMut>::access(&b,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        let mut b = self.write().expect("ContainerMut access failed: poisoned RwLock");
        <S as ScopedMut>::access_mut(&mut b,f)
    }
}

impl<'a,T,C> Interior for &'a C where C: Interior<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <C as Interior>::interior_access(self,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        <C as Interior>::interior_access_mut(self,f)
    }
}

impl<'a,T,C> Interior for &'a mut C where C: Interior<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <C as Interior>::interior_access(self,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        <C as Interior>::interior_access_mut(self,f)
    }
}

impl<'a,T,C> Interior for Box<C> where C: Interior<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <C as Interior>::interior_access(self,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        <C as Interior>::interior_access_mut(self,f)
    }
}

impl<T,C> Interior for Rc<C> where C: Interior<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <C as Interior>::interior_access(self,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        <C as Interior>::interior_access_mut(self,f)
    }
}

impl<T,C> Interior for Arc<C> where C: Interior<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <C as Interior>::interior_access(self,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        <C as Interior>::interior_access_mut(self,f)
    }
}

impl<S,T> ScopedMut for RefCell<S> where S: ScopedMut<T=T> {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <Self as Interior>::interior_access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <Self as Interior>::interior_access_mut(self,f)
    }
}

impl<S,T> ScopedMut for RwLock<S> where S: ScopedMut<T=T> {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <Self as Interior>::interior_access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <Self as Interior>::interior_access_mut(self,f)
    }
}

impl<S,T> ScopedMut for Rc<S> where S: Interior<T=T> {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <Self as Interior>::interior_access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <Self as Interior>::interior_access_mut(self,f)
    }
}

impl<S,T> ScopedMut for Arc<S> where S: Interior<T=T> {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <Self as Interior>::interior_access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <Self as Interior>::interior_access_mut(self,f)
    }
}