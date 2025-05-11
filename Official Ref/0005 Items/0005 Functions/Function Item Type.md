
> When referred to, a **function** yields a first-class **value** of the corresponding zero-sized **function item type**, which when called evaluates to a direct call to the function.



---

### 1. **Functions are "First-Class Values"**

- In Rust, **functions themselves** are **values** you can:
  - Pass around,
  - Store in variables,
  - Use as arguments to other functions.

This is similar to how numbers (`i32`, `f64`, etc.) are "first-class" — you can use them freely.

---
**Example:**
```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

let f = add_one; // We assign the function itself to a variable `f`

let result = f(10); // `f` behaves like `add_one`
println!("{result}"); // prints 11
```
- `f` is now a **value** referring to the `add_one` function.
- You can **call** it with `f(10)`.

---

### 2. **Function Item Type**

Rust is *extremely* strict about types.
Even **functions** have **types**!

When you write a function like:
```rust
fn answer_to_life_the_universe_and_everything() -> i32 {
    42
}
```

- Internally, **`answer_to_life_the_universe_and_everything`** is a **function item type**.
- A *function item type* is a **unique, zero-sized** type for **that exact function**.

> Each function you define has its **own anonymous type**, specific only to that function.

It’s **not** just “function pointer” (`fn() -> i32`), but a *specific* **function item** with its own identity.

---
**Quick notes:**
| Concept                      | Meaning |
|:-----------------------------|:--------|
| **Function item**             | The actual function itself, with its own anonymous type. |
| **Function pointer (`fn()`)** | A pointer to any function matching a signature (can point to multiple functions). |

- A **function item** knows exactly what function it is.
- A **function pointer** (`fn(...) -> ...`) could point to any matching function.

You can **coerce** a function item into a function pointer automatically:

```rust
let x: fn() -> i32 = answer_to_life_the_universe_and_everything;
```
Now `x` is a **function pointer** type (`fn() -> i32`).

---

### 3. **Zero-Sized**

- **Function item types are zero-sized.**
- They carry **no data** — because **the compiler already knows exactly** which function they refer to!
- In memory, they occupy **zero bytes**.

> In Rust, zero-sized types (ZSTs) are types that take no space at runtime.

Thus, carrying a function item around costs **nothing**.

---
**Example of Zero-Sized Type (ZST):**
```rust
struct EmptyStruct;

let empty = EmptyStruct; // takes zero bytes
```
Similarly:
```rust
let f = answer_to_life_the_universe_and_everything; 
// `f` is a ZST representing the function
```

---

### 4. **Direct Call**

When you **call** a function item (e.g., `f()`), Rust **directly** jumps to the code of that function.

- No dynamic lookup (like virtual functions).
- No vtable.
- No function pointer indirection (unless you forced it).

This is why calling a function item is **fast** — it’s like a hardcoded jump instruction.

---

### Full Illustration

```rust
fn example() -> i32 {
    5
}

fn main() {
    let a = example; // `a` is a function item, zero-sized
    let b: fn() -> i32 = example; // `b` is a function pointer
    
    let val1 = a(); // direct call
    let val2 = b(); // indirect call through function pointer

    println!("{val1} {val2}"); // 5 5
}
```
| Variable | Type                 | Call style             | Runtime cost |
|:---------|:---------------------|:-----------------------|:-------------|
| `a`       | *function item*       | direct call             | **very fast** |
| `b`       | `fn() -> i32` pointer | function pointer call   | slightly slower |

---

### Summary
- A **function** in Rust is a **zero-sized value** when referred to.
- Each function gets its **own unique anonymous type** called a **function item type**.
- You can **store** functions in variables without overhead.
- Calling a **function item** is a **direct jump**, very fast.
- You can **coerce** it into a **function pointer** (`fn(...) -> ...`) when you need more flexibility (e.g., store different functions in an array).

