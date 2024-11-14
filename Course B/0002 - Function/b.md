### Functions in Rust

**1. Defining Functions:**

* **Basic Syntax:**

```rust
fn function_name(parameter1: type1, parameter2: type2) -> return_type {
    // Function body
    // ...

    return value; // Optional if the last expression is implicitly returned
}
```

* **No Return:** If a function doesn't return a value, the return type is `()`.  It can be omitted:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

* **Implicit Returns:** The last expression in a function body is implicitly returned if no explicit `return` is used (and a return type is specified).

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y 
}
```

**2. Parameters and Arguments:**

* **Type Safety:**  Rust enforces type checking for function parameters.

* **Passing by Value:**  By default, arguments are passed by value (copied).

* **Passing by Reference:** Use references (`&`) to pass arguments without copying:

```rust
fn print_length(s: &str) {
    println!("String length: {}", s.len());
}
```

* **Mutable References:** Use mutable references (`&mut`) to modify arguments within the function:

```rust
fn change_string(s: &mut String) {
    s.push_str(" - Modified!");
}
```


**3. Function Pointers:**

* **Type:** Function pointers have the type `fn(parameter_types) -> return_type`.

* **Example:**

```rust
fn add(x: i32, y: i32) -> i32 { x + y }

fn main() {
    let f: fn(i32, i32) -> i32 = add; // Assigning the function to a variable
    let result = f(5, 10);
    println!("{}", result); // Output: 15
}
```

**4. Closures:**

* **Anonymous Functions:** Closures are anonymous functions that can capture variables from their surrounding scope.

* **Syntax:**

```rust
let closure = |x: i32| x * 2;
let result = closure(5); // result is 10
```

* **Capturing Variables:**

```rust
let multiplier = 2;
let closure = |x: i32| x * multiplier; // Captures 'multiplier'
```

**5. Generic Functions:**

* **Flexibility:**  Write functions that work with various types using type parameters.

* **Syntax:**

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // ... (function body)
}
```

**6. Methods:**

* **Associated Functions:** Functions defined within an `impl` block but *without* `self` as a parameter.  Often used as constructors.

    ```rust
    struct Point { x: i32, y: i32 }

    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }
    ```

* **Methods:** Functions within an `impl` block that take `self` as a parameter.  Operate on instances of the struct.

    ```rust
    impl Point {
        fn distance_from_origin(&self) -> f64 {
            // ...
        }
    }
    ```


**7. Traits and Methods:**

* **Defining Behavior:** Traits define shared behavior (methods) that types can implement.

* **Example:**

```rust
trait Printable {
    fn print(&self);
}
```


**8. Diverging Functions:**

* **Never Return:** Functions that never return (e.g., `panic!` or `loop {}`).  Denoted with `!` as the return type.


```rust
fn forever() -> ! {
    loop {}
}
```

