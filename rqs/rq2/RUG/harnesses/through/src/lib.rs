use std::ptr;
/// Mutate a referenced element by transferring ownership through a function.
pub fn through<T>(elem: &mut T, func: impl FnOnce(T) -> T) {
    unsafe {
        let elem_ref = elem;
        let elem = ptr::read(elem_ref);
        let elem = func(elem);
        ptr::write(elem_ref, elem);
    }
}
/// Mutate a referenced element by transferring ownership through a function, which also
/// produces an output datum which is returned from this function.
pub fn through_and<T, O>(elem: &mut T, func: impl FnOnce(T) -> (T, O)) -> O {
    unsafe {
        let elem_ref = elem;
        let elem = ptr::read(elem_ref);
        let (elem, out) = func(elem);
        ptr::write(elem_ref, elem);
        out
    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    struct TestData(i32);
    #[test]
    fn test_through_and() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = TestData(rug_fuzz_0);
        let p1 = |x: TestData| {
            let new_val = x.0 * rug_fuzz_1;
            (TestData(new_val), rug_fuzz_2)
        };
        debug_assert_eq!(crate ::through_and(& mut p0, p1), "Processed");
        debug_assert_eq!(p0.0, 10);
             }
});    }
}
