use std::ptr;
use std::rc::Rc;
use std::sync::Arc;
/// A trait describing smart reference counted pointers.
///
/// Note that in a way [`Option<Arc<T>>`][Option] is also a smart reference counted pointer, just
/// one that can hold NULL.
///
/// The trait is unsafe, because a wrong implementation will break the [ArcSwapAny]
/// implementation and lead to UB.
///
/// This is not actually expected for downstream crate to implement, this is just means to reuse
/// code for [Arc] and [`Option<Arc>`][Option] variants. However, it is theoretically possible (if
/// you have your own [Arc] implementation).
///
/// It is also implemented for [Rc], but that is not considered very useful (because the
/// [ArcSwapAny] is not `Send` or `Sync`, therefore there's very little advantage for it to be
/// atomic).
///
/// # Safety
///
/// Aside from the obvious properties (like that incrementing and decrementing a reference count
/// cancel each out and that having less references tracked than how many things actually point to
/// the value is fine as long as the count doesn't drop to 0), it also must satisfy that if two
/// pointers have the same value, they point to the same object. This is specifically not true for
/// ZSTs, but it is true for `Arc`s of ZSTs, because they have the reference counts just after the
/// value. It would be fine to point to a type-erased version of the same object, though (if one
/// could use this trait with unsized types in the first place).
///
/// Furthermore, the type should be Pin (eg. if the type is cloned or moved, it should still
/// point/deref to the same place in memory).
///
/// [Arc]: std::sync::Arc
/// [Rc]: std::rc::Rc
/// [ArcSwapAny]: crate::ArcSwapAny
pub unsafe trait RefCnt: Clone {
    /// The base type the pointer points to.
    type Base;
    /// Converts the smart pointer into a raw pointer, without affecting the reference count.
    ///
    /// This can be seen as kind of freezing the pointer ‒ it'll be later converted back using
    /// [`from_ptr`](#method.from_ptr).
    ///
    /// The pointer must point to the value stored (and the value must be the same as one returned
    /// by [`as_ptr`](#method.as_ptr).
    fn into_ptr(me: Self) -> *mut Self::Base;
    /// Provides a view into the smart pointer as a raw pointer.
    ///
    /// This must not affect the reference count ‒ the pointer is only borrowed.
    fn as_ptr(me: &Self) -> *mut Self::Base;
    /// Converts a raw pointer back into the smart pointer, without affecting the reference count.
    ///
    /// This is only called on values previously returned by [`into_ptr`](#method.into_ptr).
    /// However, it is not guaranteed to be 1:1 relation ‒ `from_ptr` may be called more times than
    /// `into_ptr` temporarily provided the reference count never drops under 1 during that time
    /// (the implementation sometimes owes a reference). These extra pointers will either be
    /// converted back using `into_ptr` or forgotten.
    ///
    /// # Safety
    ///
    /// This must not be called by code outside of this crate.
    unsafe fn from_ptr(ptr: *const Self::Base) -> Self;
    /// Increments the reference count by one.
    ///
    /// Return the pointer to the inner thing as a side effect.
    fn inc(me: &Self) -> *mut Self::Base {
        Self::into_ptr(Self::clone(me))
    }
    /// Decrements the reference count by one.
    ///
    /// Note this is called on a raw pointer (one previously returned by
    /// [`into_ptr`](#method.into_ptr). This may lead to dropping of the reference count to 0 and
    /// destruction of the internal pointer.
    ///
    /// # Safety
    ///
    /// This must not be called by code outside of this crate.
    unsafe fn dec(ptr: *const Self::Base) {
        drop(Self::from_ptr(ptr));
    }
}
unsafe impl<T> RefCnt for Arc<T> {
    type Base = T;
    fn into_ptr(me: Arc<T>) -> *mut T {
        Arc::into_raw(me) as *mut T
    }
    fn as_ptr(me: &Arc<T>) -> *mut T {
        me as &T as *const T as *mut T
    }
    unsafe fn from_ptr(ptr: *const T) -> Arc<T> {
        Arc::from_raw(ptr)
    }
}
unsafe impl<T> RefCnt for Rc<T> {
    type Base = T;
    fn into_ptr(me: Rc<T>) -> *mut T {
        Rc::into_raw(me) as *mut T
    }
    fn as_ptr(me: &Rc<T>) -> *mut T {
        me as &T as *const T as *mut T
    }
    unsafe fn from_ptr(ptr: *const T) -> Rc<T> {
        Rc::from_raw(ptr)
    }
}
unsafe impl<T: RefCnt> RefCnt for Option<T> {
    type Base = T::Base;
    fn into_ptr(me: Option<T>) -> *mut T::Base {
        me.map(T::into_ptr).unwrap_or_else(ptr::null_mut)
    }
    fn as_ptr(me: &Option<T>) -> *mut T::Base {
        me.as_ref().map(T::as_ptr).unwrap_or_else(ptr::null_mut)
    }
    unsafe fn from_ptr(ptr: *const T::Base) -> Option<T> {
        if ptr.is_null() { None } else { Some(T::from_ptr(ptr)) }
    }
}
#[cfg(test)]
mod tests_rug_69 {
    use super::*;
    use crate::ref_cnt::RefCnt;
    #[test]
    fn test_inc() {
        #[derive(Debug)]
        struct MyStruct(i32);
        impl MyStruct {
            fn into_ptr(me: Self) -> *mut Self {
                Box::into_raw(Box::new(me))
            }
            fn clone(me: &Self) -> Self {
                MyStruct(me.0)
            }
        }
        struct RefCnt<'a> {
            base: &'a MyStruct,
        }
        impl RefCnt<'_> {
            fn inc(me: &Self) -> *mut MyStruct {
                Self::into_ptr(Self::clone(me.base))
            }
            fn into_ptr(me: MyStruct) -> *mut MyStruct {
                Box::into_raw(Box::new(me))
            }
            fn clone(me: &MyStruct) -> MyStruct {
                MyStruct::clone(me)
            }
        }
        let mut v51: Option<RefCnt> = Some(RefCnt { base: &MyStruct(123) });
        if let Some(p0) = v51 {
            RefCnt::inc(&p0);
        }
    }
}
#[cfg(test)]
mod tests_rug_72 {
    use super::*;
    use crate::RefCnt;
    use crate::ArcSwap;
    use std::sync::Arc;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = String::from(rug_fuzz_0);
        let p0 = Arc::new(data);
        <std::sync::Arc<String>>::as_ptr(&p0);
             }
});    }
}
