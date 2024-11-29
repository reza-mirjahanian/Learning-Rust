# Rust Scope Reference Guide

## Basic Scope Rules
```rust
{                          // Outer scope begins
    let x = 5;
    {                      // Inner scope begins
        let y = 10;
        println!("x: {}, y: {}", x, y); // Both x and y accessible
    }                      // Inner scope ends, y is dropped
    println!("x: {}", x);  // Only x accessible
}                          // Outer scope ends, x is dropped
```

## Variable Shadowing
```rust
let x = 5;
{
    let x = 10;           // Shadows outer x
    println!("Inner x: {}", x); // Prints 10
}
println!("Outer x: {}", x);     // Prints 5

// Type changing through shadowing
let spaces = "   ";
let spaces = spaces.len(); // Valid: changes type from &str to usize
```

## Ownership and Scope
```rust
let s1 = String::from("hello");
{
    let s2 = s1;          // Ownership moved to s2
    // println!("{}", s1); // Error: s1 no longer valid
}                         // s2 dropped here

// Multiple references
let s1 = String::from("hello");
{
    let r1 = &s1;         // Immutable borrow
    let r2 = &s1;         // Multiple immutable borrows OK
    println!("{} {}", r1, r2);
}                         // r1 and r2 dropped here
```

## Lifetime Scopes
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,        // Lifetime annotation required
}
```

## Loop Scopes
```rust
let mut counter = 0;
loop {
    let temp = counter;    // New scope for each iteration
    if temp > 5 { break; }
    counter += 1;
}                         // temp dropped after each iteration

// Loop labels for nested loops
'outer: loop {
    'inner: loop {
        break 'outer;     // Breaks outer loop
    }
}
```

## Function Scopes
```rust
fn outer_function() {
    let x = 5;
    
    fn inner_function() {
        // Cannot access x here
        let y = 10;
    }
    
    // Cannot access y here
}

// Closure scope
let x = 4;
let closure = |y| x + y;  // Closure can capture x
```

## Module Scopes
```rust
mod outer_module {
    pub fn public_function() {}
    fn private_function() {}
    
    mod inner_module {
        // Can access parent module items
        super::public_function();
    }
}
```

## Match Expression Scopes
```rust
match value {
    Some(x) => {          // x only valid in this scope
        println!("{}", x);
    }
    None => {
        // x not accessible here
    }
}
```

## Trait Implementation Scopes
```rust
trait MyTrait {
    fn method(&self);
}

impl MyTrait for String {
    fn method(&self) {    // Scope limited to this implementation
        let local_var = 5;
    }
}
```

## Generic Scopes
```rust
struct Container<T> {
    item: T,              // T scope is throughout struct
}

impl<T> Container<T> {    // T scope is throughout impl block
    fn get(&self) -> &T {
        &self.item
    }
}
```

## Scope and Drop Order
```rust
struct CustomDrop;
impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

{
    let _a = CustomDrop;
    {
        let _b = CustomDrop;
        // _b dropped first
    }
    // _a dropped second
}
```

## Static and Const Scopes
```rust
static GLOBAL: i32 = 42;  // Global scope

const CONSTANT: i32 = 15; // Available in module scope

fn function() {
    static LOCAL_STATIC: i32 = 10; // Function-local static
}
```

## Use Declaration Scopes
```rust
use std::collections::HashMap;
{
    use std::collections::HashSet; // Limited to this scope
}
// HashSet not available here
```

## Error Handling Scopes
```rust
fn example() -> Result<(), Error> {
    let file = File::open("file.txt")?; // Scope tied to Result
    Ok(())
}                                       // file dropped here

match result {
    Ok(value) => {        // value scope limited to this block
        println!("{}", value);
    }
    Err(e) => {          // e scope limited to this block
        println!("{}", e);
    }
}
```

## Thread Scopes
```rust
use std::thread;

let v = vec![1, 2, 3];
thread::scope(|s| {
    s.spawn(|| {
        println!("{:?}", v); // Can borrow v
    });
});                        // Ensures all threads complete
```

## Unsafe Scopes
```rust
unsafe {
    // Unsafe operations limited to this scope
    let ptr = 0x12345usize as *const i32;
}
// Return to safe Rust
```