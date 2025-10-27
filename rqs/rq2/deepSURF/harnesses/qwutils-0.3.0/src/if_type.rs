use std::any::TypeId;

#[inline]
pub fn if_type<Specific: 'static, T: 'static>(f: impl FnOnce() -> Specific) -> Option<T> {
    if TypeId::of::<T>() == TypeId::of::<Specific>() {
        let typed = f();
        let erased: T = unsafe{ std::mem::transmute_copy(&typed) };
        std::mem::forget(typed);
        Some(erased)
    }else{
        None
    }
}