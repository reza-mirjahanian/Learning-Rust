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

