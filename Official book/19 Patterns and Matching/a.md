
---

# 🧠 Patterns and Matching in Rust — The Ultimate Guide

---

## 🧩 1. **What Is a Pattern?**

A **pattern** is a way to **destructure** types by comparing against the shape of data. Rust patterns are used in:

* `match` expressions
* `if let` and `while let`
* Function parameters
* `let` and `const` bindings
* `for` loops
* `ref` and `ref mut` bindings

---

## 🧱 2. **Basic Patterns**

### ✅ Literal Matching

```rust
let x = 5;

match x {
    1 => println!("One"),
    5 => println!("Five"),
    _ => println!("Something else"),
}
```

* Matches exact values.
* `_` is the **wildcard** (matches anything, ignores the value).

---

### ✅ Variable Binding

```rust
let x = 42;

match x {
    val => println!("Matched value: {val}"),
}
```

* `val` binds to the value.

---

### ✅ Ranges

```rust
let age = 18;

match age {
    0..=12 => println!("Child"),
    13..=19 => println!("Teenager"),
    _ => println!("Adult"),
}
```

* `0..=12` is an **inclusive range pattern**.
* You can’t use exclusive ranges (`..`) in `match`, only in `if`.

---

## 📦 3. **Destructuring Structs and Enums**

### ✅ Structs

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };

match p {
    Point { x, y } => println!("x: {x}, y: {y}"),
}
```

* Also works with field shorthand and ignoring fields:

```rust
match p {
    Point { x, .. } => println!("x is {x}"),
}
```

---

### ✅ Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

let msg = Message::Move { x: 10, y: 20 };

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({x}, {y})"),
    Message::Write(text) => println!("Write: {text}"),
}
```

---

## 🧬 4. **Destructuring Tuples & Arrays**

### ✅ Tuples

```rust
let t = (1, 2);

match t {
    (1, y) => println!("First is 1, second is {y}"),
    _ => println!("Other tuple"),
}
```

### ✅ Arrays and Slices

```rust
let arr = [1, 2, 3];

match arr {
    [1, _, _] => println!("Starts with 1"),
    [_, 2, _] => println!("Second is 2"),
    _ => println!("No match"),
}
```

---

## 🧰 5. **Advanced Pattern Features**

### ✅ Ignoring Values with `_` and `..`

```rust
fn foo(_: i32, y: i32) {
    println!("y is {y}");
}

let tuple = (1, 2, 3, 4);

match tuple {
    (1, .., 4) => println!("Starts with 1 and ends with 4"),
    _ => println!("No match"),
}
```

---

### ✅ `@` Bindings

```rust
let num = Some(7);

match num {
    Some(n @ 1..=10) => println!("Matched number in range: {n}"),
    Some(n) => println!("Other number: {n}"),
    None => println!("No value"),
}
```

* `@` binds the whole matched value to a variable **and** tests its structure.

---

## 🧪 6. **Match Guards**

Add an `if` condition **after a pattern**:

```rust
let x = Some(4);

match x {
    Some(n) if n < 5 => println!("Less than 5: {n}"),
    Some(n) => println!("Other value: {n}"),
    None => println!("None"),
}
```

---

## ♻️ 7. **`if let` and `while let`**

### ✅ `if let`

```rust
let val = Some(3);

if let Some(x) = val {
    println!("Value is {x}");
}
```

* Shorthand for one-match arm.
* Combine with guard:

```rust
if let Some(x) = val && x > 0 {
    println!("Positive value: {x}");
}
```

### ✅ `while let`

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("Popped: {top}");
}
```

---

## 🪝 8. **Ref and Mut in Patterns**

* `ref` and `ref mut` create **references** during pattern matching.

```rust
let tuple = (String::from("Hello"), String::from("World"));

let (ref a, ref b) = tuple;
println!("{a}, {b}");
```

```rust
let mut x = Some(String::from("Hi"));

if let Some(ref mut s) = x {
    s.push_str(", there");
}
```

---

## 🧱 9. **Nested Patterns**

```rust
enum Message {
    ChangeColor((u8, u8, u8)),
}

let msg = Message::ChangeColor((255, 0, 0));

