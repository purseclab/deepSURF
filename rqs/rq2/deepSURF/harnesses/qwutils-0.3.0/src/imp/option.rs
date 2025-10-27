use std::ops::*;

pub trait OptionExt<T> {
    fn with<R>(&self, f: impl FnOnce(&T)->R) -> Option<R>;
    fn with_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> Option<R>;

    fn with_if<R,U>(&self, o: &Option<U>, f: impl FnOnce(&T,&U)->R) -> Option<R>;
    fn with_mut_if<R,U>(&mut self, o: &Option<U>, f: impl FnOnce(&mut T,&U)->R) -> Option<R>;

    fn with_mut_if_saturating<R>(&mut self, o: Option<T>, f: impl FnOnce(&mut T,&T)->R) -> Option<R>;

    fn add_to<V>(&mut self, v: V) where T: AddAssign<V>;
    fn sub_to<V>(&mut self, v: V) where T: SubAssign<V>;
    fn mul_to<V>(&mut self, v: V) where T: MulAssign<V>;
    fn div_to<V>(&mut self, v: V) where T: DivAssign<V>;

    fn add_to_lossy<V>(&mut self, v: Option<V>) where T: AddAssign<V>;
    fn sub_to_lossy<V>(&mut self, v: Option<V>) where T: SubAssign<V>;
    fn mul_to_lossy<V>(&mut self, v: Option<V>) where T: MulAssign<V>;
    fn div_to_lossy<V>(&mut self, v: Option<V>) where T: DivAssign<V>;

    fn add_to_if<V>(&mut self, v: Option<V>) where T: AddAssign<V>;
    fn sub_to_if<V>(&mut self, v: Option<V>) where T: SubAssign<V>;
    fn mul_to_if<V>(&mut self, v: Option<V>) where T: MulAssign<V>;
    fn div_to_if<V>(&mut self, v: Option<V>) where T: DivAssign<V>;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn with<R>(&self, f: impl FnOnce(&T)->R) -> Option<R> {
        self.as_ref().map(f)
    }
    #[inline]
    fn with_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> Option<R> {
        self.as_mut().map(f)
    }

    #[inline]
    fn with_if<R,U>(&self, o: &Option<U>, f: impl FnOnce(&T,&U)->R) -> Option<R> {
        flatten( self.with(#[inline] |s| o.with(|o| f(s,o) )) )
    }
    #[inline]
    fn with_mut_if<R,U>(&mut self, o: &Option<U>, f: impl FnOnce(&mut T,&U)->R) -> Option<R> {
        flatten( self.with_mut(#[inline] |s| o.with(|o| f(s,o) )) )
    }

    #[inline]
    fn with_mut_if_saturating<R>(&mut self, o: Option<T>, f: impl FnOnce(&mut T,&T)->R) -> Option<R> {
        if let Some(s) = self {
            o.with(#[inline] |o| f(s,o) )
        }else{
            *self = o;
            None
        }
    }

    #[inline]
    fn add_to<V>(&mut self, v: V) where T: AddAssign<V> {
        self.with_mut(#[inline] |s| AddAssign::add_assign(s,v) );
    }
    #[inline]
    fn sub_to<V>(&mut self, v: V) where T: SubAssign<V> {
        self.with_mut(#[inline] |s| SubAssign::sub_assign(s,v) );
    }
    #[inline]
    fn mul_to<V>(&mut self, v: V) where T: MulAssign<V> {
        self.with_mut(#[inline] |s| MulAssign::mul_assign(s,v) );
    }
    #[inline]
    fn div_to<V>(&mut self, v: V) where T: DivAssign<V> {
        self.with_mut(#[inline] |s| DivAssign::div_assign(s,v) );
    }

    #[inline]
    fn add_to_lossy<V>(&mut self, v: Option<V>) where T: AddAssign<V> {
        if let Some(v) = v {
            self.add_to(v)
        }else{
            *self = None;
        }
    }
    #[inline]
    fn sub_to_lossy<V>(&mut self, v: Option<V>) where T: SubAssign<V> {
        if let Some(v) = v {
            self.sub_to(v)
        }else{
            *self = None;
        }
    }
    #[inline]
    fn mul_to_lossy<V>(&mut self, v: Option<V>) where T: MulAssign<V> {
        if let Some(v) = v {
            self.mul_to(v)
        }else{
            *self = None;
        }
    }
    #[inline]
    fn div_to_lossy<V>(&mut self, v: Option<V>) where T: DivAssign<V> {
        if let Some(v) = v {
            self.div_to(v)
        }else{
            *self = None;
        }
    }

    #[inline]
    fn add_to_if<V>(&mut self, v: Option<V>) where T: AddAssign<V> {
        if let Some(v) = v {
            self.add_to(v)
        }
    }
    #[inline]
    fn sub_to_if<V>(&mut self, v: Option<V>) where T: SubAssign<V> {
        if let Some(v) = v {
            self.sub_to(v)
        }
    }
    #[inline]
    fn mul_to_if<V>(&mut self, v: Option<V>) where T: MulAssign<V> {
        if let Some(v) = v {
            self.mul_to(v)
        }
    }
    #[inline]
    fn div_to_if<V>(&mut self, v: Option<V>) where T: DivAssign<V> {
        if let Some(v) = v {
            self.div_to(v)
        }
    }
}

#[inline]
fn flatten<T>(i: Option<Option<T>>) -> Option<T> {
    match i {
        Some(j) => j,
        None => None,
    }
}