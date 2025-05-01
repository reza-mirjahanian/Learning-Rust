# Functions in Rust

## **1. Basic Syntax**
- Declare with `fn` keyword
- Parameters require type annotations
- Return type specified with `->`

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // Implicit return (no semicolon)
}

fn explicit_return(x: bool) -> &'static str {
    return if x { "Yes" } else { "No" };
}
```

---

## **2. Parameters & Arguments**
- Pass by value (copies `Copy` types)
- Pass by reference (`&` or `&mut`)

```rust
fn modify(s: &mut String) {
    s.push_str("!");
}

fn take_ownership(s: String) {
    // s destroyed at end of scope
}
```

---

## **3. Return Behavior**
- Last expression = implicit return
- `()` = unit type (default return)
- Early returns use `return` keyword

```rust
fn maybe_number(flag: bool) -> Option<i32> {
    if flag {
        Some(42)  // Implicit return
    } else {
        None     // Implicit return
    }
}

fn early_exit(x: i32) -> i32 {
    if x < 0 { return 0; }
    x * 2
}
```

---

## **4. Associated Functions & Methods**
- **Associated functions**: No `self` parameter (`String::new()`)
- **Methods**: Take `self`, `&self`, or `&mut self`

```rust
struct Counter { count: u32 }

impl Counter {
    // Associated function
    fn new() -> Self {
        Counter { count: 0 }
    }

    // Method
    fn increment(&mut self) {
        self.count += 1;
    }
}
```

---

## **5. Function Pointers**
- Type: `fn(arg_types) -> return_type`
- Can pass regular functions (not closures)

```rust
fn square(x: i32) -> i32 { x * x }

fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

let result = apply(square, 5);  // 25
```

---

## **6. Closures**
- Anonymous functions capturing environment
- Three forms: `|x| x + 1`, `|x| { x + 1 }`, explicit types

```rust
let adder = |a, b| a + b;
let sum = adder(3, 4);

let mut counter = 0;
let mut inc = || {
    counter += 1;
    counter
};
```

### **Closure Types**
| Type          | Syntax              | Captures       |
|---------------|---------------------|----------------|
| Fn            | `impl Fn()`         | Immutable      |
| FnMut         | `impl FnMut()`      | Mutable        |
| FnOnce        | `impl FnOnce()`     | Takes ownership|

---

## **7. Generics**
- Type parameters in angle brackets
- Constrained with trait bounds

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

---

## **8. Lifetime Parameters**
- Required when returning references
- Ensure input/output lifetimes relate

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```

---

## **9. Diverging Functions**
- Never return (`!` type)
- Used in panics or infinite loops

```rust
fn panic_now() -> ! {
    panic!("Abort!");
}

fn infinite_loop() -> ! {
    loop {
        // Runs forever
    }
}
```

---

## **10. Advanced Return Patterns**
### **impl Trait**
```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

### **Multiple Error Types**
```rust
fn fallible_op() -> Result<i32, Box<dyn std::error::Error>> {
    let x: i32 = "42".parse()?;
    Ok(x)
}
```

---

## **11. Edge Cases & Tricks**
### **Empty Return**
```rust
fn no_op() {}  // Returns ()
```

### **Recursive Functions**
```rust
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n-1) }
}
```

### **Turbofish Syntax**
```rust
let num = "42".parse::<i32>().unwrap();
```

---

## **12. Comparisons**
### **Functions vs Closures**
| Feature          | Function            | Closure          |
|------------------|---------------------|------------------|
| Type Name        | Explicit            | Anonymous        |
| Environment      | No capture          | Can capture      |
| Size             | Fixed               | Variable (state) |
| Performance      | Optimized           | May have overhead|

### **Rust vs Other Languages**
- **No overloading**: Use enums or traits instead
- **No default args**: Use `Option<T>` or builder pattern
- **No variadic functions**: Use slices/tuples for multiple args

---

## **13. Tricky Parts**
### **Lifetime Elision**
```rust
// These three declarations are equivalent
fn first_word(s: &str) -> &str;
fn first_word<'a>(s: &'a str) -> &'a str;
fn first_word<'a>(s: &'a str) -> &str;
```

### **Method Resolution**
```rust
struct Data;

impl Data {
    fn process(&self) { /* &self method */ }
    fn process(self) { // self-by-value method
        // Transfers ownership
    }
}
```

### **Unsafe Functions**
```rust
unsafe fn dangerous() {
    // Potentially unsafe operations
}
```