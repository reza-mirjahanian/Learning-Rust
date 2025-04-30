A *trait* describes an abstract interface that types can implement. This interface consists of [associated items](https://doc.rust-lang.org/reference/items/associated-items.html), which come in three varieties:

-   [functions](https://doc.rust-lang.org/reference/items/associated-items.html#associated-functions-and-methods)
-   [types](https://doc.rust-lang.org/reference/items/associated-items.html#associated-types)
-   [constants](https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants)

```rust
Syntax
Trait :
   unsafe? trait IDENTIFIER  GenericParams? ( : TypeParamBounds? )? WhereClause? {
     InnerAttribute*
     AssociatedItem*
   }
```

------------------------------
The trait declaration defines a trait in the [type namespace](https://doc.rust-lang.org/reference/names/namespaces.html) of the module or block where it is located.

Associated items are defined as members of the trait within their respective namespaces. Associated types are defined in the type namespace. Associated constants and associated functions are defined in the value namespace.


------------------------------


All traits define an implicit type parameter `Self` that refers to "the type that is implementing this interface". Traits may also contain additional type parameters. These type parameters, including `Self`, may be constrained by other traits and so forth [as usual](https://doc.rust-lang.org/reference/items/generics.html).

Traits are implemented for specific types through separate [implementations](https://doc.rust-lang.org/reference/items/implementations.html).

------------------------------
Trait functions may omit the function body by replacing it with a semicolon. This indicates that the implementation must define the function. If the trait function defines a body, this definition acts as a default for any implementation which does not override it. Similarly, associated constants may omit the equals sign and expression to indicate implementations must define the constant value. **Associated types** must never define the type, the type may only be specified in an implementation

```rust
// Examples of associated trait items with and without definitions.
trait Example {
    const CONST_NO_DEFAULT: i32;
    const CONST_WITH_DEFAULT: i32 = 99;
    type TypeNoDefault;
    fn method_without_default(&self);
    fn method_with_default(&self) {}
}
``` 
Trait functions are not allowed to be [`const`](https://doc.rust-lang.org/reference/items/functions.html#const-functions).


------------------------------

