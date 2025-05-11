

# Rust `#[derive]` Attribute Technical Reference

## 1. Basic Concepts

### 1.1 Definition and Purpose

The `#[derive]` attribute in Rust is a meta-programming feature that automatically implements specific traits for custom data types. It's a form of code generation that saves developers from writing boilerplate implementations.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
```

### 1.2 Syntax and Placement

```rust
#[derive(Trait1, Trait2, ...)]
```

- Must be placed before the item definition (struct, enum, or union)
- Multiple traits can be derived in a single attribute
- Cannot be applied to functions, impl blocks, or trait definitions

## 2. Standard Library Derivable Traits

### 2.1 Core Derivable Traits

| Trait | Purpose | Requirements | Memory Impact |
|-------|---------|-------------|---------------|
| `Clone` | Explicit value duplication | All fields must implement `Clone` | None |
| `Copy` | Implicit value duplication | All fields must implement `Copy` | None |
| `Debug` | Text representation for debugging | All fields must implement `Debug` | None |
| `Default` | Create default instance | All fields must implement `Default` | None |
| `Hash` | Convert instance to hash value | All fields must implement `Hash` | None |
| `PartialEq` | Equality comparison | All fields must implement `PartialEq` | None |
| `Eq` | Reflexive equality | Requires `PartialEq` | None |
| `PartialOrd` | Ordering comparison | All fields must implement `PartialOrd` | None |
| `Ord` | Total ordering | Requires `PartialOrd` and `Eq` | None |

### 2.2 Example: Deriving Multiple Traits

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // Copy semantics
    let p3 = p1.clone(); // Explicit clone
    
    println!("{:?}", p1); // Debug printing
    println!("{}", p1 == p3); // PartialEq
    
    let default_point = Point::default(); // Default implementation
    assert_eq!(default_point.x, 0);
    assert_eq!(default_point.y, 0);
    
    let mut points = vec![Point { x: 3, y: 2 }, Point { x: 1, y: 5 }];
    points.sort(); // Ord implementation
    println!("{:?}", points); // Prints in sorted order
}
```

## 3. Internal Implementation

### 3.1 Macro Expansion Mechanism

The `#[derive]` attribute is a procedural macro that expands during compilation. For standard library traits, the compiler has built-in implementations.

```rust
// When you write:
#[derive(Debug)]
struct Point { x: f64, y: f64 }

// The compiler effectively generates:
struct Point { x: f64, y: f64 }

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
```

### 3.2 Memory Representation

Derived traits do not alter the memory layout of types. They only add implementations without adding fields or changing the size of the type.

```rust
use std::mem::size_of;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point { x: i32, y: i32 }

struct PointNoDerive { x: i32, y: i32 }

fn main() {
    assert_eq!(size_of::<Point>(), size_of::<PointNoDerive>());
    // Both are exactly 8 bytes (on a typical 64-bit system)
}
```

## 4. Advanced Features and Capabilities

### 4.1 Custom Derive Macros

Rust allows creating custom derive macros to implement traits automatically:

```rust
// In a proc-macro crate
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl MyTrait for #name {
            fn my_method(&self) -> String {
                format!("Instance of {}", stringify!(#name))
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

Using a custom derive:

```rust
#[derive(MyTrait)]
struct CustomType {
    // fields
}

// This expands to:
// impl MyTrait for CustomType {
//     fn my_method(&self) -> String {
//         format!("Instance of {}", "CustomType")
//     }
// }
```

### 4.2 Derive Attributes

Custom derive macros can accept additional attributes that modify their behavior:

```rust
#[derive(MyTrait)]
#[my_trait(skip_fields = "password")]
struct User {
    username: String,
    password: String,
}
```

Implementation example:

```rust
#[proc_macro_derive(MyTrait, attributes(my_trait))]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    // Parse attributes and generate implementation accordingly
}
```

## 5. Edge Cases and Limitations

### 5.1 Generic Types and Trait Bounds

When deriving traits for generic types, the generated implementation includes necessary trait bounds:

```rust
#[derive(Clone, Debug)]
struct GenericPoint<T> {
    x: T,
    y: T,
}

