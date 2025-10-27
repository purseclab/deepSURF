/// implement ScopedMut for a type
#[macro_export]
macro_rules! impl_scoped_mut {
    ($t:ty) => {
        impl $crate::scoped::ScopedMut for $t {
            $crate::impl_scoped_mut_inner!($t);
        }
    };
}
/// impl ScopedMut for T {
///     impl_scoped_mut_inner!(T);
/// }
#[macro_export]
macro_rules! impl_scoped_mut_inner {
    ($t:ty) => {
        type T=$t;
        #[inline]
        fn access<R>(&self, f: impl FnOnce(&Self::T)->R) -> R {
            f(self)
        }
        #[inline]
        fn access_mut<R>(&mut self, f: impl FnOnce(&mut Self::T)->R) -> R {
            f(self)
        }
    };
}
