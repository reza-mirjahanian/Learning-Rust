# Rust Functions Guide

## Basic Function Syntax
```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // function body
}
```

## 1. Function Declaration Patterns

### Regular Functions
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // Implicit return without semicolon
}
```

### Functions with Multiple Returns
```rust
fn divide_and_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}
```

### Generic Functions
```rust
fn print_type<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}
```

## 2. Return Types & Patterns

### Early Returns
```rust
fn process_number(x: i32) -> Option<i32> {
    if x < 0 {
        return None;
    }
    Some(x * 2)
}
```

### Result Returns
```rust
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok(x / y)
}
```

### Never Type (!)
```rust
fn exit_program() -> ! {
    std::process::exit(1);
}
```

## 3. Function Parameters

### Default Parameters (Using Option)
```rust
fn greet(name: Option<&str>) {
    println!("Hello, {}!", name.unwrap_or("World"));
}
```

### Variable Number of Parameters
```rust
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}
```

### Mutable Parameters
```rust
fn modify(value: &mut i32) {
    *value += 1;
}
```

## 4. Advanced Function Features

### Closures
```rust
let add = |x: i32, y: i32| x + y;
let multiply = |x, y| {
    x * y
}
```

### Function Pointers
```rust
fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
```

### Associated Functions (Static Methods)
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}
```

## 5. Function Attributes

### Test Functions
```rust
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}
```

### Inline Functions
```rust
#[inline]
fn quick_add(x: i32, y: i32) -> i32 {
    x + y
}
```

### Deprecated Functions
```rust
#[deprecated(since = "1.0.0", note = "please use `new_function` instead")]
fn old_function() {
    // ...
}
```

## 6. Higher-Order Functions

### Map, Filter, Reduce
```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
```

### Function Composition
```rust
fn compose<F, G, T, U, V>(f: F, g: G) -> impl Fn(T) -> V
where
    F: Fn(U) -> V,
    G: Fn(T) -> U,
{
    move |x| f(g(x))
}
```

## 7. Async Functions
```rust
async fn fetch_data() -> Result<String, Error> {
    // Async operations
}
```

## 8. Unsafe Functions
```rust
unsafe fn dangerous() {
    // Unsafe operations
}
```

## 9. Function Traits

### Fn Traits
```rust
fn execute<F>(f: F) where F: Fn() {
    f();
}

fn execute_mut<F>(mut f: F) where F: FnMut() {
    f();
}

fn execute_once<F>(f: F) where F: FnOnce() {
    f();
}
```

## 10. Function Overloading (Through Traits)
```rust
trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("Integer: {}", self);
    }
}

impl Printable for String {
    fn print(&self) {
        println!("String: {}", self);
    }
}
```

## 11. Generic Constraints
```rust
fn process<T: Display + Clone>(item: T) {
    println!("{}", item);
}

fn complex_constraint<T: Display + Clone, U: PartialEq + Default>(t: T, u: U) {
    // Implementation
}
```

## 12. Function Privacy
```rust
pub fn public_function() {
    private_function();
}

fn private_function() {
    // Only accessible within the same module
}
```

## 13. Function Documentation
```rust
/// Adds two numbers together
/// 
/// # Arguments
/// * `x` - First number
/// * `y` - Second number
/// 
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```