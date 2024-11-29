# Const vs Immutable Variables in Rust

## Key Differences Table

| Feature | `const` | Immutable `let` |
|---------|---------|-----------------|
| Mutability | Never mutable | Immutable by default |
| Scope | Global | Block-scoped |
| Evaluation | Compile-time | Runtime |
| Type annotation | Required | Optional |
| Memory location | Inlined | Stack/Heap |
| Shadowing | Not allowed | Allowed |
| Expression complexity | Limited | Any expression |

## Const Usage
```rust
// Constants must be type annotated
const MAX_POINTS: u32 = 100_000;

// Can be used in global scope
const PI: f64 = 3.14159;

// Only compile-time constant expressions allowed
const SQUARES: [i32; 4] = [1, 4, 9, 16];

// Can use in const contexts
const ARRAY_SIZE: usize = 4;
let array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
```

## Immutable Let Usage
```rust
// Type annotation optional
let x = 5;
let y: i32 = 10;

// Can be initialized with runtime values
let random = rand::random::<i32>();

// Can be shadowed
let value = 5;
let value = value + 1;  // New variable, shadows previous one

// Complex expressions allowed
let result = {
    let x = 3;
    x * x
};
```

## Compile-Time vs Runtime
```rust
// Const: Must be compile-time evaluable
const COMPUTED: u32 = 15 * 3; // OK
const RANDOM: u32 = rand::random(); // Error: not const

// Let: Can use runtime values
let runtime_value = std::env::var("SOME_VAR").unwrap();
```

## Memory and Performance
```rust
// Const: Inlined at compile time
const MAX: u32 = 100;
// Equivalent to directly writing:
let x = 100;

// Let: Allocated at runtime
let max = 100;
// Stored in memory location
```

## Scope and Visibility
```rust
// Const: Can be global
const GLOBAL: i32 = 42;

mod my_module {
    // Visible based on privacy rules
    pub const MODULE_CONST: i32 = 10;
}

fn example() {
    // Let: Only block-scoped
    let local = 5;
    {
        let inner = 10;
    } // inner goes out of scope
} // local goes out of scope
```

## Use Cases

### When to Use Const
```rust
// Configuration values
const CONFIG_PATH: &str = "/etc/app/config";

// Mathematical constants
const PI: f64 = 3.14159265359;

// Array sizes
const BUFFER_SIZE: usize = 1024;

// Bit masks
const MASK: u32 = 0xFF00;
```

### When to Use Immutable Let
```rust
// Runtime computed values
let result = complex_calculation();

// User input
let user_input = get_user_input();

// File contents
let file_content = std::fs::read_to_string("file.txt")?;

// Results of function calls
let processed_data = process_data(&input);
```

## Limitations and Rules

### Const Limitations
```rust
// Must be type annotated
const NO_TYPE = 5; // Error

// Cannot use non-const functions
const RANDOM: i32 = rand::random(); // Error

// Cannot be mutable
const mut MUTABLE: i32 = 5; // Error

// Cannot be shadowed
const VALUE: i32 = 5;
const VALUE: i32 = 6; // Error
```

### Let Features
```rust
// Can be shadowed with different types
let value = 5;
let value = "hello"; // OK

// Can use complex runtime expressions
let value = if condition {
    compute_something()
} else {
    compute_something_else()
};

// Can be part of pattern matching
let (x, y) = (1, 2);
```

## Memory Layout
```rust
// Const: No runtime memory allocation
const BYTES: [u8; 3] = [1, 2, 3];
// Inlined wherever used

// Let: Allocated in memory
let bytes = [1, 2, 3];
// Stored on stack or heap depending on type
```