https://doc.rust-lang.org/reference/items/implementations.html

There are two types of implementations:

-   inherent implementations
-   [trait](https://doc.rust-lang.org/reference/items/traits.html) implementations

The nominal type is called the *implementing type* and the associable items are the *associated items* to the implementing type.


Type and const parameters must always constrain the implementation. Lifetimes must constrain the implementation if the lifetime is used in an associated type.

Examples of constraining situations:

```Rust 

// T constrains by being an argument to GenericTrait.
impl<T> GenericTrait<T> for i32 { /* ... */ }

// T constrains by being an argument to GenericStruct
impl<T> Trait for GenericStruct<T> { /* ... */ }

// Likewise, N constrains by being an argument to ConstGenericStruct
impl<const N: usize> Trait for ConstGenericStruct<N> { /* ... */ }

// T constrains by being in an associated type in a bound for type `U` which is
// itself a generic parameter constraining the trait.
impl<T, U> GenericTrait<U> for u32 where U: HasAssocType<Ty = T> { /* ... */ }

// Like previous, except the type is `(U, isize)`. `U` appears inside the type
// that includes `T`, and is not the type itself.
impl<T, U> GenericStruct<U> where (U, isize): HasAssocType<Ty = T> { /* ... */ }


```

Examples of non-constraining situations:

```Rust

// The rest of these are errors, since they have type or const parameters that
// do not constrain.

// T does not constrain since it does not appear at all.
impl<T> Struct { /* ... */ }

// N does not constrain for the same reason.
impl<const N: usize> Struct { /* ... */ }

// Usage of T inside the implementation does not constrain the impl.
impl<T> Struct {
    fn uses_t(t: &T) { /* ... */ }
}

// T is used as an associated type in the bounds for U, but U does not constrain.
impl<T, U> Struct where U: HasAssocType<Ty = T> { /* ... */ }

// T is used in the bounds, but not as an associated type, so it does not constrain.
impl<T, U> GenericTrait<U> for u32 where U: GenericTrait<T> {}

```

Example of an allowed unconstraining lifetime parameter:

```Rust
impl<'a> Struct {}

```

Example of a disallowed unconstraining lifetime parameter:

```Rust
impl<'a> HasAssocType for Struct {
    type Ty = &'a Struct;
}

```