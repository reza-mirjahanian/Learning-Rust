

In simple terms:

- In Rust, when you define **function parameters**, you're actually **destructuring** or **pattern matching** the arguments — just like in a `let` binding **without `else`**.
- The pattern **must match 100% of the time** — it **cannot fail** (because there is no `else` in function calls).

This is why **only *irrefutable* patterns** are allowed.

---

# ✅ Irrefutable Pattern

- An **irrefutable pattern** is one that **always** matches the value.
- Example: matching a tuple `(i32, i32)` with `(x, y)` — always succeeds.

**In contrast:**  
A **refutable pattern** (like `Some(x)`) can **fail** if you get a `None`, so it is **NOT allowed** directly in parameters without extra checks.

---

# ⚡ Your example:

```rust
fn first((value, _): (i32, i32)) -> i32 { 
    value 
}
```

### Step-by-Step:
- Input type is `(i32, i32)` → always two integers, so matching will **never fail**.
- Inside the parameter list: `(value, _)`
  - `value` **captures** the **first** element.
  - `_` means **"ignore"** the **second** element — **don't bind it to a name**.

Thus, when you call:

```rust
let result = first((10, 20));
println!("{result}"); // 10
```

It:
- **takes** the first `10` into `value`.
- **ignores** `20` entirely.

---

# 🎯 Why and When We Use Underscore `_`

| When we use `_` | Why |
|:----------------|:----|
| We **don't care** about a value. | Save memory, avoid warnings about unused variables. |
| To **ignore parts** of a structure (tuple, struct, enum). | Focus only on parts you need. |
| When writing **generic or temporary code** where only some fields matter. | Clean and future-proof. |

---

# 🔥 Practical Examples

### **Tuple Destructuring**

```rust
fn process_point((x, _): (i32, i32)) {
    println!("X coordinate: {x}");
}
```
- We **ignore** the y-coordinate.

---

### **Struct Destructuring**

```rust
struct Person {
    name: String,
    age: u32,
}

fn greet(Person { name, .. }: Person) {
    println!("Hello, {name}!");
}
```
- `..` means **"ignore all other fields"**.
- Only **`name`** is bound; `age` is ignored.

---

### **Enum Matching**

```rust
enum MyEnum {
    Foo(i32, i32),
}

fn handle_foo(MyEnum::Foo(x, _): MyEnum) {
    println!("Got x: {x}");
}
```
- We take only the first part of `Foo`.

---

### **Ignoring Entire Parameters**

```rust
fn dummy(_: i32) {
    println!("I don't care about the input!");
}
```
- We **completely ignore** the argument.

---

# ⚡ Important: Only Irrefutable Patterns Allowed in Function Parameters

**Allowed patterns**:
| Pattern Example               | Why allowed? |
|:-------------------------------|:-------------|
| `x`                            | Always matches |
| `(x, y)`                       | Tuple always matches |
| `(x, _)`                       | Tuple with ignored part always matches |
| `Struct { field, .. }`          | Always matches a struct |

**NOT allowed directly**:
| Pattern Example               | Why NOT allowed? |
|:-------------------------------|:-----------------|
| `Some(x)`                      | Might be `None` (failure possible) |
| `Ok(x)`                        | Might be `Err(_)` |
| `Err(e)`                       | Might be `Ok(_)` |

---

# 🚀 Final Key Takeaways

- **Function parameters behave like an irrefutable `let` binding**.
- Use **`_`** to **ignore** values you don't need.
- Only use patterns that **always match** (no `Some(_)`, `Ok(_)`, etc.).
- It helps in writing **clearer**, **more efficient**, **warning-free** code.

---

