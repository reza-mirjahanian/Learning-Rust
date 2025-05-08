## Constant Items in Rust: A Technical Reference Guide

---

### 1. Introduction to Constant Items (`const`)

#### 1.1. Definition and Purpose

A **constant item** (or simply `const`) in Rust is an identifier bound to a value that is known at compile time. Constants are not stored in a specific memory location that can be referenced at runtime; instead, their values are typically inlined directly into the places where they are used.

**Primary Purposes:**

1.  **Named Literal Values:** Provide meaningful names for hardcoded values, improving code readability and maintainability.
2.  **Compile-Time Computation:** Define values that can be computed entirely at compile time, ensuring they are fixed and validated before runtime.
3.  **API Definitions:** Define fixed values as part of a library's public API (e.g., default settings, limits, bitflags).
4.  **Generic Programming:** Used with `const` generics to parameterize types by constant values.

Constants are inherently immutable. Once declared, their value can never change.

#### 1.2. Basic Syntax

Constants are declared using the `const` keyword, followed by an identifier, a type annotation, an equals sign, and a value expression.

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.1415926535;
const APP_NAME: &str = "My Awesome App";
```

*   `MAX_POINTS`, `PI`, `APP_NAME`: The name of the constant. By convention, `SCREAMING_SNAKE_CASE` is used.
*   `: u32`, `: f64`, `: &str`: The explicit type annotation. This is **mandatory** for constants.
*   `= 100_000`, `= 3.1415926535`, `= "My Awesome App"`: The value expression, which must be a *constant expression*.

#### 1.3. Core Properties

*   **Immutability:** Constants are deeply immutable. You cannot use `mut` with a `const`.
    ```rust
    // const MUTABLE_CONST: mut i32 = 5; // Compile-time error: constants cannot be mutable
    ```
*   **Compile-Time Evaluation:** The value of a `const` must be evaluatable at compile time. This means it cannot depend on runtime state, I/O, or other non-deterministic factors.
*   **Type Annotation Required:** The type of a `const` must always be explicitly annotated. Rust does not infer types for `const` items.
*   **No Fixed Memory Address:** Constants typically do not have a fixed memory address. The compiler usually inlines their values where they are used. Attempting to take a reference to a `const` whose type is not a reference type itself will result in the value being embedded. If the `const`'s type is a reference (e.g., `&'static str`), the reference points to statically allocated data.

---

### 2. Defining and Using `const` Items

#### 2.1. Type Annotation Requirement

As mentioned, type annotations are mandatory for `const` items.

```rust
const DEFAULT_TIMEOUT_MS: u64 = 5000; // Correct
// const DEFAULT_TIMEOUT_MS = 5000; // Compile-time error: missing type for `const` item
```

#### 2.2. Allowed Expressions in `const` Initializers

The initializer for a `const` item must be a *constant expression*. This means it must be evaluatable by the compiler at compile time. The set of operations allowed in constant expressions has expanded over Rust versions but generally includes:

*   Literals (integers, floats, booleans, characters, strings).
*   Paths to other `const` items.
*   Tuple, array, and struct expressions if their fields are also constant expressions.
*   Enum variants.
*   Built-in arithmetic and logical operations.
*   Casts (if valid at compile time).
*   Address-of operator (`&`) if it results in a reference to a `static` or to the interior of a constant expression.
*   Function calls to `const fn` (see Section 4.1).
*   `unsafe` blocks (if the operations within are `const`-compatible, e.g., dereferencing a raw pointer derived from a `static` or `const`).
*   Control flow like `if`, `match`, `loop`, `while` (within `const fn` or `const` blocks).
*   Variable bindings (within `const fn` or `const` blocks).

**Not Allowed (Generally):**

*   Calls to non-`const` functions.
*   Heap allocations (unless through `const fn` that specifically support it, like `String::new()` or `Vec::new()` which are `const`).
*   Operations that depend on runtime environment (e.g., random number generation, system time, FFI calls to non-`const` functions).
*   Dereferencing raw pointers that don't point to `static` data.
*   Types with interior mutability like `Cell` or `RefCell` cannot generally be directly *mutated* in a `const` initializer, though values of these types can be created if their constructors are `const`.