// Expands to:
// impl<T: Clone> Clone for GenericPoint<T> {
//    // Clone implementation
// }
// 
// impl<T: Debug> Debug for GenericPoint<T> {
//    // Debug implementation
// }
```

### 5.2 Recursive Types

For recursive types, some derived traits need special attention:

```rust
#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Works fine for Clone and Debug
// Could cause stack overflow for naive recursion-based implementations
```

### 5.3 Zero-Sized Types

Derive works on zero-sized types, though some traits may behave specially:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Empty;

// All these implementations are valid but some are trivial
```

## 6. Deriving Traits for Different Type Definitions

### 6.1 Structs

```rust
// Regular structs
#[derive(Debug, Clone)]
struct Regular {
    field1: i32,
    field2: String,
}

// Tuple structs
#[derive(Debug, Clone)]
struct Tuple(i32, String);

// Unit structs
#[derive(Debug, Clone)]
struct Unit;
```

### 6.2 Enums

```rust
#[derive(Debug, Clone, PartialEq)]
enum Status {
    Active,
    Inactive,
    Pending { reason: String },
    Processing(u32),
}
```

Generated implementations handle all variants appropriately.

### 6.3 Unions

```rust
#[derive(Copy, Clone)]
union IntOrFloat {
    i: i32,
    f: f32,
}
```

Note: Fewer traits can be derived for unions. `Debug`, `PartialEq`, etc., cannot be automatically derived for unions.

## 7. Visibility and Scope Implications

### 7.1 Visibility Rules

Derived trait implementations follow these visibility rules:

- The derived implementation is always visible wherever the type is visible
- If a type has private fields, derived traits may still expose information about those fields:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    private_id: u64, // Private but still shows in Debug output
}
```

### 7.2 Trait Bounds in Generic Contexts

```rust
#[derive(Clone)]
struct Wrapper<T> {
    value: T,
}

// The following won't compile unless T: Clone
fn duplicate<T>(w: Wrapper<T>) -> (Wrapper<T>, Wrapper<T>) {
    let clone = w.clone();
    (w, clone)
}
```

## 8. Detailed Behavior of Standard Derivable Traits

### 8.1 `Clone` and `Copy`

- `Clone`: Calls `.clone()` on each field
- `Copy`: Marker trait with no methods, requires all fields to be `Copy`

```rust
#[derive(Clone, Copy)]
struct Point {
    x: f64, // f64 implements Copy
    y: f64, // f64 implements Copy
}

// Won't compile - String doesn't implement Copy
// #[derive(Copy)]
// struct Person {
//     name: String,
// }
```

### 8.2 `Debug`

- Formats structs as `NameOfType { field: value, ... }`
- Formats tuple structs as `NameOfType(value, ...)`
- Formats enums as `VariantName` or `VariantName { field: value, ... }` or `VariantName(value, ...)`

### 8.3 `PartialEq`, `Eq`, `PartialOrd`, and `Ord`

- `PartialEq`: Compares each field for equality
- `Eq`: Just a marker trait that requires `PartialEq`
- `PartialOrd`: Compares fields in declaration order
- `Ord`: Ensures total ordering based on field declaration order

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,   // Compared first
    age: u8,        // Compared second (only if names are equal)
}

let p1 = Person { name: "Alice".into(), age: 30 };
let p2 = Person { name: "Bob".into(), age: 25 };
let p3 = Person { name: "Alice".into(), age: 35 };

// p1 < p2 is false (A comes before B)
// p1 < p3 is true (same name, but 30 < 35)
```

### 8.4 `Hash`

- Calls `.hash()` on each field in declaration order
- Used for hashtable lookups

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Key {
    id: u32,
    timestamp: u64,
}

fn main() {
    let mut map = HashMap::new();
    map.insert(Key { id: 1, timestamp: 123 }, "value");
}
```

### 8.5 `Default`

- Creates an instance with the default value of each field
- Fields must implement `Default`

```rust
#[derive(Default)]
struct Configuration {
    timeout: Option<u32>,      // Default is None
    max_connections: usize,    // Default is 0
    hostname: String,          // Default is empty string
}

