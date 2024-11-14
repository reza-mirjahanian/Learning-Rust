### Function Declaration

- **Basic Syntax**:
  ```rust
  fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
      // function body
  }
  ```
- **Example**:
  ```rust
  fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  ```

### Parameters

- **By Value**:
  - Ownership is moved to the function.
  - Example:
    ```rust
    fn take_ownership(s: String) {
        println!("{}", s);
    }
    ```

- **By Reference**:
  - Borrow data without taking ownership.
  - Immutable reference:
    ```rust
    fn borrow(s: &String) {
        println!("{}", s);
    }
    ```
  - Mutable reference:
    ```rust
    fn modify(s: &mut String) {
        s.push_str(" modified");
    }
    ```

- **Multiple Parameters**:
  - Functions can take multiple parameters of different types.
  - Example:
    ```rust
    fn concatenate(a: &str, b: &str) -> String {
        format!("{}{}", a, b)
    }
    ```

### Return Types

- **Explicit Return**:
  - Use `->` to specify return type.
  - Example:
    ```rust
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    ```

- **Implicit Return**:
  - Last expression without a semicolon is returned.
  - Example:
    ```rust
    fn divide(a: f64, b: f64) -> f64 {
        a / b
    }
    ```

- **Early Return**:
  - Use `return` keyword to exit early.
  - Example:
    ```rust
    fn check_positive(n: i32) -> bool {
        if n < 0 {
            return false;
        }
        true
    }
    ```

- **Multiple Return Values**:
  - Return tuples for multiple values.
  - Example:
    ```rust
    fn swap(a: i32, b: i32) -> (i32, i32) {
        (b, a)
    }
    ```

### Function Types

- **Free Functions**:
  - Defined outside of any `impl` block.
  - Example:
    ```rust
    fn greet() {
        println!("Hello!");
    }
    ```

- **Associated Functions**:
  - Defined within an `impl` block but do not take `self`.
  - Often used as constructors.
  - Example:
    ```rust
    struct Circle {
        radius: f64,
    }

    impl Circle {
        fn new(radius: f64) -> Circle {
            Circle { radius }
        }
    }
    ```

- **Methods**:
  - Functions that take `self` as a parameter.
  - Can be:
    - `self` (by value)
    - `&self` (immutable reference)
    - `&mut self` (mutable reference)
  - Example:
    ```rust
    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn set_radius(&mut self, radius: f64) {
            self.radius = radius;
        }
    }
    ```

### Ownership and Borrowing

- **Passing Ownership**:
  - Function takes ownership of the parameter.
  - Original variable is invalidated.
  - Example:
    ```rust
    fn consume(s: String) {
        println!("{}", s);
    }
    ```

- **Borrowing**:
  - Functions can borrow parameters immutably or mutably.
  - Ensures the original data remains accessible.
  - Example:
    ```rust
    fn display(s: &String) {
        println!("{}", s);
    }
    ```

### Generic Functions

- **Definition**:
  - Use angle brackets with type parameters.
  - Example:
    ```rust
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    ```

- **Multiple Type Parameters**:
  - Separate with commas.
  - Example:
    ```rust
    fn compare<T: PartialOrd, U: std::fmt::Debug>(a: T, b: T) -> bool {
        println!("{:?}", b);
        a > b
    }
    ```

### Closures

- **Definition**:
  - Anonymous functions defined using `|parameters| { body }`.
  - Example:
    ```rust
    let add = |a: i32, b: i32| -> i32 {
        a + b
    };
    ```

- **Capturing Environment**:
  - Closures can capture variables from their enclosing scope.
  - Example:
    ```rust
    let multiplier = 2;
    let multiply = |x| x * multiplier;
    ```

- **Type Inference**:
  - Often, parameter and return types are inferred.
  - Example:
    ```rust
    let square = |x| x * x;
    ```

### Higher-Order Functions

- **Functions that Take or Return Other Functions**.
- **Example**:
  ```rust
  fn apply<F>(f: F) -> i32
  where
      F: FnOnce() -> i32,
  {
      f()
  }

  let five = apply(|| 5);
  ```