```rust
const VALUE_A: i32 = 10;
const VALUE_B: i32 = VALUE_A * 2 + 5; // OK: uses another const and arithmetic

const ARRAY_CONST: [i32; 3] = [VALUE_A, VALUE_B, 30]; // OK: array of const expressions

struct Point { x: i32, y: i32 }
const ORIGIN: Point = Point { x: 0, y: 0 }; // OK: struct constructor

const GREETING: &str = "Hello"; // OK: string literal (&'static str)
const REFERENCE_TO_STATIC: &'static i32 = &VALUE_A; // This creates a new anonymous static containing VALUE_A, and REFERENCE_TO_STATIC points to it.

// This example requires a const fn for more complex logic
const fn compute_value(input: u32) -> u32 {
    input * input + 1
}
const COMPUTED: u32 = compute_value(5); // OK: calls a const fn
```

#### 2.3. Scope of Definition

Constants can be defined in several scopes:

**2.3.1. Module/Crate Level (Global Constants)**

Defined outside any function or `impl` block, they are accessible according to Rust's visibility rules (see Section 5.1).

```rust
// In lib.rs or main.rs or any module file
mod config {
    pub const MAX_USERS: usize = 1000;
    const INTERNAL_FLAG: bool = true;
}

fn main() {
    println!("Max users: {}", config::MAX_USERS);
    // println!("{}", config::INTERNAL_FLAG); // Error: INTERNAL_FLAG is private
}
```

**2.3.2. `impl` Blocks (Associated Constants)**

Constants can be associated with a type (struct, enum, or trait).

```rust
struct Circle {
    radius: f64,
}

impl Circle {
    const DEFAULT_RADIUS: f64 = 1.0;

    fn new_default() -> Self {
        Circle { radius: Self::DEFAULT_RADIUS }
    }
}

trait Shape {
    const NAME: &'static str;
    fn name(&self) -> &'static str {
        Self::NAME
    }
}

impl Shape for Circle {
    const NAME: &'static str = "Circle";
}

fn main() {
    println!("Default circle radius: {}", Circle::DEFAULT_RADIUS);
    let c = Circle::new_default();
    println!("Shape name: {}", c.name()); // Uses associated const from trait impl
    println!("Shape name via type: {}", Circle::NAME);
}
```
Associated constants in traits provide a way to define constants that implementations must (or can) provide.

**2.3.3. Function/Block Level (Local Constants)**

Constants can be defined within functions or other block scopes. Their scope is limited to that block.

```rust
fn process_data(data: &[u8]) {
    const MAX_CHUNK_SIZE: usize = 1024;
    if data.len() > MAX_CHUNK_SIZE {
        println!("Data exceeds max chunk size of {}", MAX_CHUNK_SIZE);
    }
    // MAX_CHUNK_SIZE is not accessible outside process_data
}

fn main() {
    process_data(&[0; 2000]);
    // println!("{}", MAX_CHUNK_SIZE); // Error: cannot find value `MAX_CHUNK_SIZE` in this scope
}
```
Local constants behave like global constants (inlined, no fixed address) but are only visible locally.

---

### 3. Behavior and Semantics

#### 3.1. Inlining and Value Substitution

The most significant characteristic of `const` items is that they are typically **inlined** wherever they are used. The compiler substitutes the value of the constant directly into the code.

```rust
const COUNT: i32 = 10;

fn main() {
    let x = COUNT * 5; // Compiler likely replaces COUNT with 10, so x = 10 * 5;
    println!("{}", x); // Prints 50
    println!("{}", COUNT); // Prints 10 (inlined here too)
}
```
This means that changing the value of a `const` in a library requires recompilation of all downstream crates that use it, as the old value would have been inlined.

#### 3.2. No Fixed Memory Address (Rvalue Semantics)

Unlike `static` items, `const` items do not necessarily have a fixed memory address. They are primarily rvalues (values that can appear on the right-hand side of an assignment).

