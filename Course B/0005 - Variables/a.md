# Rust Variables Complete Reference

## Variable Declaration
```rust
// Basic declaration
let x = 5;                 // Immutable by default
let mut y = 10;            // Mutable variable
const MAX_POINTS: u32 = 100; // Constant (must specify type)
static GREETING: &str = "Hello"; // Static variable
```

## Variable Shadowing
```rust
let x = 5;
let x = x + 1;  // New variable shadows previous one
let x = x * 2;  // Can change type when shadowing
```

## Data Types

### Scalar Types
```rust
// Integers
let a: i8 = -128;          // 8-bit signed
let b: u8 = 255;           // 8-bit unsigned
let c: i16 = -32768;       // 16-bit signed
let d: u16 = 65535;        // 16-bit unsigned
let e: i32 = -2147483648;  // 32-bit signed
let f: u32 = 4294967295;   // 32-bit unsigned
let g: i64 = -9223372036854775808; // 64-bit signed
let h: u64 = 18446744073709551615; // 64-bit unsigned
let i: i128 = -170141183460469231731687303715884105728; // 128-bit signed
let j: isize = 0;          // Architecture-dependent signed
let k: usize = 0;          // Architecture-dependent unsigned

// Floating-point
let f1: f32 = 3.14;        // 32-bit float
let f2: f64 = 3.14159265359; // 64-bit float

// Boolean
let t: bool = true;
let f: bool = false;

// Character
let c: char = 'z';         // Unicode character (4 bytes)
```

### Compound Types
```rust
// Tuple
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;       // Destructuring
let first = tup.0;         // Access by index

// Array
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 5];        // [0, 0, 0, 0, 0]
let first = arr[0];        // Access by index
```

## Type Inference and Annotations
```rust
let guess = "42".parse().expect("Not a number!"); // Error: type annotation needed
let guess: u32 = "42".parse().expect("Not a number!"); // OK
```

## Variable Scope and Memory Management
```rust
{
    let x = 5;            // x comes into scope
    // x is valid here
}                         // x goes out of scope

let x = String::from("hello"); // x owns the string
let y = x;                     // ownership moved to y
// x can no longer be used
```

## References and Borrowing
```rust
let x = String::from("hello");
let y = &x;              // Immutable reference
let z = &mut x;          // Mutable reference (only one allowed)

// Cannot have mutable and immutable references simultaneously
let r1 = &x;
let r2 = &x;
let r3 = &mut x;         // Error!
```

## Variable Attributes
```rust
#[allow(dead_code)]
let unused_variable = 5;

#[deprecated]
const OLD_CONSTANT: i32 = 10;
```

## Pattern Binding
```rust
let (a, b, c) = (1, 2, 3);  // Tuple destructuring
let [head, tail @ ..] = [1, 2, 3, 4]; // Array pattern matching
```

```
fn main() {
    let array = [1, 2, 3, 4];
    let [head, tail @ ..] = array;

    println!("Head: {}", head); // Outputs: Head: 1
    println!("Tail: {:?}", tail); // Outputs: Tail: [2, 3, 4]
}

```
## Type Aliases
```rust
type Kilometers = i32;
let distance: Kilometers = 5;
```

## Memory Management Rules
1. Each value has a single owner
2. Only one mutable reference at a time
3. Cannot have mutable and immutable references simultaneously
4. References must always be valid

## Variable Initialization
```rust
let x;                    // Declared but not initialized
x = 5;                    // Initialize later (must happen before use)

let y: i32;              // Type annotation without initialization
assert_eq!(y, 5);        // Error: used before initialization
```

## Special Variables
```rust
let _unused = 5;         // Prefix with _ to suppress warnings
let _x = 5;             // Explicitly marked as unused

// Ignored pattern in match
match some_value {
    _ => println!("Default case"),
}
```

## Numeric Operations and Type Casting
```rust
let x = 5;
let y = 2;

let sum = x + y;
let difference = x - y;
let product = x * y;
let quotient = x / y;    // Integer division
let remainder = x % y;

// Type casting
let float = 3.14;
let integer = float as i32;
```

## String Variables
```rust
let s1 = String::from("hello"); // Owned string
let s2 = "world";              // String literal (&str)
let s3 = s1 + s2;             // String concatenation
let len = s3.len();           // String length
```

## Constants vs Statics
```rust
const MAX_POINTS: u32 = 100_000; // Inlined at compile time
static GREETING: &str = "Hello"; // Single memory location

const fn constant_fn() -> i32 {  // Constant function
    42
}
```