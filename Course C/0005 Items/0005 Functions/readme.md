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


### Const functions



Functions qualified with the `const` keyword are [const functions](https://doc.rust-lang.org/reference/const_eval.html#const-functions), as are [tuple struct](https://doc.rust-lang.org/reference/items/structs.html) and [tuple variant](https://doc.rust-lang.org/reference/items/enumerations.html) constructors. *Const functions* can be called from within [const contexts](https://doc.rust-lang.org/reference/const_eval.html#const-context).



Const functions may use the [`extern`](https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier) function qualifier.



Const functions are not allowed to be [async](https://doc.rust-lang.org/reference/items/functions.html#async-functions).


### Async functions

Functions may be qualified as async, and this can also be combined with the `unsafe` qualifier:

```
async fn regular_example() { }
async unsafe fn unsafe_example() { }

```


Async functions do no work when called: instead, they capture their arguments into a future. When polled, that future will execute the function's body.


An async function is roughly equivalent to a function that returns [`impl Future`](https://doc.rust-lang.org/reference/types/impl-trait.html) and with an [`async move` block](https://doc.rust-lang.org/reference/expressions/block-expr.html#async-blocks) as its body:

```
// Source
async fn example(x: &str) -> usize {
    x.len()
}
```

is roughly equivalent to:

```
// Desugared
fn example<'a>(x: &'a str) -> impl Future<Output = usize> + 'a {
    async move { x.len() }
}
```

The actual desugaring is more complex:



-   The return type in the desugaring is assumed to capture all lifetime parameters from the `async fn` declaration. This can be seen in the desugared example above, which explicitly outlives, and hence captures, `'a`.



-   The [`async move` block](https://doc.rust-lang.org/reference/expressions/block-expr.html#async-blocks) in the body captures all function parameters, including those that are unused or bound to a `_` pattern. This ensures that function parameters are dropped in the same order as they would be if the function were not async, except that the drop occurs when the returned future has been fully awaited.

For more information on the effect of async, see [`async` blocks](https://doc.rust-lang.org/reference/expressions/block-expr.html#async-blocks).