```rust
const MY_CONST: i32 = 42;
// static MY_STATIC: i32 = 42;

fn main() {
    // let const_ref = &MY_CONST; // This is allowed.
    // The Rust reference says: "Constants may not have a fixed address in memory,
    // though the compiler may place them in read-only memory if their type allows for it.
    // If the type of the constant has a fixed address (e.g. `&'static i32`), then the constant
    // refers to that address. Otherwise, the constant's value is inlined wherever it is used.
    // Taking a reference to a constant value causes that value to be stored in a new,
    // anonymous static item, and the reference will point to that."

    let val1 = MY_CONST;
    let val2 = MY_CONST;
    // &val1 and &val2 would be different addresses if val1/val2 are stack variables.

    // Example demonstrating the "anonymous static" behavior for references to consts
    const ANSWER: i32 = 42;
    let r1: &'static i32 = &ANSWER;
    let r2: &'static i32 = &ANSWER;

    // r1 and r2 might point to the same or different anonymous statics.
    // The compiler is free to duplicate the data or deduplicate it.
    // Let's test this behavior.
    // On my machine (Rust 1.78), they point to the same address.
    // This implies the compiler "promotes" `ANSWER` to a static location for the reference.
    println!("Address of r1: {:p}", r1);
    println!("Address of r2: {:p}", r2);
    // if std::ptr::eq(r1, r2) {
    //     println!("r1 and r2 point to the same memory location.");
    // } else {
    //     println!("r1 and r2 point to different memory locations.");
    // }

    const GREETING: &str = "Hello"; // GREETING itself is a &'static str
    let p1: *const u8 = GREETING.as_ptr();
    let p2: *const u8 = GREETING.as_ptr();
    println!("Address of GREETING (p1): {:p}", p1);
    println!("Address of GREETING (p2): {:p}", p2); // p1 and p2 will be the same here because GREETING's type is already a reference.
}
```
The key is that `MY_CONST` itself is not a memory location; it's a value. When `&MY_CONST` is used, the *value* $42$ is materialized into a read-only memory location (an anonymous `static`), and the reference points to that.

#### 3.3. `const` vs. `static`

Constants (`const`) and static items (`static`) are often confused but have crucial differences.

| Feature             | `const`                                                                 | `static`                                                                |
| ------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| **Mutability**      | Always immutable. Cannot use `mut`.                                     | Can be mutable (`static mut`). Accessing/modifying `static mut` is `unsafe`. |
| **Memory Address**  | Typically no fixed memory address; value is inlined.                      | Has a fixed memory address. Lives for the `'static` lifetime.          |
| **Value**           | Represents a value, an rvalue.                                          | Represents a memory location, an lvalue.                                |
| **Type Annotation** | Mandatory.                                                              | Mandatory.                                                              |
| **Initializer**     | Must be a constant expression, evaluated at compile time.               | Must be a constant expression, evaluated at compile time.               |
| **Lifetime**        | Value is 'baked in' at compile time.                                      | `'static` lifetime. Exists for the entire program duration.            |
| **Usage**           | For named literal values, compile-time calculations.                    | For global variables, fixed data shared across threads (if `Sync`).     |
| **Inlining**        | Values are usually inlined.                                               | Accessed via its memory address (not inlined).                        |
| **`Drop`**          | Types used in `const` cannot have `Drop` implemented (unless wrapped in `ManuallyDrop` and not dropped). | Types used in `static` cannot have `Drop` implemented.                   |
| **References**      | `&CONST_ITEM` creates a reference to a value typically promoted to an anonymous static. | `&STATIC_ITEM` gives a reference to the static's memory location.       |

**Example Illustrating Differences:**

```rust
const MY_CONST_VALUE: i32 = 10;
static MY_STATIC_VALUE: i32 = 10;
static mut MY_MUT_STATIC: i32 = 20;

fn print_addresses() {
    // Taking reference to a const's value:
    let const_ref1: &'static i32 = &MY_CONST_VALUE;
    let const_ref2: &'static i32 = &MY_CONST_VALUE;

    // Taking reference to a static:
    let static_ref1: &'static i32 = &MY_STATIC_VALUE;
    let static_ref2: &'static i32 = &MY_STATIC_VALUE;

    println!("Address of const_ref1: {:p}", const_ref1);
    println!("Address of const_ref2: {:p}", const_ref2); // Might be same or different from const_ref1 based on compiler optimization. Usually same.

    println!("Address of static_ref1: {:p}", static_ref1);
    println!("Address of static_ref2: {:p}", static_ref2); // Guaranteed to be same as static_ref1.

    unsafe {
        let mut_static_ref: &'static i32 = &MY_MUT_STATIC;
        println!("Address of mut_static_ref: {:p}", mut_static_ref);
        MY_MUT_STATIC = 30; // Unsafe operation
        println!("MY_MUT_STATIC new value: {}", MY_MUT_STATIC);
    }
}

fn main() {
    print_addresses();
    let a = MY_CONST_VALUE; // MY_CONST_VALUE (10) likely inlined
    let b = MY_STATIC_VALUE; // MY_STATIC_VALUE accessed from its memory location
    println!("const: {}, static: {}", a, b);
}
```

