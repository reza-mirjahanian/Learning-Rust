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


[Attributes on functions](https://doc.rust-lang.org/reference/items/functions.html#attributes-on-functions)
-----------------------------------------------------------------------------------------------------------

This example shows an inner attribute on a function. The function is documented with just the word "Example".

```
fn documented() {
    #![doc = "Example"]
}

```

> Note: Except for lints, it is idiomatic to only use outer attributes on function items.



The attributes that have meaning on a function are [`cfg`](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute), [`cfg_attr`](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute), [`deprecated`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-deprecated-attribute), [`doc`](https://doc.rust-lang.org/rustdoc/the-doc-attribute.html), [`export_name`](https://doc.rust-lang.org/reference/abi.html#the-export_name-attribute), [`link_section`](https://doc.rust-lang.org/reference/abi.html#the-link_section-attribute), [`no_mangle`](https://doc.rust-lang.org/reference/abi.html#the-no_mangle-attribute), [the lint check attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html#lint-check-attributes), [`must_use`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute), [the procedural macro attributes](https://doc.rust-lang.org/reference/procedural-macros.html), [the testing attributes](https://doc.rust-lang.org/reference/attributes/testing.html), and [the optimization hint attributes](https://doc.rust-lang.org/reference/attributes/codegen.html#optimization-hints). Functions also accept attributes macros.



[Attributes on function parameters](https://doc.rust-lang.org/reference/items/functions.html#attributes-on-function-parameters)
-------------------------------------------------------------------------------------------------------------------------------


[Outer attributes](https://doc.rust-lang.org/reference/attributes.html) are allowed on function parameters and the permitted [built-in attributes](https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index) are restricted to `cfg`, `cfg_attr`, `allow`, `warn`, `deny`, and `forbid`.

```
fn len(
    #[cfg(windows)] slice: &[u16],
    #[cfg(not(windows))] slice: &[u8],
) -> usize {
    slice.len()
}


```


Inert helper attributes used by procedural macro attributes applied to items are also allowed but be careful to not include these inert attributes in your final `TokenStream`.

For example, the following code defines an inert `some_inert_attribute` attribute that is not formally defined anywhere and the `some_proc_macro_attribute` procedural macro is responsible for detecting its presence and removing it from the output token stream.

```
#[some_proc_macro_attribute]
fn foo_oof(#[some_inert_attribute] arg: u8) {
}
```