### Function Pointers

- **Definition**:
  - Use `fn` keyword followed by signature.
  - Example:
    ```rust
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let func: fn(i32, i32) -> i32 = add;
    ```

- **Usage with Higher-Order Functions**:
  - Pass functions as arguments.
  - Example:
    ```rust
    fn operate(a: i32, b: i32, f: fn(i32, i32) -> i32) -> i32 {
        f(a, b)
    }

    let result = operate(2, 3, add);
    ```

### Default Parameters

- **Rust Does Not Support Default Parameters Directly**.
- **Workarounds**:
  - Function Overloading via Traits.
  - Using Option Types.
  - Example with Option:
    ```rust
    fn greet(name: Option<&str>) {
        match name {
            Some(n) => println!("Hello, {}!", n),
            None => println!("Hello!"),
        }
    }

    greet(Some("Alice"));
    greet(None);
    ```

### Variadic Functions

- **Rust’s Support**:
  - Limited to calling C variadic functions via FFI.
  - Example:
    ```rust
    extern "C" {
        fn printf(format: *const i8, ...) -> i32;
    }
    ```

### Inline and Optimization Attributes

- **`#[inline]`**:
  - Suggests to the compiler to inline the function.
  - Example:
    ```rust
    #[inline]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ```

- **`#[inline(always)]` and `#[inline(never)]`**:
  - Forcefully inline or prevent inlining.
  - Example:
    ```rust
    #[inline(always)]
    fn always_inline() { /* ... */ }

    #[inline(never)]
    fn never_inline() { /* ... */ }
    ```

### Doc Comments

- **Single Line**:
  - Use `///` for documentation.
  - Example:
    ```rust
    /// Adds two numbers.
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ```

- **Inner Doc Comments**:
  - Use `//!` for module-level docs.
  - Example:
    ```rust
    //! This module handles mathematical operations.
    ```

### Testing Functions