**When to use which:**

*   Use `const` for values that are fixed at compile time and can be substituted directly into code, like mathematical constants, configuration flags, or default values.
*   Use `static` for data that needs a persistent memory location throughout the program's execution, such as global state (use with caution, especially `static mut`). `static` is also used for values that are too large to be reasonably inlined or whose identity (address) matters.

---

### 4. Advanced `const` Features

#### 4.1. `const fn` (Constant Functions)

`const fn` are functions that can be evaluated at compile time. They are crucial for performing more complex computations within `const` initializers.

**4.1.1. Defining `const fn`**

```rust
const fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

const FIVE_FACTORIAL: u32 = factorial(5); // Evaluated at compile time

fn main() {
    println!("5! = {}", FIVE_FACTORIAL); // Prints 120
    println!("10! = {}", factorial(10)); // Can also be called at runtime
}
```
A `const fn` can also be called in a non-const context (at runtime).

**4.1.2. What's allowed in `const fn`**

The set of features usable within `const fn` has been steadily expanding. As of Rust 1.78, it includes:

*   Most expressions allowed in regular functions:
    *   Variable bindings (`let`).
    *   Arithmetic, logical, comparison operations.
    *   Tuple, array, struct, enum variant creation.
    *   Field access, indexing.
    *   Method calls (if the methods are also `const fn`).
    *   Control flow: `if`, `else`, `match`, `loop`, `while`, `for` (over ranges or `const`-iterable types).
    *   `return`, `break`, `continue`.
    *   Blocks `{ ... }`.
*   Calls to other `const fn`.
*   `unsafe` blocks and `unsafe fn` calls (if the operations are const-compatible).
    *   This allows raw pointer manipulation, union field access, etc., under `unsafe` contracts.
*   References and dereferences.
*   Some limited forms of trait bounds on generic parameters if those traits are `const`-compatible.
*   Mutable local variables (`let mut`).

**Limitations (Gradually being lifted):**

*   Cannot call non-`const fn` (including most FFI).
*   Cannot perform I/O, interact with the OS, or access non-deterministic resources.
*   Dynamic dispatch (`dyn Trait`) is generally not usable in `const` contexts (though this area is evolving).
*   Heap allocations: Standard library collections like `String::new()` and `Vec::new()` are now `const fn`, allowing their creation in `const` context. However, arbitrary dynamic allocation via `Box::new` with non-`const` values is not generally possible.
*   Floating point operations in `const fn` are subject to strict determinism rules.
*   Panic in `const fn` during compile-time evaluation results in a compile error.

**4.1.3. Calling `const fn` in `const` context**

This is the primary use case for defining `const fn`.

```rust
const fn is_even(n: u32) -> bool {
    n % 2 == 0
}

const IS_FOUR_EVEN: bool = is_even(4);
const MESSAGE: &str = if IS_FOUR_EVEN { "4 is even" } else { "4 is odd" };

fn main() {
    println!("{}", MESSAGE); // Prints "4 is even"
}
```

#### 4.2. `const` Blocks

A `const` block `const { ... }` allows a sequence of statements to be executed in a `const` context, yielding a final value. This is useful for more complex initialization logic that doesn't fit neatly into a single expression or a separate `const fn`.

```rust
const COMPLEX_SETUP: [i32; 4] = const {
    let mut temp_arr = [0; 4];
    temp_arr[0] = 10;
    temp_arr[1] = 20;
    // More complex logic, loops, etc., can go here
    let mut i = 2;
    while i < 4 {
        temp_arr[i] = temp_arr[i-1] + temp_arr[i-2];
        i += 1;
    }
    temp_arr // The value of the last expression is the value of the const block
};

fn main() {
    println!("{:?}", COMPLEX_SETUP); // Prints [10, 20, 30, 50]
}
```
`const` blocks provide a more imperative style for compile-time computations within `const` item initializers.

#### 4.3. `const` Generics

Constants can be used as generic parameters for types, functions, and impls. This allows types to be parameterized by values, not just other types or lifetimes. This is a powerful feature for type-level programming and creating highly optimized data structures.

