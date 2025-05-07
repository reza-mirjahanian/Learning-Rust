```hs
Syntax
TypeAlias :
   type IDENTIFIER GenericParams? ( : TypeParamBounds )? WhereClause? ( = Type WhereClause?)? ;
```
A *type alias* defines a new name for an existing [type](https://doc.rust-lang.org/reference/types.html) in the [type namespace](https://doc.rust-lang.org/reference/names/namespaces.html) of the module or block where it is located. Type aliases are declared with the keyword `type`. Every value has a single, specific type, but may implement several different traits, and may be compatible with several different type constraints.

For example, the following defines the type `Point` as a synonym for the type `(u8, u8)`, the type of pairs of unsigned 8 bit integers:

```
type Point = (u8, u8);
let p: Point = (41, 68);
```

----------------

A type alias to a tuple-struct or unit-struct cannot be used to qualify that type's constructor:

```rust
struct MyStruct(u32);

use MyStruct as UseAlias;
type TypeAlias = MyStruct;

let _ = UseAlias(5); // OK
let _ = TypeAlias(5); // Doesn't work

``` 

A type alias, when not used as an [associated type](https://doc.rust-lang.org/reference/items/associated-items.html#associated-types), must include a [*Type*](https://doc.rust-lang.org/reference/types.html#type-expressions) and may not include [*TypeParamBounds*](https://doc.rust-lang.org/reference/trait-bounds.html).



A type alias, when used as an [associated type](https://doc.rust-lang.org/reference/items/associated-items.html#associated-types) in a [trait](https://doc.rust-lang.org/reference/items/traits.html), must not include a [*Type*](https://doc.rust-lang.org/reference/types.html#type-expressions) specification but may include [*TypeParamBounds*](https://doc.rust-lang.org/reference/trait-bounds.html).



A type alias, when used as an [associated type](https://doc.rust-lang.org/reference/items/associated-items.html#associated-types) in a [trait impl](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementations), must include a [*Type*](https://doc.rust-lang.org/reference/types.html#type-expressions) specification and may not include [*TypeParamBounds*](https://doc.rust-lang.org/reference/trait-bounds.html).



Where clauses before the equals sign on a type alias in a [trait impl](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementations) (like `type TypeAlias<T> where T: Foo = Bar<T>`) are deprecated. Where clauses after the equals sign (like `type TypeAlias<T> = Bar<T> where T: Foo`) are preferred.