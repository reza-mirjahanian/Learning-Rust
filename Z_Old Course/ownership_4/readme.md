

- _Ownership_ is a set of rules that govern how a Rust program manages memory
- Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data;
- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there

---------
-   Each value in Rust has an  _owner_.
-   There can only be one owner at a time.
-   When the owner goes out of scope, the value will be dropped.
- ------------
In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called _Resource Acquisition Is Initialization (RAII)_. The `drop` function in Rust will be familiar to you if you’ve used RAII patterns.

there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any _automatic_ copying can be assumed to be inexpensive in terms of runtime performance

------

Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait
group of simple scalar values can implement  `Copy`, and nothing that requires allocation or is some form of resource can implement  `Copy`. Here are some of the types that implement  `Copy`:

-   All the integer types, such as  `u32`.
-   The Boolean type,  `bool`, with values  `true`  and  `false`.
-   All the floating-point types, such as  `f64`.
-   The character type,  `char`.
-   Tuples, if they only contain types that also implement  `Copy`. For example,  `(i32, i32)`  implements  `Copy`, but  `(i32, String)`  does not.
- -----
- The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

- Rust has a feature for using a value without transferring ownership, called _references_.
- Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value
--------------
A  _data race_  is similar to a race condition and happens when these three behaviors occur:

-   Two or more pointers access the same data at the same time.
-   At least one of the pointers is being used to write to the data.
-   There’s no mechanism being used to synchronize access to the data.


### [Dangling References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references)

In languages with pointers, it’s easy to erroneously create a  _dangling pointer_—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.


Let’s recap what we’ve discussed about references:

-   At any given time, you can have  _either_  one mutable reference  _or_  any number of immutable references.
-   References must always be valid.
----------------------------------
_Slices_ let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.


The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.