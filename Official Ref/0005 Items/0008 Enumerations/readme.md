https://doc.rust-lang.org/reference/items/enumerations.html

### *struct-like enum variant*,

Enum constructors can have either named or unnamed fields:

```rust
enum Animal {
    Dog(String, f64),
    Cat { name: String, weight: f64 },
}

let mut a: Animal = Animal::Dog("Cocoa".to_string(), 37.2);
a = Animal::Cat { name: "Spotty".to_string(), weight: 2.7 };

```

In this example, `Cat` is a *struct-like enum variant*, whereas `Dog` is simply called an enum variant


### unit-only enum

An enum where no constructors contain fields are called a *field-less enum*. For example, this is a fieldless enum:

```
enum Fieldless {
    Tuple(),
    Struct{},
    Unit,
}

```

If a field-less enum only contains unit variants, the enum is called an *unit-only enum*. For example:

```
enum Enum {
    Foo = 3,
    Bar = 2,
    Baz = 1,
}
```
------------

```rust
enum Examples {
    UnitLike,
    TupleLike(i32),
    StructLike { value: i32 },
}

use Examples::*; // Creates aliases to all variants.
let x = UnitLike; // Path expression of the const item.
let x = UnitLike {}; // Struct expression.
let y = TupleLike(123); // Call expression.
let y = TupleLike { 0: 123 }; // Struct expression using integer field names.
let z = StructLike { value: 123 }; // Struct expression.


``` 
[Discriminants](https://doc.rust-lang.org/reference/items/enumerations.html#discriminants)
------------------------------------------------------------------------------------------
- Explicit discriminants
- Implicit discriminants


-------------

### zero variants
```rust
enum ZeroVariants {}

// Zero-variant enums are equivalent to the never type, but they cannot be coerced into other types.

let x: ZeroVariants = panic!();
let y: u32 = x; // mismatched type error
``` 