let config = Configuration::default();
// All fields have their default values
```

## 9. Custom Derive with Attribute Parameters

### 9.1 Example with Builder Pattern

```rust
// In a proc-macro crate
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    // Implementation
}

// Usage
#[derive(Builder)]
struct Command {
    executable: String,
    #[builder(default = "10")]
    timeout: u32,
    #[builder(each = "arg")]
    args: Vec<String>,
}
```

## 10. Common Design Patterns with Derive

### 10.1 Newtype Pattern

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Username(String);

// Type safety through newtypes with automatically derived traits
```

### 10.2 Type State Pattern

```rust
#[derive(Debug, Clone)]
struct Draft;

#[derive(Debug, Clone)]
struct Published;

#[derive(Debug, Clone)]
struct Post<State> {
    content: String,
    state: std::marker::PhantomData<State>,
}
```

## 11. Comparison with Other Languages

| Language | Feature | Similarities | Differences |
|----------|---------|-------------|-------------|
| Java | Annotations | Both use metadata to generate code | Java annotations require explicit processor setup |
| C# | Attributes | Both support attributes for code generation | C# attributes often use reflection at runtime |
| Kotlin | Data classes | Both generate common methods | Kotlin's is built into the language, not an attribute |
| TypeScript | Decorators | Both modify class behavior | TypeScript decorators run at runtime |
| Scala | Case classes | Both generate equality, debug methods | Case classes are their own construct, not an attribute |
| Haskell | Deriving | Most similar concept | Haskell's system is based on typeclasses |

## 12. Troubleshooting and Gotchas

### 12.1 Common Compilation Errors

```rust
// Error: the trait `Copy` may not be implemented for this type
#[derive(Copy)]
struct HasString {
    s: String, // String doesn't implement Copy
}

// Error: the trait `Default` cannot be derived for struct
#[derive(Default)]
struct NoDefault {
    file: std::fs::File, // File doesn't implement Default
}
```

### 12.2 Derive Ordering Dependencies

Some derives depend on others:
- `Eq` requires `PartialEq`
- `Ord` requires `PartialOrd` and `Eq`
- `Copy` requires `Clone`

```rust
// Correct ordering with dependencies
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate(i32, i32);
```

## 13. Performance Considerations

### 13.1 Compile-Time vs. Runtime Costs

- Derive macros increase compile time but have no runtime cost
- Generated implementations may not be optimized for specific use cases
- For hot paths, manual implementations can sometimes be more efficient

### 13.2 Code Size Impact

```rust
// Each derived trait adds implementation code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point { x: i32, y: i32 }

// Increases binary size compared to:
struct PointMinimal { x: i32, y: i32 }
```

## 14. Best Practices and Tips

### 14.1 When to Use Derive

- Use for standard, predictable implementations
- Consider manual implementation for:
  - Custom equality semantics
  - Performance-critical code
  - Types with special behavior requirements

### 14.2 Recommended Trait Combinations

```rust
// Value types (small, copyable)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate { x: i32, y: i32 }

// Data types (general purpose)
#[derive(Debug, Clone, PartialEq, Eq)]
struct Person { name: String, age: u8 }

// For collections/ordering
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SortableItem { priority: u8, name: String }
```

### 14.3 Documentation with Derive

```rust
/// A 2D point in Cartesian space.
///
/// Implements:
/// - `Debug` for string representation
/// - `Clone` and `Copy` for value semantics
/// - `PartialEq` and `Eq` for equality comparisons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```

## 15. Advanced Implementation Details

### 15.1 Derive Macro Hygiene

Custom derive macros must be careful about name conflicts:

```rust
// Good practice in derive macros
let expanded = quote! {
    impl #trait_name for #struct_name {
        fn method(&self) -> ::std::option::Option<::std::string::String> {
            // Using fully qualified paths
        }
    }
};
```

### 15.2 Conditional Derive

You can use conditional compilation with derive:

```rust
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
struct Config {
    // fields
}
```

This applies `Serialize` and `Deserialize` only when the "serde" feature is enabled.