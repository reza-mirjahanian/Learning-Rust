https://doc.rust-lang.org/reference/items/functions.html


#### ()
If the output type is not explicitly stated, it is the [unit type](https://doc.rust-lang.org/reference/types/tuple.html).

----
#### Safe
The `safe` function is semantically only allowed when used in an [`extern` block](https://doc.rust-lang.org/reference/items/external-blocks.html).

----
#### SelfParam
If the first parameter is a *SelfParam*, this indicates that the function is a [method](https://doc.rust-lang.org/reference/items/associated-items.html#methods).

Functions with a self parameter may only appear as an [associated function](https://doc.rust-lang.org/reference/items/associated-items.html#associated-functions-and-methods) in a [trait](https://doc.rust-lang.org/reference/items/traits.html) or [implementation](https://doc.rust-lang.org/reference/items/implementations.html).

----
#### variadic

A parameter with the `...` token indicates a [variadic function](https://doc.rust-lang.org/reference/items/external-blocks.html#variadic-functions), and may only be used as the last parameter of an [external block](https://doc.rust-lang.org/reference/items/external-blocks.html) function. The variadic parameter may have an optional identifier, such as `args: ...`.



Functions without a body block are terminated with a semicolon. This form may only appear in a [trait](https://doc.rust-lang.org/reference/items/traits.html) or [external block](https://doc.rust-lang.org/reference/items/external-blocks.html).


----

When a generic function is referenced, its type is instantiated based on the context of the reference. For example, calling the `foo` function here:

```
use std::fmt::Debug;

fn foo<T>(x: &[T]) where T: Debug {
    // details elided
}

foo(&[1, 2]);

```


will instantiate type parameter `T` with `i32`.