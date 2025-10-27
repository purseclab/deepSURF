use std::borrow::Cow;
use super::*;

impl<'a,S,T> ScopedMut for &'a mut S where S: ScopedMut<T=T> + Clone {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <S as ScopedMut>::access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <S as ScopedMut>::access_mut(self,f)
    }
}

impl<'a,S,T> ScopedMut for Box<S> where S: ScopedMut<T=T> + Clone {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <S as ScopedMut>::access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <S as ScopedMut>::access_mut(self,f)
    }
}

impl<S,T> ScopedMut for Cow<'_,S> where S: ScopedMut<T=T> + Clone {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <S as ScopedMut>::access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        let b = self.to_mut();
        <S as ScopedMut>::access_mut(b,f)
    }
}