- **Unit Tests**:
  - Use `#[cfg(test)]` and `#[test]` attributes.
  - Example:
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            assert_eq!(add(2, 3), 5);
        }
    }
    ```

- **Integration Tests**:
  - Placed in `tests/` directory as separate files.
  - Example:
    ```rust
    // tests/integration_test.rs
    use my_crate::add;

    #[test]
    fn test_add_integration() {
        assert_eq!(add(10, 20), 30);
    }
    ```

### Best Practices

- **Clear Naming Conventions**:
  - Use snake_case for function names.
  - Example: `fn calculate_sum() { /* ... */ }`

- **Use Descriptive Names**:
  - Function names should convey purpose.
  - Example: `fn fetch_user_data(user_id: u32) -> User { /* ... */ }`

- **Minimal Side Effects**:
  - Functions should ideally perform a single task.

- **Use Immutable References When Possible**:
  - Prefer `&T` over `&mut T` to enhance safety and concurrency.

- **Leverage Generics and Traits**:
  - For flexibility and reusability.
  - Example:
    ```rust
    fn print_item<T: std::fmt::Display>(item: T) {
        println!("{}", item);
    }
    ```

- **Handle Errors Gracefully**:
  - Use `Result` and `Option` types.
  - Example:
    ```rust
    fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }
    ```

### Common Patterns

- **Builder Pattern**:
  - For constructing complex objects.
  - Example:
    ```rust
    struct Config {
        debug: bool,
        verbose: bool,
    }

    impl Config {
        fn new() -> Self {
            Config { debug: false, verbose: false }
        }

        fn debug(mut self, debug: bool) -> Self {
            self.debug = debug;
            self
        }

        fn verbose(mut self, verbose: bool) -> Self {
            self.verbose = verbose;
            self
        }
    }

    let config = Config::new().debug(true).verbose(true);
    ```

- **Factory Functions**:
  - Associated functions that create instances.
  - Example:
    ```rust
    impl Circle {
        fn with_radius(radius: f64) -> Circle {
            Circle { radius }
        }
    }

    let c = Circle::with_radius(5.0);
    ```

### Advanced Features

- **Higher-Kinded Types via Traits**:
  - Simulate higher-kinded types using traits for more abstract functions.

- **Function Traits**:
  - **`Fn`**, **`FnMut`**, **`FnOnce`**:
    - Define how closures capture variables.
    - **`Fn`**: Immutable references.
    - **`FnMut`**: Mutable references.
    - **`FnOnce`**: Takes ownership.

- **Dynamic Function Dispatch**:
  - Use trait objects for dynamic calling.
  - Example:
    ```rust
    fn execute(f: &dyn Fn()) {
        f()
    }

    let closure = || println!("Executing closure!");
    execute(&closure);
    ```

### Error Handling in Functions

- **Using `Result`**:
  - Return `Result` for functions that can fail.
  - Example:
    ```rust
    fn read_file(path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
    ```

- **Propagating Errors with `?`**:
  - Simplify error propagation.
  - Example:
    ```rust
    fn read_and_print(path: &str) -> Result<(), std::io::Error> {
        let content = read_file(path)?;
        println!("{}", content);
        Ok(())
    }
    ```

### Inline Documentation and Examples

- **Code Examples in Docs**:
  - Ensures examples are tested.
  - Example:
    ```rust
    /// Adds two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// let sum = my_crate::add(2, 3);
    /// assert_eq!(sum, 5);
    /// ```
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ```

### Attribute Macros

- **Custom Attributes**:
  - Enhance function behavior.
  - Example:
    ```rust
    #[inline]
    fn optimized_add(a: i32, b: i32) -> i32 {
        a + b
    }
    ```

- **`#[allow]` and `#[deny]`**:
  - Control compiler warnings.
  - Example:
    ```rust
    #[allow(dead_code)]
    fn unused_function() { /* ... */ }
    ```

### Recursive Functions

- **Definition**:
  - Functions that call themselves.
  - Example:
    ```rust
    fn factorial(n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }
    ```

- **Tail Recursion**:
  - Optimized by the compiler for certain cases.
  - Example:
    ```rust
    fn tail_recursive_factorial(n: u32, acc: u32) -> u32 {
        if n == 0 {
            acc
        } else {
            tail_recursive_factorial(n - 1, acc * n)
        }
    }
    ```

### Function Overloading

- **Rust Does Not Support Traditional Overloading**.
- **Alternatives**:
  - Use different function names.
  - Use generic functions with trait bounds.
  - Example:
    ```rust
    fn print_i32(x: i32) {
        println!("i32: {}", x);
    }

    fn print_f64(x: f64) {
        println!("f64: {}", x);
    }
    ```

### Function Visibility

- **Public Functions**:
  - Use `pub` keyword to make functions accessible outside the module.
  - Example:
    ```rust
    pub fn public_function() { /* ... */ }
    ```

- **Private Functions**:
  - Default visibility, accessible only within the module.
  - Example:
    ```rust
    fn private_function() { /* ... */ }
    ```

### Async Functions

- **Definition**:
  - Defined with `async` keyword.
  - Return a `Future`.
  - Example:
    ```rust
    async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        Ok(body)
    }
    ```

- **Calling Async Functions**:
  - Use `.await` within an `async` context.
  - Example:
    ```rust
    #[tokio::main]
    async fn main() {
        match fetch_data("https://example.com").await {
            Ok(data) => println!("{}", data),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    ```

### Unsafe Functions

- **Definition**:
  - Use `unsafe` keyword to perform operations that the compiler can't guarantee safety for.
  - Example:
    ```rust
    unsafe fn dangerous() {
        // Unsafe operations
    }
    ```

- **Calling Unsafe Functions**:
  - Must be done within an `unsafe` block.
  - Example:
    ```rust
    unsafe {
        dangerous();
    }
    ```

### Function Pointers vs Closures

| Aspect               | Function Pointers                  | Closures                           |
|----------------------|------------------------------------|------------------------------------|
| Syntax               | Named functions (`fn`)             | Anonymous (`|args| { body }`)      |
| Capture Variables    | Cannot capture                         | Can capture from environment       |
| Type                 | Fixed type based on signature      | Implement `Fn`, `FnMut`, `FnOnce`  |
| Use Cases            | Simple callbacks, FFI              | Inline small functions, higher-order functions |

### Examples

- **Simple Function**:
  ```rust
  fn greet(name: &str) {
      println!("Hello, {}!", name);
  }
  ```

- **Function with Multiple Parameters and Return**:
  ```rust
  fn calculate(a: i32, b: i32) -> (i32, i32) {
      (a + b, a * b)
  }
  ```

- **Generic Function with Trait Bounds**:
  ```rust
  fn largest<T: PartialOrd>(list: &[T]) -> &T {
      let mut largest = &list[0];
      for item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }
  ```

- **Closure Example**:
  ```rust
  let numbers = vec![1, 2, 3];
  let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
  ```

- **Higher-Order Function Example**:
  ```rust
  fn apply<F>(f: F) -> i32
  where
      F: FnOnce() -> i32,
  {
      f()
  }

  let result = apply(|| 10);
  ```

### Tables for Reference

#### Function Attributes

| Attribute           | Description                                        | Example                                    |
|---------------------|----------------------------------------------------|--------------------------------------------|
| `#[inline]`         | Suggests the compiler to inline the function.      | `#[inline] fn add(a: i32, b: i32) -> i32 { a + b }` |
| `#[allow(...)]`     | Allows specific warnings within the function.      | `#[allow(dead_code)] fn unused() {}`       |
| `#[deny(...)]`      | Denies specific warnings, causing compilation error. | `#[deny(unused_variables)] fn func(x: i32) {}` |

#### Function Traits

| Trait     | Description                                         | Usage Example                               |
|-----------|-----------------------------------------------------|---------------------------------------------|
| `Fn`      | For closures that do not mutate captured variables. | `fn apply<F: Fn()>(f: F) { f(); }`          |
| `FnMut`   | For closures that may mutate captured variables.    | `fn apply_mut<F: FnMut()>(mut f: F) { f(); }` |
| `FnOnce`  | For closures that consume captured variables.       | `fn apply_once<F: FnOnce()>(f: F) { f(); }` |

### Additional Tips

- **Minimize Side Effects**:
  - Aim for functions to have clear input-output behavior without hidden state changes.

- **Use Traits for Shared Behavior**:
  - Define shared functionality across different types.
  - Example:
    ```rust
    trait Printable {
        fn print(&self);
    }

    impl Printable for Circle {
        fn print(&self) {
            println!("Circle with radius {}", self.radius);
        }
    }
    ```

- **Leverage Documentation Tools**:
  - Use `cargo doc` to generate HTML documentation from doc comments.

- **Consistent Error Handling**:
  - Prefer using `Result` and `Option` over panicking.
  - Example:
    ```rust
    fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
    ```

- **Optimize for Performance When Necessary**:
  - Use `#[inline]` and other optimization hints judiciously based on profiling data.

- **Keep Functions Small and Focused**:
  - Each function should have a single responsibility for better readability and maintainability.

- **Avoid Deep Nesting**:
  - Refactor deeply nested code into smaller helper functions.

- **Consistent Parameter Ordering**:
  - Maintain a consistent order for parameters across similar functions to reduce cognitive load.

- **Utilize Pattern Matching**:
  - Use `match` statements within functions to handle different cases clearly.

    ```rust
    fn describe_number(n: i32) -> &'static str {
        match n {
            0 => "zero",
            1..=9 => "single digit",
            _ => "multiple digits",
        }
    }
    ```

- **Prefer Immutable Variables**:
  - Use `let` instead of `let mut` unless mutation is necessary.

    ```rust
    fn add_one(x: i32) -> i32 {
        let y = x + 1;
        y
    }
    ```

