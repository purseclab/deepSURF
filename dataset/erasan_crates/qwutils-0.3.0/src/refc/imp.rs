use std::rc::Rc;
use std::sync::Arc;
use crate::*;

impl<T> RefClonable for Arc<T> where T: ?Sized {
    #[inline]
    fn refc(&self) -> Self {
        Arc::clone(self)
    }
}

impl<T> RefClonable for Rc<T> where T: ?Sized {
    #[inline]
    fn refc(&self) -> Self {
        Rc::clone(self)
    }
}

impl<T> RefClonable for Box<T> where T: RefClonable {
    #[inline]
    fn refc(&self) -> Self {
        Box::new( (**self).refc() )
    }
}