CBox
====
This library provides a `CBox` struct, which provides a uniform API for C pointers
that are owned by Rust types. It simply calls the C destructor when it falls out of
scope.
