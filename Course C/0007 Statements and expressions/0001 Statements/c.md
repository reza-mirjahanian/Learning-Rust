

# Statements in Rust: Technical Reference Guide

---

## Table of Contents

1. [Overview](#overview)
2. [Types of Statements](#types-of-statements)

   * Expression Statements
   * Item Statements
   * Let Statements
   * Semi-colon vs. No Semi-colon
3. [Behavior and Semantics](#behavior-and-semantics)
4. [Internal Compiler Details](#internal-compiler-details)
5. [Memory and Code Generation Effects](#memory-and-code-generation-effects)
6. [Visibility, Scoping, and Lifetime](#visibility-scoping-and-lifetime)
7. [Attributes on Statements](#attributes-on-statements)
8. [Edge Cases and Lesser-Known Features](#edge-cases-and-lesser-known-features)
9. [Limitations, Gotchas, and Non-Obvious Behaviors](#limitations-gotchas-and-non-obvious-behaviors)
10. [Comparison with Other Languages](#comparison-with-other-languages)
11. [Trade-off Comparison Table](#trade-off-comparison-table)

---

# Overview

In Rust, a **statement** is a syntactic construct that either **performs an action** or **binds a name to a value**. Statements control the flow of execution, memory management, and program structure.

---

# Types of Statements

---

## Expression Statements

Evaluate an expression for its **side effect**, **discarding** the resulting value.

```rust
fn main() {
    3 + 4; // Expression statement: result discarded
}
```

* Must **end with a semicolon** (`;`).
* **Converts** an expression into a statement by discarding the value.

---

## Item Statements

Declare **items** like functions, structs, constants, modules, etc., inside other blocks.

```rust
fn main() {
    fn local_function() {} // Item statement inside function
    local_function();
}
```

* **Nested item** (rare outside functions).
* Subject to **normal item visibility** rules.

---

## Let Statements

Bind a value to a **pattern** using `let`.

```rust
fn main() {
    let x = 5; // Let statement
}
```

* Optional type annotation.
* May or may not initialize immediately.

Advanced form with pattern matching:

```rust
let (a, b) = (1, 2);
```

---

## Semicolon vs. No Semicolon

| With Semicolon            | Without Semicolon                        |
| :------------------------ | :--------------------------------------- |
| Converts into a statement | Leaves as expression (value is returned) |

Example:

```rust
fn foo() -> i32 {
    5 // No semicolon → returned
}

fn bar() {
    5; // Semicolon → evaluated and discarded
}
```

---

# Behavior and Semantics

| Aspect            | Behavior                                           |
| :---------------- | :------------------------------------------------- |
| Evaluation order  | Top to bottom, left to right                       |
| Resource lifetime | Bound to enclosing block                           |
| Drop timing       | Value drops when out of scope unless `mem::forget` |

* Statements **create scopes** indirectly by introducing variables.
* **Shadowing** allowed: reusing same variable name.

Example:

```rust
let x = 5;
let x = x + 1; // Shadows old x
```

---

# Internal Compiler Details

* Rust lowers statements into **MIR (Mid-level Intermediate Representation)**.
* Statements contribute to **basic blocks** (linear chains of execution).
* **Let statements** allocate stack slots.
* Expressions that **produce unused values** still execute, unless optimized away in `release` mode.

MIR Example (conceptual):

```text
bb0: {
    _1 = 5;
    _2 = Add(_1, 1);
}
```

---

# Memory and Code Generation Effects

| Type of Statement | Memory Effect                                         |
| :---------------- | :---------------------------------------------------- |
| Let               | Allocates storage                                     |
| Expression        | Temporary storage for intermediate results            |
| Item              | Defines new symbol, memory at global/static if needed |

* Dead code removal optimizations prune unnecessary statements in release mode (`-C opt-level=3`).

---

# Visibility, Scoping, and Lifetime

| Construct              | Scope                     |
| :--------------------- | :------------------------ |
| `let` binding          | Current block and below   |
| `fn`/item inside block | Scoped to enclosing block |
| `unsafe` block         | New unsafe sub-scope      |

* **Block expressions** (`{}`) create **new scopes** for statements:

```rust
fn main() {
    {
        let x = 10;
    }
    // x is no longer available
}
```

---

# Attributes on Statements

Rust permits attributes on:

* **Item statements**
* **Let statements** (rarely, via unstable features or macros)

### On Item Statement

```rust
fn main() {
    #[inline(always)]
    fn helper() {} // attribute on a local function
}
```

### On Let Statement (via Macros or RFCs)

Direct attributes on let-statements (e.g., to inline allocation optimizations) are being explored but **currently unstable**.

```rust
// Hypothetical unstable example:
#[cfg(feature = "foo")]
let x = 5;
```

Realistic way today: encapsulate inside function or use `cfg_if!` macro.

---

# Edge Cases and Lesser-Known Features

### 1. Dangling Semicolons

Empty statement:

```rust
fn main() {
    ; // valid empty statement
}
```

Useful for macros generating zero or multiple statements safely.

---

### 2. Let Else

`let else` for pattern matching in a statement:

```rust
fn main() {
    let Some(x) = Some(10) else {
        panic!("Not Some!");
    };
    println!("{}", x);
}
```

* Available from **Rust 1.65+**.
* Replaces error-prone `match` boilerplate.

---

### 3. Statements Inside Const Contexts

Const expressions only allow certain kinds of statements:

```rust
const fn add(x: i32, y: i32) -> i32 {
    let result = x + y; // OK
    result
}
```

* Loops (`for`, `while`) inside `const fn` are unstable as of now unless using `while let`.

---

# Limitations, Gotchas, and Non-Obvious Behaviors

| Issue                | Description                                                           |
| :------------------- | :-------------------------------------------------------------------- |
| Return in Statements | `return` itself acts like a special statement, terminating evaluation |
| Break vs Continue    | Only valid in loops or labeled scopes                                 |
| Shadowing pitfalls   | Shadowing may hide bugs if not careful                                |

Example of hidden problem:

```rust
let value = Some(5);
let value = value.unwrap(); // hides the original meaning of `value`
```

---

# Comparison with Other Languages

| Language | Equivalent                          | Notes                                                    |
| :------- | :---------------------------------- | :------------------------------------------------------- |
| C        | Expression Statements, Declarations | Similar, but Rust's `let` has richer pattern matching    |
| Java     | Statements and Expressions          | Java expressions can't be standalone without context     |
| Go       | Statements, Declarations            | Go lacks expression-based blocks like Rust has           |
| Python   | Everything is a statement           | Python separates expressions and statements less cleanly |

---

# Trade-off Comparison Table

| Type                 | Pros                                 | Cons                                  |
| :------------------- | :----------------------------------- | :------------------------------------ |
| `let` Statement      | Clear, structured variable ownership | Repeated shadowing can confuse        |
| Expression Statement | Side-effect execution                | Silent value discarding may hide bugs |
| Item Statement       | Locally scoped items possible        | Uncommon, may confuse readers         |
| Let-Else Statement   | Clean pattern error handling         | Needs Rust 1.65+                      |

---

# Conclusion

* Statements in Rust **control flow**, **manage memory**, and **structure program logic**.
* The boundary between **expressions** and **statements** is critical for understanding **ownership**, **lifetimes**, and **performance**.
* Correct usage of `let`, expression statements, and proper scoping dramatically impacts **readability** and **efficiency**.

---
