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