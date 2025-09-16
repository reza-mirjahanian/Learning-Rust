# 🦀 Rust’s Witchcraft: The Power of Macros

---


---

## 🔮 Why Macros Are Powerful

* **Not just syntax sugar**
  Other languages often restrict macros to simple *syntax rewriting*.
* **Rust’s difference**
  In Rust, the **entire language** is available at **compile-time** as well as runtime.
* **Historical roots**
  Lisp pioneered this idea half a century ago. Rust continues this tradition with modern tooling.

> 💡 *Think of a macro as a function that runs at compile-time and rewrites your code before it ever runs.*

---

## 🛠️ Declarative Macros (`macro_rules!`)

Declarative macros are like **match statements at compile-time**.
They allow you to **template code by example**.

### Example: Simple Macro

```rust
macro_rules! say_hello {
    () => {
        println!("Hello");
    };
}
```

📌 **Usage**

```rust
say_hello!(); // expands to println!("Hello");
```

---

### Example: Macro with Parameters

```rust
macro_rules! bounded_impl {
    ($T:ty, $min:expr, $max:expr) => {
        impl Bounded for $T {
            fn min_value() -> Self { $min }
            fn max_value() -> Self { $max }
        }
    };
}
```

📌 **Usage**

```rust
bounded_impl!(u8, u8::MIN, u8::MAX);
```

📋 **Expansion**

```rust
impl Bounded for u8 {
    fn min_value() -> Self { u8::MIN }
    fn max_value() -> Self { u8::MAX }
}
```

---

## ⚡ Procedural Macros

Procedural macros go beyond declarative ones:

* Can execute **arbitrary code** at compile-time.
* Useful for building **DSLs**, embedding other languages, or doing **compile-time checks**.

### 🔗 Types of Procedural Macros

1. **Custom Derive Macros**

   * Add behavior to structs and enums.
   * Example: `#[derive(Debug, Clone)]`

2. **Attribute-like Macros**

   * Define custom attributes.
   * Example: `#[route(GET, "/")] fn index() {}`

3. **Function-like Macros**

   * Look like normal function calls but work on tokens.
   * Example: `html! { <div>"Hello"</div> }`

---

## 🌍 Real-World Magic

### 📑 HTML in Rust

Rust doesn’t have native HTML/XML literals, but macros let you embed them:

```rust
html! {
    <h1>"Hello, world!"</h1>
}
```

✅ Syntax highlighting
✅ Compile-time checks

---

### 🧠 Embedding a Lisp

A Lisp macro inside Rust:

```rust
lisp! {
    (def factorial (fn (n)
        (if (<= n 1)
            1
            (* n (factorial (- n 1))))))
}
```

📌 **Expansion** → Normal Rust functions at compile-time.

Errors inside the Lisp code show up as **native Rust compiler errors**, directly in your IDE.

---

### 🗄️ SQL at Compile Time (SQLx)

```rust
let users = sqlx::query!("SELECT * FROM users WHERE age > $1", 21)
    .fetch_all(&pool)
    .await?;
```

🔮 What happens at compile-time:

1. `query!` runs with sample data.
2. Executes against your **local dev database**.
3. Verifies column names, parameter types, and syntax.
4. If invalid → compiler error *at the exact line*.

---

## 📊 Macro Types in Rust

| Macro Type         | Example              | Purpose                                |
| ------------------ | -------------------- | -------------------------------------- |
| **Declarative**    | `macro_rules!`       | Pattern-matching & code templating     |
| **Custom Derive**  | `#[derive(Debug)]`   | Auto-implement traits                  |
| **Attribute-like** | `#[route(GET, "/")]` | Define custom attributes for items     |
| **Function-like**  | `sqlx::query!(...)`  | Token-based transformations at compile |

---

## 🎁 Key Takeaway

Rust macros unlock **impossible-seeming features**:

* Write **less boilerplate**.
* Build **DSLs inside Rust**.
* Get **compile-time guarantees** other languages can’t offer.

---


