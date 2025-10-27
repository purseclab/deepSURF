use std::rc::Rc;
use std::sync::Arc;
use crate::*;
impl<T> RefClonable for Arc<T>
where
    T: ?Sized,
{
    #[inline]
    fn refc(&self) -> Self {
        Arc::clone(self)
    }
}
impl<T> RefClonable for Rc<T>
where
    T: ?Sized,
{
    #[inline]
    fn refc(&self) -> Self {
        Rc::clone(self)
    }
}
impl<T> RefClonable for Box<T>
where
    T: RefClonable,
{
    #[inline]
    fn refc(&self) -> Self {
        Box::new((**self).refc())
    }
}
#[cfg(test)]
mod tests_rug_230 {
    use super::*;
    use crate::{RefClonable, imp};
    use std::sync::Arc;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: Arc<i32> = Arc::new(rug_fuzz_0);
        p0.refc();
             }
});    }
}
#[cfg(test)]
mod tests_rug_231 {
    use super::*;
    use crate::RefClonable;
    use crate::refc::imp::Rc;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Rc<String> = Rc::new(String::from(rug_fuzz_0));
        p0.refc();
             }
});    }
}
