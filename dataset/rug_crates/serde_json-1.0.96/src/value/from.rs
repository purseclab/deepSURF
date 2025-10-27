use super::Value;
use crate::map::Map;
use crate::number::Number;
use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::iter::FromIterator;

macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Value {
                fn from(n: $ty) -> Self {
                    Value::Number(n.into())
                }
            }
        )*
    };
}

from_integer! {
    i8 i16 i32 i64 isize
    u8 u16 u32 u64 usize
}

#[cfg(feature = "arbitrary_precision")]
from_integer! {
    i128 u128
}

impl From<f32> for Value {
    /// Convert 32-bit floating point number to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let f: f32 = 13.37;
    /// let x: Value = f.into();
    /// ```
    fn from(f: f32) -> Self {
        Number::from_f32(f).map_or(Value::Null, Value::Number)
    }
}

impl From<f64> for Value {
    /// Convert 64-bit floating point number to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let f: f64 = 13.37;
    /// let x: Value = f.into();
    /// ```
    fn from(f: f64) -> Self {
        Number::from_f64(f).map_or(Value::Null, Value::Number)
    }
}

impl From<bool> for Value {
    /// Convert boolean to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let b = false;
    /// let x: Value = b.into();
    /// ```
    fn from(f: bool) -> Self {
        Value::Bool(f)
    }
}

impl From<String> for Value {
    /// Convert `String` to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let s: String = "lorem".to_string();
    /// let x: Value = s.into();
    /// ```
    fn from(f: String) -> Self {
        Value::String(f)
    }
}

