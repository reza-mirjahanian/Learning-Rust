Rust adds the `T: Sized` bound to all generics by default. Adding `?Sized` removes that bound, allowing non-sized types too.


To clarify, the `Sized` trait means we know something's size at compile time. If you ever want to store something as a local variable the compiler needs to know how much space to set aside, and adding a `?Sized` to your where clause relaxes that restriction.

See [Dynamically Sized Types (DSTs)](https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts) from *The Nomicon* if you want to go further down the `?Sized` rabbit hole.