```rust
// N is a const generic parameter
struct ArrayWrapper<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> ArrayWrapper<T, N> {
    const CAPACITY: usize = N; // Associated const using the generic const param

    fn new() -> Self {
        ArrayWrapper { data: [T::default(); N] }
    }
}

fn main() {
    let arr_3: ArrayWrapper<i32, 3> = ArrayWrapper::new();
    let arr_5: ArrayWrapper<u8, 5> = ArrayWrapper::new();

    println!("Capacity of arr_3: {}", ArrayWrapper::<i32, 3>::CAPACITY); // Prints 3
    println!("arr_3 data: {:?}", arr_3.data);
    println!("arr_5 data: {:?}", arr_5.data);
}
```
While `const` generics themselves are a broad topic, their interaction with `const` items (like defining associated constants based on generic const parameters) is relevant.

#### 4.4. `const` in Patterns

Constants can be used in `match` patterns and other pattern contexts (e.g., `if let`, `while let`).

```rust
const RED: u32 = 0xFF0000;
const GREEN: u32 = 0x00FF00;
const BLUE: u32 = 0x0000FF;

fn describe_color(color: u32) {
    match color {
        RED => println!("It's red!"),
        GREEN => println!("It's green!"),
        BLUE => println!("It's blue!"),
        _ => println!("It's some other color."),
    }
}

fn main() {
    describe_color(RED); // Prints "It's red!"
    describe_color(0x123456); // Prints "It's some other color."
}
```
For a `const` to be used in a pattern, its type must be an integer, `bool`, `char`, `&str`, or an enum variant without fields if the enum implements `PartialEq` and `Eq` (or is a "structural-match" type). More precisely, the type must be "structurally matchable". This means the compiler must be able to determine equality at compile time. References to statics are also allowed.

```rust
const ADMIN_USER_ID: &str = "admin";

fn check_user(user_id: &str) {
    match user_id {
        ADMIN_USER_ID => println!("Admin access granted."),
        _ => println!("Regular user."),
    }
}
```

---

### 5. Attributes and Modifiers for `const` Items

#### 5.1. Visibility Modifiers

Standard Rust visibility modifiers apply to `const` items, controlling where they can be accessed:

*   **`pub`**: Publicly visible.
    ```rust
    pub const PUBLIC_CONST: i32 = 1;
    ```
*   **`pub(crate)`**: Visible within the current crate.
    ```rust
    pub(crate) const CRATE_CONST: i32 = 2;
    ```
*   **`pub(super)`**: Visible within the parent module.
    ```rust
    mod M {
        pub(super) const SUPER_CONST: i32 = 3;
    }
    ```
*   **`pub(in path)`**: Visible within a specific path.
    ```rust
    mod A {
        pub mod B {
             pub(in crate::A) const A_B_CONST: i32 = 4;
        }
        // Can access A_B_CONST here
    }
    ```
*   **No modifier (private)**: Visible only within the current module and its children.
    ```rust
    const PRIVATE_CONST: i32 = 5;
    ```

#### 5.2. Documentation (`#[doc]`)

Constants can (and should) be documented using doc comments.

```rust
/// The maximum number of connections allowed.
///
/// This value is used to configure the server pool.
pub const MAX_CONNECTIONS: usize = 256;
```
These comments are processed by `rustdoc` to generate documentation.

#### 5.3. Lints (`#[allow]`, `#[deny]`, `#[forbid]`)

Lint attributes can be applied to `const` items to control compiler warnings.

```rust
#[allow(dead_code)] // Suppress warning if this const is unused
const UNUSED_CONFIG_FLAG: bool = false;
```

#### 5.4. `#[deprecated]`

Marks a `const` as deprecated, issuing a warning when it's used.

```rust
#[deprecated(since = "0.2.0", note = "Use NEW_TIMEOUT instead")]
pub const OLD_TIMEOUT: u32 = 1000;
pub const NEW_TIMEOUT: u32 = 1500;

fn main() {
    // Using OLD_TIMEOUT will produce a deprecation warning.
    println!("Old timeout: {}", OLD_TIMEOUT);
}
```

#### 5.5. (Implicit) `#[must_use]` via Type System

A `const` item itself cannot be directly annotated with `#[must_use]`. However, if the *type* of the `const` is marked `#[must_use]`, or if it's the result of a `const fn` marked `#[must_use]