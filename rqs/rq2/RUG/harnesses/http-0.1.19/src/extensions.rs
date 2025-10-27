use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::fmt;
#[allow(warnings)]
type AnyMap = HashMap<TypeId, Box<dyn Any + Send + Sync>, BuildHasherDefault<IdHasher>>;
#[derive(Default)]
struct IdHasher(u64);
impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }
    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}
/// A type map of protocol extensions.
///
/// `Extensions` can be used by `Request` and `Response` to store
/// extra data derived from the underlying protocol.
#[derive(Default)]
pub struct Extensions {
    map: Option<Box<AnyMap>>,
}
impl Extensions {
    /// Create an empty `Extensions`.
    #[inline]
    pub fn new() -> Extensions {
        Extensions { map: None }
    }
    /// Insert a type into this `Extensions`.
    ///
    /// If a extension of this type already existed, it will
    /// be returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::Extensions;
    /// let mut ext = Extensions::new();
    /// assert!(ext.insert(5i32).is_none());
    /// assert!(ext.insert(4u8).is_none());
    /// assert_eq!(ext.insert(9i32), Some(5i32));
    /// ```
    pub fn insert<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .get_or_insert_with(|| Box::new(HashMap::default()))
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(|boxed| {
                #[allow(warnings)]
                { (boxed as Box<dyn Any + 'static>).downcast().ok().map(|boxed| *boxed) }
            })
    }
    /// Get a reference to a type previously inserted on this `Extensions`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::Extensions;
    /// let mut ext = Extensions::new();
    /// assert!(ext.get::<i32>().is_none());
    /// ext.insert(5i32);
    ///
    /// assert_eq!(ext.get::<i32>(), Some(&5i32));
    /// ```
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.map
            .as_ref()
            .and_then(|map| map.get(&TypeId::of::<T>()))
            .and_then(|boxed| {
                #[allow(warnings)] { (&**boxed as &(dyn Any + 'static)).downcast_ref() }
            })
    }
    /// Get a mutable reference to a type previously inserted on this `Extensions`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::Extensions;
    /// let mut ext = Extensions::new();
    /// ext.insert(String::from("Hello"));
    /// ext.get_mut::<String>().unwrap().push_str(" World");
    ///
    /// assert_eq!(ext.get::<String>().unwrap(), "Hello World");
    /// ```
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.map
            .as_mut()
            .and_then(|map| map.get_mut(&TypeId::of::<T>()))
            .and_then(|boxed| {
                #[allow(warnings)]
                { (&mut **boxed as &mut (dyn Any + 'static)).downcast_mut() }
            })
    }
    /// Remove a type from this `Extensions`.
    ///
    /// If a extension of this type existed, it will be returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::Extensions;
    /// let mut ext = Extensions::new();
    /// ext.insert(5i32);
    /// assert_eq!(ext.remove::<i32>(), Some(5i32));
    /// assert!(ext.get::<i32>().is_none());
    /// ```
    pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.map
            .as_mut()
            .and_then(|map| map.remove(&TypeId::of::<T>()))
            .and_then(|boxed| {
                #[allow(warnings)]
                { (boxed as Box<dyn Any + 'static>).downcast().ok().map(|boxed| *boxed) }
            })
    }
    /// Clear the `Extensions` of all inserted extensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::Extensions;
    /// let mut ext = Extensions::new();
    /// ext.insert(5i32);
    /// ext.clear();
    ///
    /// assert!(ext.get::<i32>().is_none());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        if let Some(ref mut map) = self.map {
            map.clear();
        }
    }
}
impl fmt::Debug for Extensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Extensions").finish()
    }
}
#[test]
fn test_extensions() {
    #[derive(Debug, PartialEq)]
    struct MyType(i32);
    let mut extensions = Extensions::new();
    extensions.insert(5i32);
    extensions.insert(MyType(10));
    assert_eq!(extensions.get(), Some(& 5i32));
    assert_eq!(extensions.get_mut(), Some(& mut 5i32));
    assert_eq!(extensions.remove::< i32 > (), Some(5i32));
    assert!(extensions.get::< i32 > ().is_none());
    assert_eq!(extensions.get::< bool > (), None);
    assert_eq!(extensions.get(), Some(& MyType(10)));
}
#[cfg(test)]
mod tests_rug_534 {
    use super::*;
    use crate::extensions::IdHasher;
    use crate::std::hash::Hasher;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_534_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = b"test_data";
        let mut p0 = IdHasher::default();
        let p1: &[u8] = rug_fuzz_0;
        p0.write(p1);
        let _rug_ed_tests_rug_534_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_535 {
    use super::*;
    use crate::std::hash::Hasher;
    use extensions::IdHasher;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = IdHasher::default();
        let mut p1: u64 = rug_fuzz_0;
        <IdHasher as std::hash::Hasher>::write_u64(&mut p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_536 {
    use super::*;
    use crate::std::hash::Hasher;
    use crate::extensions::IdHasher;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_536_rrrruuuugggg_test_rug = 0;
        let mut p0: IdHasher = IdHasher::default();
        p0.finish();
        let _rug_ed_tests_rug_536_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_537 {
    use super::*;
    use crate::Extensions;
    #[test]
    fn test_new_extensions() {
        let _rug_st_tests_rug_537_rrrruuuugggg_test_new_extensions = 0;
        let extensions = Extensions::new();
        let _rug_ed_tests_rug_537_rrrruuuugggg_test_new_extensions = 0;
    }
}
#[cfg(test)]
mod tests_rug_538 {
    use super::*;
    use crate::Extensions;
    use std::any::TypeId;
    use std::collections::HashMap;
    use std::any::Any;
    #[test]
    fn test_insert() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut ext = Extensions::new();
        debug_assert!(ext.insert(rug_fuzz_0).is_none());
        debug_assert!(ext.insert(rug_fuzz_1).is_none());
        debug_assert_eq!(ext.insert(rug_fuzz_2), Some(5i32));
             }
});    }
}
#[cfg(test)]
mod tests_rug_539 {
    use super::*;
    use crate::Extensions;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_539_rrrruuuugggg_test_rug = 0;
        let mut p0 = Extensions::new();
        <Extensions>::get::<i32>(&p0);
        let _rug_ed_tests_rug_539_rrrruuuugggg_test_rug = 0;
    }
}
use super::*;
#[cfg(test)]
mod tests_rug_540 {
    use super::*;
    use crate::Extensions;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_540_rrrruuuugggg_test_rug = 0;
        let mut p0 = Extensions::new();
        let p0_ref = &mut p0;
        <extensions::Extensions>::get_mut::<String>(p0_ref).unwrap();
        let _rug_ed_tests_rug_540_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_541 {
    use super::*;
    use crate::Extensions;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Extensions::new();
        p0.insert(rug_fuzz_0);
        debug_assert_eq!(p0.remove:: < i32 > (), Some(5i32));
        debug_assert!(p0.get:: < i32 > ().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_rug_542 {
    use super::*;
    use crate::Extensions;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_542_rrrruuuugggg_test_rug = 0;
        let mut p0 = Extensions::new();
        <extensions::Extensions>::clear(&mut p0);
        debug_assert!(p0.get:: < i32 > ().is_none());
        let _rug_ed_tests_rug_542_rrrruuuugggg_test_rug = 0;
    }
}
