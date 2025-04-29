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
