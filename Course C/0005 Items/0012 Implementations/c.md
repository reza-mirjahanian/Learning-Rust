# Rust Implementation Technical Reference Guide



---

## Table of Contents

1. [Basic Syntax and Structure](#basic-syntax-and-structure)
2. [Function Implementations (`impl` blocks)](#function-implementations-impl-blocks)
3. [Method Implementations and `self`](#method-implementations-and-self)
4. [Associated Constants and Types](#associated-constants-and-types)
5. [Generic Implementations](#generic-implementations)
6. [Trait Implementations (`impl Trait`)](#trait-implementations-impl-trait)
7. [Blanket Implementations](#blanket-implementations)
8. [Private vs Public Implementations](#private-vs-public-implementations)
9. [Visibility Rules and Scoping](#visibility-rules-and-scoping)
10. [Attributes and Modifiers](#attributes-and-modifiers)
11. [Internal Representation and Memory Layout](#internal-representation-and-memory-layout)
12. [Lesser-Known Features and Edge Cases](#lesser-known-features-and-edge-cases)
13. [Gotchas, Tips, and Tricks](#gotchas-tips-and-tricks)
14. [Comparison with Other Languages](#comparison-with-other-languages)

---

## 1. Basic Syntax and Structure

### Definition
Rust uses `impl` blocks to associate functions or methods with structs, enums, or traits.

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}
```

### Key Points:
- Functions inside `impl` blocks are called **associated functions**.
- They can be static (no `self`) or instance methods (with `self`).

---

## 2. Function Implementations (`impl` blocks)

### Associated Functions
Functions not taking `self` as a parameter.

```rust
impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}
```

Call via `Point::origin()`.

### Static Methods
Same as associated functions; no receiver object.

---

## 3. Method Implementations and `self`

### Instance Methods
Methods that take `self` as the first argument.

```rust
impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x.pow(2) + self.y.pow(2)) as f64
    }
}
```

| `self` Form       | Description                         | Ownership         |
|------------------|-------------------------------------|-------------------|
| `&self`          | Immutable borrow                    | Shared reference  |
| `&mut self`      | Mutable borrow                      | Exclusive mutable |
| `self`           | Takes ownership                     | Owned             |

#### Example:

```rust
impl Point {
    fn into_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }
}
```

Calling `point.into_tuple()` consumes `point`.

---

## 4. Associated Constants and Types

### Associated Constants

```rust
impl Point {
    const ORIGIN_X: i32 = 0;
    const ORIGIN_Y: i32 = 0;
}
```

Usage: `Point::ORIGIN_X`

### Associated Types

Used in trait implementations.

```rust
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}
```

---

## 5. Generic Implementations

You can implement generic functions over types.

```rust
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```

### Where Clause for Constraints

```rust
impl<T> Wrapper<T>
where
    T: std::fmt::Debug,
{
    fn print(&self) {
        println!("{:?}", self.value);
    }
}
```

---

## 6. Trait Implementations (`impl Trait`)

### Basic Trait Implementation

```rust
trait Printable {
    fn print(&self);
}

impl Printable for Point {
    fn print(&self) {
        println!("({}, {})", self.x, self.y);
    }
}
```

### Blanket Implementation

Implementing for all types that meet a condition.

```rust
impl<T: Display> Printable for T {
    fn print(&self) {
        println!("{}", self);
    }
}
```

---

## 7. Blanket Implementations

These are trait implementations applied to many types using bounds.

```rust
trait Loggable {
    fn log(&self);
}

impl<T: Debug> Loggable for T {
    fn log(&self) {
        println!("{:?}", self);
    }
}
```

Useful for providing default behaviors across types.

---

## 8. Private vs Public Implementations

By default, implementations are private.

To expose them publicly:

```rust
pub struct PublicPoint {
    pub x: i32,
    pub y: i32,
}

impl PublicPoint {
    pub fn public_method(&self) {
        // visible outside
    }

    fn private_method(&self) {
        // only visible within module
    }
}
```

---

## 9. Visibility Rules and Scoping

| Keyword   | Scope                             |
|----------|-----------------------------------|
| No modifier | Module-private                   |
| `pub`     | Visible everywhere                 |
| `pub(crate)` | Crate-visible only              |
| `pub(super)` | Parent module only              |
| `pub(in path)` | Custom path scope             |

### Example:

```rust
mod outer {
    pub mod inner {
        pub struct MyStruct;

        impl MyStruct {
            pub(in crate::outer) fn accessible_in_outer(&self) {}
        }
    }
}
```

---

## 10. Attributes and Modifiers

### Common Attributes for `impl` Blocks

| Attribute             | Purpose                                                  |
|-----------------------|-----------------------------------------------------------|
| `#[derive(...)]`      | Auto-generates trait impls (not on `impl`, but related)  |
| `#[inline]`           | Suggest inlining                                          |
| `#[must_use]`         | Warn if return value is unused                            |
| `#[deprecated]`       | Mark method/impl as deprecated                            |
| `#[cfg(...)]`         | Conditional compilation                                   |
| `#[doc(hidden)]`      | Hide from documentation                                   |

### Examples:

```rust
#[deprecated(note = "use new_api instead")]
fn old_api() {}

#[cfg(feature = "experimental")]
impl ExperimentalFeature {
    fn enable() {}
}
```

---

## 11. Internal Representation and Memory Layout

### Memory Layout

Rust guarantees layout compatibility under certain conditions (e.g., `#[repr(C)]`):

```rust
#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}
```

This allows safe FFI usage.

### Vtables for Trait Objects

When using trait objects (`dyn Trait`), Rust creates a vtable containing function pointers and metadata.

```rust
let p: Box<dyn Printable> = Box::new(Point::new(1, 2));
```

Internally represented as:

```c
struct {
    data_ptr: *const (),
    vtable: *const VTable,
}
```

Where `VTable` contains function pointers like `drop`, `print`, etc.

---

## 12. Lesser-Known Features and Edge Cases

### 1. Multiple `impl` Blocks

You can split implementation across multiple blocks.

```rust
impl Point {
    fn a(&self) {}
}

impl Point {
    fn b(&self) {}
}
```

### 2. Negative Trait Implementations (Unstable)

```rust
#![feature(negative_impls)]

impl !Send for MyType {}
```

Prevents types from being marked as `Send`.

### 3. Orphan Rule

You can only implement a foreign trait on a local type.

```rust
// ❌ Invalid: implementing `Display` on `Vec<i32>` (both foreign)
impl Display for Vec<i32> { /* ... */ }
```

Workaround: Use a wrapper type.

```rust
struct MyVec(Vec<i32>);
impl Display for MyVec { /* ... */ }
```

### 4. Inherent vs Trait Methods

If a name conflicts between inherent and trait method, you must disambiguate:

```rust
trait Foo {
    fn bar();
}

impl Point {
    fn bar() {}  // inherent
}

impl Foo for Point {
    fn bar() {}  // trait
}

// Call with:
Point::bar(); // calls inherent
<Point as Foo>::bar(); // calls trait
```

---

## 13. Gotchas, Tips, and Tricks

| Tip | Explanation |
|-----|-------------|
| Prefer `&self` unless mutation or ownership is needed | Avoid unnecessary borrowing |
| Use `#[must_use]` on important return values | Prevent accidental ignores |
| Split logic between inherent and trait impls | For better abstraction |
| Use `impl Trait` in return position | For simpler APIs |
| Use `Default` trait for zero-arg constructors | Instead of `new()` |
| Always use `pub` for public APIs | Otherwise hidden by default |

### Example: Using `impl Trait` in Return

```rust
fn get_iterator() -> impl Iterator<Item = i32> {
    (0..).take(5)
}
```

This hides the concrete type.

---

## 14. Comparison with Other Languages

| Feature | Rust | C++ | Java | Python |
|--------|------|-----|------|--------|
| Static methods | `fn new()` in `impl` | `static void foo()` | `public static void foo()` | `@staticmethod def foo():` |
| Instance methods | `fn bar(&self)` | `void bar()` | `void bar()` | `def bar(self):` |
| Traits/interfaces | `trait` + `impl Trait for Type` | `class Interface { virtual void foo() = 0; }` | `interface` | `Abstract Base Classes` |
| Generics | `impl<T>` | Templates | Generics | Dynamic typing |
| Visibility control | `pub`, `pub(crate)`, etc. | Access modifiers | `public`, `private` | Module-level |
| VTables | Implicit for trait objects | Explicit via virtual functions | Built-in interface dispatch | Dynamic attribute lookup |
| Blanket impls | Yes (`impl<T: Trait>`) | Via templates | Not directly supported | Monkey-patching possible |
| Associated types | Yes (`type Item`) | Typedefs / nested types | Generics | N/A |
| Default implementations | Yes (in traits) | Yes (C++20 Concepts?) | Yes (default methods) | Mixins / ABCs |
| Inlineable | `#[inline]` | `inline` keyword | JVM decides | N/A |
| Deprecation | `#[deprecated]` | `[[deprecated]]` | `@Deprecated` | `warnings.warn()` |

---

## Summary Table: Trade-offs Between Implementation Styles

| Style | Use Case | Pros | Cons |
|-------|----------|------|------|
| Inherent `impl` | Logic tightly bound to type | Fast, simple | Not reusable |
| Trait `impl` | Abstract behavior sharing | Extensible, composable | More indirection |
| Blanket `impl` | Generalize behavior across types | Reusable, DRY | Hard to debug |
| Generic `impl` | Flexible logic across types | Reusable | Complex lifetimes |
| `pub` vs non-`pub` | API design | Control visibility | Can break encapsulation if misused |

---

