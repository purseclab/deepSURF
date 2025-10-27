pub mod imp;
pub mod macros;
/// a type which inner type T can be accessed scoped
/// 
/// use impl_scoped_mut!(T) if a implementation is missing
pub trait ScopedMut {
    type T;

    fn access<R>(&self, f: impl FnOnce(&Self::T)->R) -> R;
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut Self::T)->R) -> R;
}
/// like ScopedMut, but explict with interior mutability
pub trait Interior {
    type T;

    fn interior_access<R>(&self, f: impl FnOnce(&Self::T)->R) -> R;
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut Self::T)->R) -> R;
}
