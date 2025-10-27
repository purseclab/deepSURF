# qwutils (WIP)  
```rust
use qwutils::*;
```

## Impls
```rust
Option {
    fn with(&self, |&T|->R) -> R; //short for as_ref().map()
    fn with_if(&self, &Option<U>, |&T,&U|->R) -> R;
    fn add/sub/mul/div_to(&self, &V);
}
bool {
    fn map(&self,||->R) -> R;
    //map_or, map_or_else also available
    fn option(&self) -> Option<()>;
    fn result(&self) -> Result<(),()>;
    //more bool impls: https://crates.io/crates/boolinator
}
Array/Tuple {
    [T; N]::to_tuple(self) -> (T, ...);
    (T, ...)::to_array(self) -> [T; N];
}
Vec {
    fn push_option(&mut self, o: Option<T>);
    fn grow_to_with(&mut self, len, ||->T);
    fn grow_to(&mut self, len, T) where T: Clone;
    fn grow_to_default(&mut self, len) where T: Default;
}
Result {
    fn expect_nodebug(&self, &str); //for T without Debug
    //expect_err, unwrap, unwrap_err also available
}
Range {
    fn len(&self) -> T where T: Sub<T>;
}
```
For most functions _mut variants are available

## Traits 

### RefClonable  
```rust
trait RefClonable {
    fn refc(&self) -> Self;
}
```

- Fast reference cloning
- Similar to Clone
- Implemented for Rc/Arc
- ```refcounted.refc()``` behaves like ```Rc/Arc::clone(&refcounted)```

### ScopedAccess (WIP)
```rust
trait ScopedAccess {
    fn access(&self, |&T|->R) -> R;
    fn access_mut(&mut self, |&mut T|->R) -> R;
}
```

- Acces inner types which are only accessible scoped
- implemented for RefCell, RwLock, Rc/Arc<RefCell/RwLock>, references, Box, ...

## Macros

### static_stor

TODO

## Functions

### if_type (WIP)
```rust
pub fn if_type<T,Specific>(||->Specific) -> T;
```
- calls the given function if T and Specific are the same type (by comparing TypeId)
- Both Types must be statically known (T: 'static, Specific: 'static)