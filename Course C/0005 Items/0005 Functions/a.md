**Basic Function Syntax**  
```rust
// fn <name>(<params>) -> <ReturnType> { <body> }
fn add(x: i32, y: i32) -> i32 {
    x + y   // implicit return (no semicolon)
}

fn greet() {
    println!("Hello, world!"); // returns ()
}
```

**Key Points:**  
- **Parameter types** are mandatory.  
- **Return type** omitted ⇒ returns `()`.  
- **Last expression without `;`** is returned.

---

**Parameters & Return Types**  
- Multiple, zero or tuple params.  
- Return tuple, array, etc.  
```rust
fn swap(a: i32, b: i32) -> (i32, i32) { (b, a) }

fn nothing() -> () { }     // explicit unit
fn nothing_implicit() { }  // implicit unit
```

---

**Ownership & Borrowing**  
```rust
fn takes_ownership(s: String) { /* s moved */ }
fn borrows(s: &String) { /* s borrowed */ }
fn mutable_borrow(s: &mut String) { s.push_str("!"); }
fn returns_owned(s: String) -> String { s }  // move back
```

**Tricky:** Returning a reference requires lifetimes:
```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}
```

---

**Implicit Returns & Semicolons**  
```rust
fn f1() -> i32 {
    5          // return 5
}

fn f2() -> i32 {
    return 5;  // explicit return
}

fn f3() {
    let _ = 1; // semicolon ⇒ no return from this line
}
```

- A line ending with `;` yields `()`.  
- Mixing `return` and expression return: allowed but avoid confusion.

---

**Diverging Functions (`!`)**  
```rust
fn forever() -> ! {
    loop { }
}

fn error_and_exit(msg: &str) -> ! {
    panic!("{}", msg);
}
```
- Never returns; useful in generics for “unreachable” branches.

---

**Generic Functions**  
```rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}
```
- **Trait bounds** required to call methods/operators.  
- **Where clauses** for readability:
  ```rust
  fn concat<T, U>(x: T, y: U) -> String
  where
      T: ToString,
      U: ToString,
  {
      x.to_string() + &y.to_string()
  }
  ```

---

**Associated Functions vs. Methods**  
```rust
struct Point { x: f64, y: f64 }

impl Point {
    // Associated function (no self)
    fn origin() -> Self { Point { x:0.0, y:0.0 } }

    // Method (takes self)
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
```

---

**Closures & Fn Traits**  

| Trait    | Signature           | Captures         | Can be called multiple times |
|----------|---------------------|------------------|------------------------------|
| `Fn`     | `&self`             | by reference     | ✅                            |
| `FnMut`  | `&mut self`         | by mutable ref   | ✅                            |
| `FnOnce` | `self`              | by value (move)  | ❌ (once if consumes)         |

```rust
let x = 5;
let c = |y: i32| x + y;         // Fn: x by reference
let mut z = 0;
let mut incr = |i: i32| { z += i; }; // FnMut
let s = String::from("hi");
let consume = move || println!("{}", s); // FnOnce
```

---

**Function Pointers**  
```rust
fn square(x: i32) -> i32 { x * x }
let f: fn(i32) -> i32 = square;
let v = vec![1,2,3].into_iter().map(f).collect::<Vec<_>>(); 
```

---

**Higher-Order Functions**  
```rust
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}
```

---

**Recursion & Tail Calls**  
```rust
fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}
```
- **Rust does not guarantee tail-call optimization**; avoid deep recursion.

---

**Async Functions**  
```rust
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}
```
- Returns `impl Future<Output = ...>`.  
- Can’t use `?` without `Result`.  
- Use `.await` in async context or spawn tasks.

---

**Unsafe Functions & FFI**  
```rust
extern "C" {
    fn c_func(x: i32) -> i32;
}

unsafe fn raw_ptr_deref(p: *const i32) -> i32 {
    *p
}

#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

**Attributes & Inlining**  
```rust
#[inline]        // hint to optimizer
#[inline(always)]
#[inline(never)]
fn heavy() { /* ... */ }

#[track_caller]  // better panic info
fn check(cond: bool) {
    if !cond { panic!("Check failed"); }
}
```

---

**Error Handling Patterns**  
```rust
fn parse_and_add(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn maybe_double(x: Option<i32>) -> Option<i32> {
    x.map(|v| v * 2)
}
```

---

**Simulating Default Params & Overloading**  
- Use `Option<T>` for defaults.  
- Use traits (e.g. `From<…>`) or builder pattern.  
- No true function overloading—use traits or enums.

---

**Comparison with Other Languages**  
- **C/C++**: Rust requires explicit ownership/borrowing, no implicit conversions.  
- **Java/C#**: No method overloading by default, but generics are monomorphized.  
- **Python/JS**: Statically typed, mandatory annotations for generics and lifetimes.

---

**Tips & Tricks**  
- **`allow(dead_code)`** to suppress warnings on unused functions.  
- **Trailing commas** in long parameter lists.  
- **Use `#[must_use]`** on functions returning `Result`/`Option`.  
- **Function pointers vs closures**: pointers faster, closures more flexible.  
- **`impl Trait` in args** for concise higher-order fn signatures.  
- **`where` clauses** improve readability for complex bounds.  
- **Diverging functions** (`-> !`) for unreachable code paths.  
- **`#[inline]`** on small generics to reduce code bloat.  
- **Use `self` method chaining** in builders for ergonomic APIs.  
- **Pattern-match tuples** in parameters: `fn foo((x,y): (i32,i32)) {}`.  
- **Be cautious with recursion depth**—consider iteration or explicit stack.