match msg {
    Message::ChangeColor((r, g, b)) => println!("RGB: {r}, {g}, {b}"),
}
```

Nested destructuring works **recursively**.

---

## 🧨 10. **Irrefutable vs Refutable Patterns**

| Pattern Type | Can Fail? | Example Usage    |
| ------------ | --------- | ---------------- |
| Irrefutable  | ❌ No      | `let x = 5`      |
| Refutable    | ✅ Yes     | `if let Some(x)` |

Use **irrefutable** patterns in `let`, function arguments, etc.

```rust
let (x, y) = (1, 2);  // ✅ Irrefutable

let Some(v) = maybe_val; // ❌ Compile error if maybe_val is None
```

Use `if let` instead.

---

## 🧠 11. **Exhaustiveness and the `_` Arm**

Rust enforces **exhaustive matching** for `match`.

```rust
enum Direction { Up, Down }

fn go(dir: Direction) {
    match dir {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        // Missing variant causes compiler error
    }
}
```

You can use `_` as a **catch-all**:

```rust
match some_val {
    Specific(x) => println!("Got it"),
    _ => println!("Something else"),
}
```

---

## 🧮 12. **Pattern Comparison With Other Concepts**

| Concept               | Rust Pattern Match                                  | Equivalent in Other Languages          |
| --------------------- | --------------------------------------------------- | -------------------------------------- |
| `match`               | `switch-case` but **exhaustive and powerful**       | C, JavaScript `switch`, Python `match` |
| `if let`              | Conditional destructuring                           | Python `if isinstance()` + unpack      |
| Destructuring structs | Like object destructuring in JavaScript/Python      | More powerful due to type enforcement  |
| `ref`, `ref mut`      | Explicit control over borrow/reference during match | Unique to Rust's ownership model       |
| `@` bindings          | Simultaneous test and bind                          | Not present in most languages          |

---

## 🔍 13. **Edge Cases and Gotchas**

### ❗ Wildcard `_` binds nothing

```rust
let _ = some_function(); // Calls function but discards result
```

### ❗ Mixing `match` with guards can hide coverage

```rust
match Some(4) {
    Some(x) if x > 5 => println!("Greater"),
    Some(_) => println!("Less or equal"),
    None => println!("None"),
}
```

If you forget the second `Some(_)`, match becomes **incomplete**.

---

## 🏁 14. **Custom Example: Recursive Enum Match with Destructuring**

```rust
enum Expr {
    Val(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Val(x) => *x,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}
```

---






## 🧠 15. **Pattern Ergonomics (Ownership, Borrowing, Mutability)**

Rust automatically **borrows or moves** based on the context. Understanding **pattern ergonomics** helps avoid unnecessary `ref` or `clone`.

---

### 🔧 Example: Match by Reference

```rust
let name = String::from("Alice");

match name {
    ref n => println!("Name is: {}", n),
}
// name is moved unless we use ref
```

* To avoid move: match on a reference.

```rust
match &name {
    n => println!("Ref matched: {}", n),
}
// or
match name.as_str() {
    "Alice" => println!("Matched!"),
    _ => println!("Not matched"),
}
```

---

### 🔧 Example: Mutable Reference

```rust
let mut name = String::from("Alice");

match &mut name {
    n => n.push_str(" Smith"),
}
```

* No need for `ref mut` if matching on `&mut`.

---

### 📜 Summary Table

| Context          | Action Needed               | Example                       |
| ---------------- | --------------------------- | ----------------------------- |
| Match by move    | nothing                     | `match x`                     |
| Match by ref     | `ref` or `match &x`         | `match &x { val => ... }`     |
| Match by mut ref | `ref mut` or `match &mut x` | `match &mut x { val => ... }` |

---

## 🧙 16. **Nested and Deep Patterns (Recursive Matching)**

Rust allows deeply **recursive** patterns.

### 📦 Example: Nested Enums

```rust
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

let expr = Expr::Add(
    Box::new(Expr::Num(1)),
    Box::new(Expr::Mul(Box::new(Expr::Num(2)), Box::new(Expr::Num(3))))
);

match expr {
    Expr::Add(_, box Expr::Mul(_, box Expr::Num(3))) => {
        println!("Ends in * 3");
    },
    _ => println!("Other"),
}
```

* `box` in pattern destructures `Box<T>`.

---

## 🧰 17. **`box` Patterns for Heap Destructuring**

Used for destructuring values inside `Box<T>` (heap-allocated).

### ✅ Usage:

```rust
let val = Box::new(5);

match val {
    box x => println!("Got x = {x}"),
}
```

### ⚠️ Limitation

* Only works in patterns, not expressions.
* Not commonly needed with `&Box<T>`.

---

## 🔀 18. **Or Patterns (`|`)**

Matches **multiple patterns**:

```rust
let x = 1;

match x {
    1 | 2 => println!("One or two"),
    3 => println!("Three"),
    _ => println!("Other"),
}
```

* Works with enums:

```rust
enum Direction { Up, Down, Left, Right }

match Direction::Up {
    Direction::Left | Direction::Right => println!("Horizontal"),
    Direction::Up | Direction::Down => println!("Vertical"),
}
```

* Combine with destructuring:

```rust
let point = (0, 5);

match point {
    (0, y) | (y, 0) => println!("On axis, y = {y}"),
    _ => println!("Not on axis"),
}
```

---

## 🕵️ 19. **Match Guards with Or Patterns**

Guards apply to **entire arm**, not individual subpatterns.

```rust
let x = 4;

match x {
    1 | 2 if x > 1 => println!("2 only"), // x == 1 would still go here, but fails guard
    _ => println!("Other"),
}
```

To apply separate guards:

```rust
match x {
    1 if x == 1 => println!("One"),
    2 if x == 2 => println!("Two"),
    _ => println!("Other"),
}
```

---

## 🧪 20. **Match as Expression (Returning Values)**

```rust
let x = 10;
let label = match x {
    1 => "one",
    2 => "two",
    _ => "many",
};
println!("{label}");
```

* `match` is an **expression**, must be exhaustive.
* Each arm must return the **same type**.

---

## 🎯 21. **Match on Result and Option**

### ✅ Option

```rust
let maybe = Some("data");

match maybe {
    Some(v) => println!("Got: {v}"),
    None => println!("None"),
}
```

* Or use `if let`:

```rust
if let Some(v) = maybe {
    println!("Unwrapped: {v}");
}
```

### ✅ Result

```rust
let res: Result<i32, &str> = Ok(42);

match res {
    Ok(n) => println!("Success: {n}"),
    Err(e) => println!("Error: {e}"),
}
```

---

## 📏 22. **Exhaustiveness with Enums**

Compiler ensures full coverage:

```rust
enum State { Start, Stop }

fn handle(state: State) {
    match state {
        State::Start => println!("Start"),
        State::Stop => println!("Stop"),
    }
}
```

If enum expands, code **won’t compile** until you add the new variant.

Use `_` arm if you want a catch-all:

```rust
match state {
    State::Start => ...,
    _ => ...,
}
```

---

## 🧮 23. **Bindings Inside Patterns**

Bind fields while matching:

```rust
struct Person {
    name: String,
    age: u8,
}

let person = Person { name: "Bob".into(), age: 30 };

match person {
    Person { name: ref n @ "Bob", age } => println!("{n} is {age} years old"),
    _ => {}
}
```

---

## 🧵 24. **Patterns in Loops, Closures, and Functions**

### ✅ Function Arguments

```rust
fn print_coords((x, y): (i32, i32)) {
    println!("{x}, {y}");
}
```

### ✅ Closure Args

```rust
let coords = vec![(0, 1), (2, 3)];

coords.iter().for_each(|&(x, y)| println!("{x}, {y}"));
```

### ✅ Loop Destructuring

```rust
let vec = vec![(1, "a"), (2, "b")];

for (i, val) in vec {
    println!("{i}: {val}");
}
```

---

## 🧬 25. **Pattern Matching in `let` Chains** (Rust 1.65+)

```rust
if let Some(x) = Some(2) && x > 0 {
    println!("Positive: {x}");
}
```

Also works in `while let`, `match`, etc.

---

## ✅ RECAP TABLE

| Feature               | Purpose                      | Example                               |
| --------------------- | ---------------------------- | ------------------------------------- |
| `match`               | Branch by pattern            | `match val { ... }`                   |
| `if let`, `while let` | Simplified match             | `if let Some(x) = opt`                |
| `_`, `..`             | Wildcard and rest            | `(_, ..)`                             |
| `@`                   | Bind with condition          | `Some(x @ 1..=5)`                     |
| `box`                 | Destructure heap             | `box val`                             |
| `ref`, `ref mut`      | Create references in pattern | `let ref x = value`                   |
| Guards (`if`)         | Add conditions to arms       | `match x { Some(n) if n > 5 => ... }` |
| Nested patterns       | Recursive destructuring      | `Message::Move { x: _, y: _ }`        |

---