impl<'a> From<&'a str> for Value {
    /// Convert string slice to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let s: &str = "lorem";
    /// let x: Value = s.into();
    /// ```
    fn from(f: &str) -> Self {
        Value::String(f.to_string())
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    /// Convert copy-on-write string to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Borrowed("lorem");
    /// let x: Value = s.into();
    /// ```
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Owned("lorem".to_string());
    /// let x: Value = s.into();
    /// ```
    fn from(f: Cow<'a, str>) -> Self {
        Value::String(f.into_owned())
    }
}

impl From<Number> for Value {
    /// Convert `Number` to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::{Number, Value};
    ///
    /// let n = Number::from(7);
    /// let x: Value = n.into();
    /// ```
    fn from(f: Number) -> Self {
        Value::Number(f)
    }
}

impl From<Map<String, Value>> for Value {
    /// Convert map (with string keys) to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::{Map, Value};
    ///
    /// let mut m = Map::new();
    /// m.insert("Lorem".to_string(), "ipsum".into());
    /// let x: Value = m.into();
    /// ```
    fn from(f: Map<String, Value>) -> Self {
        Value::Object(f)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    /// Convert a `Vec` to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// ```
    fn from(f: Vec<T>) -> Self {
        Value::Array(f.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Value {
    /// Convert a slice to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: &[&str] = &["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// ```
    fn from(f: &'a [T]) -> Self {
        Value::Array(f.iter().cloned().map(Into::into).collect())
    }
}

impl<T: Into<Value>> FromIterator<T> for Value {
    /// Convert an iteratable type to a `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v = std::iter::repeat(42).take(5);
    /// let x: Value = v.collect();
    /// ```
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: Vec<_> = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into_iter().collect();
    /// ```
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use serde_json::Value;
    ///
    /// let x: Value = Value::from_iter(vec!["lorem", "ipsum", "dolor"]);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Value::Array(iter.into_iter().map(Into::into).collect())
    }
}

impl<K: Into<String>, V: Into<Value>> FromIterator<(K, V)> for Value {
    /// Convert an iteratable type to a `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: Vec<_> = vec![("lorem", 40), ("ipsum", 2)];
    /// let x: Value = v.into_iter().collect();
    /// ```
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Value::Object(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl From<()> for Value {
    /// Convert `()` to `Value`
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let u = ();
    /// let x: Value = u.into();
    /// ```
    fn from((): ()) -> Self {
        Value::Null
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => Value::Null,
            Some(value) => Into::into(value),
        }
    }
}

#[cfg(test)]
mod tests_rug_715 {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_from() {
        let p0: i8 = 42;

        Value::from(p0);
    }
}

#[cfg(test)]
mod tests_rug_716 {
    use super::*;
    use crate::value::Value;
    use std::convert::From;

    #[test]
    fn test_from() {
        let p0: i16 = 42;

        let result: Value = <Value as From<i16>>::from(p0);
        
        // Add assertions here
        
    }
}#[cfg(test)]
mod tests_rug_717 {
    use super::*;
    use crate::value::Value;
    
    #[test]
    fn test_rug() {
        let p0: i32 = 42;
        
        Value::from(p0);
    }
}#[cfg(test)]
mod tests_rug_718 {
    use super::*;
    use crate::value::Value;
    use std::convert::From;
    
    #[test]
    fn test_from() {
        let p0: i64 = 42;
        let _ = <Value as std::convert::From<i64>>::from(p0);
    }
}#[cfg(test)]
mod tests_rug_720 {
    use super::*;
    use crate::value::Value;
    use std::convert::From;

    #[test]
    fn test_from() {
        let p0: u8 = 42;

        let result: Value = From::<u8>::from(p0);
    }
}#[cfg(test)]
mod tests_rug_722 {
    use super::*;
    use crate::value::Value;
    
    #[test]
    fn test_from() {
        let p0: u32 = 42;
        Value::from(p0);
    }
}
#[cfg(test)]
mod tests_rug_724 {
    use super::*;
    use crate::value::{self, Number};
    
    #[test]
    fn test_rug() {
        let mut p0: usize = 42;
        
        <value::Value as std::convert::From<usize>>::from(p0);
        
        // additional assertions if necessary
    }
}
#[cfg(test)]
mod tests_rug_725 {
    use super::*;
    use crate::value::Value;
    
    #[test]
    fn test_from() {
        let p0: f32 = 13.37;
        
        <Value>::from(p0);
    }
}        
        #[cfg(test)]
        mod tests_rug_726 {
            use super::*;
            use crate::value::Value;
            use crate::value::Number;
            
            #[test]
            fn test_rug() {
                let mut p0 = 3.14;
                <Value as std::convert::From<f64>>::from(p0);
            }
        }
                            
#[cfg(test)]
mod tests_rug_727 {
    use super::*;
    use crate::Value;
    
    #[test]
    fn test_from() {
        let p0: bool = false;
        let result: Value = Value::from(p0);
    }
    
}

#[cfg(test)]
mod tests_rug_728 {
    use super::*;
    use crate::value::Value;
    use std::convert::From;
    
    #[test]
    fn test_from() {
        let p0: std::string::String = "lorem".to_string();
        
        <Value as std::convert::From<std::string::String>>::from(p0);
    }
}
  
#[cfg(test)]
mod tests_rug_729 {
    use super::*;
    use crate::Value;
    
    #[test]
    fn test_from() {
        let p0: &str = "lorem";
        
        <Value as std::convert::From<&str>>::from(&p0);
    }
}
#[cfg(test)]
mod tests_rug_730 {

    use crate::value::from::Cow;
    use crate::Value;
    use std::convert::From;

    #[test]
    fn test_from() {
        let p0: Cow<'static, str> = Cow::Borrowed("lorem");
        let x: Value = Value::from(p0);

        assert_eq!(x, Value::String("lorem".to_string()));

        let p0: Cow<'static, str> = Cow::Owned("lorem".to_string());
        let x: Value = Value::from(p0);

        assert_eq!(x, Value::String("lorem".to_string()));
    }
}
#[cfg(test)]
mod tests_rug_731 {
    use super::*;
    use crate::{Number, Value};
    
    #[test]
    fn test_from() {
        let p0: Number = Number::from_f32(3.14159).unwrap().into();
        
        let result: Value = <Value>::from(p0);
        
        // Add assertions here
    }
}#[cfg(test)]
mod tests_rug_732 {
    use super::*;
    use crate::{Map, Value};
    use crate::value;
    use std::collections::BTreeMap;

    #[test]
    fn test_from() {
        let mut p0: Map<String, Value> = Map::new();
        p0.insert("Lorem".to_string(), "ipsum".into());

        <Value as std::convert::From<Map<String, Value>>>::from(p0);
    }
}#[cfg(test)]
mod tests_rug_734 {
    use super::*;
    use crate::Value;

    #[test]
    fn test_from_slice_to_value() {
        let p0: &'static [Value] = &[];

        <Value as std::convert::From<&[Value]>>::from(p0);
    }
}
#[cfg(test)]
mod tests_rug_736 {
    use super::*;
    use crate::{Value, Map};
    use crate::value;
    
    #[test]
    fn test_rug() {
        let mut p0: Map<String, Value> = Map::new();

        <Value>::from_iter::<Map<String, Value>>(p0);

    }
}
#[cfg(test)]
mod tests_rug_738 {
    use super::*;
    use crate::value;
    use std::convert::From;
    
    #[test]
    fn test_from() {
        let mut v126: Option<value::Value> = Some(value::Value::Null);
        <value::Value>::from(v126);
    }
}