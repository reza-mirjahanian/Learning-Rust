# Rust Shadowing Complete Reference

## Basic Shadowing
```rust
// Variable shadowing
let x = 5;
let x = x + 1;    // Shadows previous x
let x = x * 2;    // Shadows previous x again
println!("{}", x); // Output: 12

// Function parameter shadowing
fn example(x: i32) {
    let x = x * 2;  // Shadows parameter x
    println!("{}", x);
}
```

## Type Changing Through Shadowing
```rust
// Valid: Change type through shadowing
let data = "   ";        // type: &str
let data = data.len();   // type: usize

// Multiple type transformations
let value = "123";       // type: &str
let value = value.parse::<i32>().unwrap(); // type: i32
let value = value.to_string(); // type: String
```

## Scope-Based Shadowing
```rust
let x = 5;
{
    let x = 10;          // Inner scope shadows
    println!("Inner x: {}", x); // Prints 10
}
println!("Outer x: {}", x);     // Prints 5

// Multiple nested scopes
let x = 1;
{
    let x = 2;
    {
        let x = 3;
        println!("Innermost x: {}", x); // Prints 3
    }
    println!("Middle x: {}", x);        // Prints 2
}
println!("Outer x: {}", x);             // Prints 1
```

## Pattern Matching and Shadowing
```rust
// Match expression shadowing
let x = Some(5);
match x {
    Some(x) => println!("Found: {}", x), // Shadows outer x
    None => println!("Nothing"),
}

// If-let shadowing
let maybe_value = Some(42);
if let Some(maybe_value) = maybe_value {
    println!("{}", maybe_value); // Shadows outer maybe_value
}
```

## Loop Shadowing
```rust
let mut x = 0;
for x in 0..5 {          // Shadows outer x in loop
    println!("Loop x: {}", x);
}
println!("Original x: {}", x); // Original x unchanged

// While let shadowing
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
while let Some(top) = stack.pop() {
    let top = top * 2;   // Shadows matched top
    println!("{}", top);
}
```

## Function and Closure Shadowing
```rust
fn shadow_demo() {
    let x = 1;
    {
        let x = |y| x + y; // Closure shadows x with function
        println!("Closure result: {}", x(2));
    }
    println!("Original x: {}", x);
}

// Method implementation shadowing
struct Example {
    value: i32,
}

impl Example {
    fn modify(&self) {
        let value = self.value * 2; // Shadows field name
        println!("Modified: {}", value);
    }
}
```

## Mutable vs Immutable Shadowing
```rust
let mut x = 5;
let x = x + 1;           // Creates new immutable binding

// Shadowing with different mutability
let value = 42;
let mut value = value;   // Creates new mutable binding
value += 1;
```

## Shadowing in Match Arms
```rust
match some_value {
    Some(n) => {
        let n = n * 2;   // Shadows matched n
        println!("{}", n);
    }
    None => println!("No value"),
}
```

## Common Use Cases and Patterns

### Temporary Transformations
```rust
let data = "123";
let data = data.parse::<i32>().unwrap();
// Original string no longer needed
```

### Input Processing
```rust
let input = "   hello   ";
let input = input.trim();
let input = input.to_uppercase();
```

### Complex Initialization
```rust
let config = get_default_config();
let config = apply_user_settings(config);
let config = validate_config(config)?;
```

## Shadowing Limitations

### Cannot Shadow Across Match Arms
```rust
match value {
    Some(x) => {
        let x = x * 2;
    }
    None => {
        let x = 0; // Different x, not shadowing
    }
}
```

### No Shadowing in Different Match Patterns
```rust
match value {
    x @ 1..=5 => println!("Small: {}", x),
    x @ 6..=10 => println!("Medium: {}", x), // New binding
}
```

## Best Practices

### Clear Intent
```rust
// Good: Clear transformation
let bytes = "hello";
let bytes = bytes.as_bytes();

// Avoid: Confusing reuse
let data = 5;
let data = String::from("5"); // Unclear why type changed
```

### Scope Management
```rust
// Good: Limited scope for transformed value
{
    let temp = original.to_uppercase();
    // Work with uppercase version
}
// Continue with original
```

### Documentation
```rust
// Document significant shadowing
let value = get_raw_value();
// Transform for processing
let value = value.normalize();
// Prepare for output
let value = value.format();
```