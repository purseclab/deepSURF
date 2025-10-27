use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use super::*;
pub trait NotEmpty: Sized {
    fn _is_empty(&self) -> bool;
    #[inline]
    fn not_empty(self) -> Option<Self> {
        self._is_empty().map(|| self)
    }
}
impl<T> NotEmpty for &Vec<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<T> NotEmpty for &mut Vec<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<T> NotEmpty for Vec<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl NotEmpty for &str {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl NotEmpty for &mut str {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl NotEmpty for &String {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl NotEmpty for &mut String {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl NotEmpty for String {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<T> NotEmpty for &HashSet<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<T> NotEmpty for &mut HashSet<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<T> NotEmpty for HashSet<T> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<K, V> NotEmpty for &HashMap<K, V> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<K, V> NotEmpty for &mut HashMap<K, V> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<K, V> NotEmpty for HashMap<K, V> {
    #[inline]
    fn _is_empty(&self) -> bool {
        self.is_empty()
    }
}
#[cfg(test)]
mod tests_rug_213 {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_213_rrrruuuugggg_test_rug = 0;
        let mut p0: HashSet<i32> = HashSet::new();
        crate::not_empty::NotEmpty::not_empty(p0);
        let _rug_ed_tests_rug_213_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_214 {
    use super::*;
    use crate::NotEmpty;
    use std::vec::Vec;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v31: Vec<i32> = Vec::new();
        v31.push(rug_fuzz_0);
        debug_assert!(! v31._is_empty());
             }
});    }
}
#[cfg(test)]
mod tests_rug_215 {
    use super::*;
    use crate::NotEmpty;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_215_rrrruuuugggg_test_rug = 0;
        let mut v31: std::vec::Vec<i32> = std::vec::Vec::new();
        v31._is_empty();
        let _rug_ed_tests_rug_215_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_216 {
    use super::*;
    use crate::NotEmpty;
    use std::vec::Vec;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_216_rrrruuuugggg_test_rug = 0;
        let mut p0: Vec<i32> = Vec::new();
        <Vec<i32> as NotEmpty>::_is_empty(&p0);
        let _rug_ed_tests_rug_216_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_217 {
    use super::*;
    use crate::NotEmpty;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &str = rug_fuzz_0;
        debug_assert_eq!(p0._is_empty(), false);
             }
});    }
}
#[cfg(test)]
mod tests_rug_218 {
    use super::*;
    use crate::NotEmpty;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &mut str = &mut String::from(rug_fuzz_0);
        debug_assert_eq!(< & mut str as NotEmpty > ::_is_empty(& p0), false);
             }
});    }
}
#[cfg(test)]
mod tests_rug_219 {
    use super::*;
    use crate::NotEmpty;
    use std::string::String;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let sample_string: String = String::from(rug_fuzz_0);
        let p0: &String = &sample_string;
        p0._is_empty();
             }
});    }
}
#[cfg(test)]
mod tests_rug_220 {
    use super::*;
    use crate::NotEmpty;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &mut std::string::String = &mut String::from(rug_fuzz_0);
        p0._is_empty();
             }
});    }
}
#[cfg(test)]
mod tests_rug_221 {
    use super::*;
    use crate::NotEmpty;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: std::string::String = String::from(rug_fuzz_0);
        p0._is_empty();
             }
});    }
}
#[cfg(test)]
mod tests_rug_222 {
    use super::*;
    use crate::NotEmpty;
    use std::collections::HashSet;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_222_rrrruuuugggg_test_rug = 0;
        let mut p0: &HashSet<i32> = &HashSet::new();
        p0._is_empty();
        let _rug_ed_tests_rug_222_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_223 {
    use super::*;
    use std::collections::HashSet;
    use crate::NotEmpty;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_223_rrrruuuugggg_test_rug = 0;
        let mut p0: HashSet<i32> = HashSet::new();
        p0._is_empty();
        let _rug_ed_tests_rug_223_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_224 {
    use super::*;
    use crate::NotEmpty;
    use std::collections::HashSet;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_224_rrrruuuugggg_test_rug = 0;
        let mut p0: HashSet<i32> = HashSet::new();
        p0._is_empty();
        let _rug_ed_tests_rug_224_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_225 {
    use super::*;
    use crate::NotEmpty;
    use crate::not_empty::HashMap;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v71: HashMap<&str, &str> = HashMap::new();
        v71.insert(rug_fuzz_0, rug_fuzz_1);
        v71.insert(rug_fuzz_2, rug_fuzz_3);
        debug_assert!(! v71._is_empty());
             }
});    }
}
#[cfg(test)]
mod tests_rug_226 {
    use super::*;
    use crate::NotEmpty;
    use crate::not_empty::HashMap;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: HashMap<_, _> = HashMap::new();
        p0.insert(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(p0._is_empty(), false);
        p0.clear();
        debug_assert_eq!(p0._is_empty(), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_227 {
    use super::*;
    use crate::NotEmpty;
    use crate::not_empty::HashMap;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: HashMap<&str, &str> = HashMap::new();
        p0.insert(rug_fuzz_0, rug_fuzz_1);
        p0.insert(rug_fuzz_2, rug_fuzz_3);
        debug_assert!(! < HashMap < & str, & str > as NotEmpty > ::_is_empty(& p0));
             }
});    }
}
