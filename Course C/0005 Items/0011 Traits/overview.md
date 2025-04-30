### **1. Declaring & Implementing Traits**

```rust
// basic syntax
pub trait Summarize {
    fn headline(&self) -> String;                 // required method
    fn summary(&self) -> String {                 // default method
        format!("(read more: {})", self.headline())
    }
}

// implement for a concrete type
impl Summarize for i32 {
    fn headline(&self) -> String { format!("Number: {self}") }
}
```

* `trait` names live in the type namespace – you can import them with `use`. 
> **[Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html)** · *doc.rust-lang.org*
  
** Default bodies are inherited unless the `impl` overrides them. 
> **[Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html)** · *Rust Documentation*
  

---

### **2. Trait Bounds & Generics**

| Syntax | Example | Notes |
|--------|---------|-------|
| Inline bound | `fn print<T: Summarize>(x: T)` | Multiple bounds with `+` |
| `where`-clause | <br>`fn log<T, U>(t: T, u: U)`<br>`where`<br>`    T: Summarize + Clone,`<br>`    U: Display,` | Reads better for long bounds |
| Higher-Ranked | `for<'a> F: Fn(&'a str)` | required for “any lifetime” cases 
> **[Higher-Rank Trait Bounds - The Rustonomicon](https://doc.rust-lang.org/nomicon/hrtb.html?utm_source=chatgpt.com)** · *Rust Documentation*
> Intense. There aren't many places outside of the Fn traits where we encounter HRTBs, and even for those we have a nice magic sugar for the common cases.
 |

Edge-case: to bound a lifetime on the *return* value you need HRTBs:

```rust
fn apply<F>(f: F) -> i32
where
    for<'a> F: Fn(&'a i32) -> &'a i32
{
    let x = 0;
    *f(&x)
}
```

---

### **3. Associated Items**

#### Associated **types**

```rust
trait Graph {
    type Node;                // associated type
    fn neighbors(&self, n: &Self::Node) -> Vec<Self::Node>;
}
```

*Avoid type-parameter explosion and allow ergonomic references like `Self::Node`.*

#### **Generic Associated Types (GATs)** — stable since 1.65

```rust
trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a;             // lifetime may vary per use-site

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
```

> **[Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html?utm_source=chatgpt.com)** · *Rust Documentation*
> Traits items that begin with the unsafe keyword indicate that implementing the trait may be unsafe. It is safe to use a correctly implemented unsafe trait. The ...


#### Associated **constants**

```rust
trait Real {
    const ZERO: Self;
}
```

---

### **4. Super-Traits & Inheritance**

```rust
trait DisplayArea { fn area(&self) -> f64; }

trait Circle: DisplayArea {
    fn radius(&self) -> f64 {
        (self.area() / std::f64::consts::PI).sqrt()
    }
}
```
Super-traits give automatic access to their methods on any type that implements the sub-trait. 
> **[Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html)** · *Rust Documentation*


---

### **5. Blanket Implementations**

```rust
// std: every T that implements Display automatically gets ToString
impl<T: Display + ?Sized> ToString for T {}
```
This “cover-all” pattern reduces boilerplate and is used throughout the standard library (e.g., `From<T>` ⇒ automatic `Into`). 
> **[Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html?utm_source=chatgpt.com)** · *Rust Documentation*
> Blanket implementations appear in the documentation for the trait in the “Implementors” section. Traits and trait bounds let us write code that uses generic ...


---

### **6. Auto / Marker Traits**

* Built-in: **Send**, **Sync**, **Unpin**, **Freeze**, **UnwindSafe**.  
* Custom marker traits are simply empty traits (often `unsafe` when they promise extra invariants).  
* **Negative impls** forbid a type from implementing an auto trait:

```rust
unsafe impl !Send for RcCell { /* … */ }   // stable
```

---

### **7. Operator Overloading**

Implement the relevant trait from `std::ops`:

```rust
use std::ops::{Add, Deref};

impl Add for Vec<i32> {
    type Output = Vec<i32>;
    fn add(self, rhs: Self) -> Self::Output { self.into_iter().chain(rhs).collect() }
}
```

Each operator has an associated trait (`AddAssign`, `Mul`, `Index`, etc.).

---

### **8. Trait Objects & Dynamic Dispatch**

*Syntax* — `&dyn Trait`, `Box<dyn Trait + Send + 'static>`, `Arc<dyn Trait>`  
A trait object is a fat pointer: **(data-ptr, vtable-ptr)**. 
> **[Trait object types - The Rust Reference](https://doc.rust-lang.org/reference/types/trait-object.html)** · *Rust Documentation*
  

```rust
fn log(item: &dyn Summarize) {          // late binding
    println!("{}", item.summary());
}

let n: Box<dyn Summarize> = Box::new(42);
log(&*n);
```

#### 8.1 Dyn-Compatibility (formerly “object-safety”)

| Rule (summarised) | Why |
|-------------------|-----|
| `Sized` **must not** be a super-trait | dyn types are unsized |
| No associated **consts** | no space in vtable ✔ |
| No **generic associated types** | vtable cannot encode generics |
| Methods callable on a trait object must: <br>• have **no type parameters**<br>• take `&Self`, `&mut Self`, `Box<Self>`, `Rc<Self>`, `Arc<Self>` or `Pin` thereof<br>• not return `Self` or `impl Trait`<br>• not be `async fn` | keeps a fixed, known signature | 
> **[Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html)** · *Rust Documentation*
 |

If a trait violates any rule you’ll get **E0038** at compile time. 
> **[E0038 - Error codes index](https://doc.rust-lang.org/error_codes/E0038.html?utm_source=chatgpt.com)** · *Rust Documentation*
> Some traits are not allowed to be used as trait object types. The traits that are allowed to be used as trait object types are called "dyn-compatible" traits.
  

#### 8.2 Trait Upcasting (stable 1.78)

```rust
trait A { fn a(&self); }
trait B: A { fn b(&self); }

let obj: Box<dyn B> = Box::new(...);
let up: Box<dyn A> = obj;      // automatic upcast
```  
Upcasting to add auto-traits is also allowed (`dyn Trait` → `dyn Trait + Send`). 
> **[Rust Release Notes](https://doc.rust-lang.org/beta/releases.html)** · *Rust Documentation*
  

---

### **9. `impl Trait` (Opaque Types)**

* **Return-position** — hide a concrete type:  
  ```rust
  fn make_vec() -> impl Iterator<Item = i32> { 0..10 }
  ```
* **Argument-position** — shorthand for a generic parameter:  
  ```rust
  fn print_all(items: impl IntoIterator<Item = i32>) { … }
  ```

Opaque types are *statically* dispatched, unlike `dyn Trait`. 
> **[Impl trait type - The Rust Reference](https://doc.rust-lang.org/reference/types/impl-trait.html?utm_source=chatgpt.com)** · *Rust Documentation*
> impl Trait provides ways to specify unnamed but concrete types that implement a specific trait. It can appear in two sorts of places.
  

---

### **10. Specialization (Nightly)**

* `#![feature(min_specialization)]` – safe subset now available.  
* Full specialization remains unstable; emulate with new-type wrappers or sealed-trait pattern when writing libraries.

---

### **11. Trait Aliases (Nightly)**

```rust
#![feature(trait_alias)]
trait Io = Read + Write + Send + Unpin;
```
Aliases group bounds for reuse but are not new traits. 
> **[specialization - The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/language-features/specialization.html?utm_source=chatgpt.com)** · *Rust Documentation*
> min_specialization · 2.136. mips_target_feature · 2.137. more_maybe_bounds · 2.138. more_qualified_paths · 2.139. multiple_supertrait_upcastable · 2.140 ...
  

---

### **12. Comparison with Other Languages**

| Concept | Rust **trait** | Java **interface** | Haskell **type-class** | Swift **protocol** |
|---------|---------------|--------------------|------------------------|--------------------|
| Default method bodies | ✔ | ✔ (since 8) | ✖ | ✔ |
| Associated types | ✔ (`type`) | ✖ | ✔ | ✔ (`associatedtype`) |
| Multiple impl per type | *One* impl per pair (coherence rule) | Unlimited | Unlimited | Unlimited |
| Compile-time generics | ✔ (monomorphization) | Limited (type erasure) | ✔ | ✔ |
| Dynamic dispatch | `dyn Trait` opt-in | default | ✖ (needs existential wrapper) | default |

---

### **13. Coherence & The Orphan Rule**

*You may implement either:*

1. **Your trait** for **any type**  
2. **Any trait** for **your type**  

…but **not** a foreign trait for a foreign type. This prevents conflicting blanket impls. 
> **[Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html?utm_source=chatgpt.com)** · *Rust Documentation*
> This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ...
  

---

### **14. Advanced Patterns & Tips**

* **Sealed-trait** pattern prevents downstream impls:  
  ```rust
  mod sealed { pub trait Sealed {} }
  pub trait MyTrait: sealed::Sealed { … }
  ```
* **Extension traits** add methods to external types without new-type boilerplate.  
* **Builder DSLs**: return `Self` or `impl Trait` to chain calls fluidly.  
* Trait bounds on **const generics** are now fully supported.  
* Use **auto traits** + **negative impls** to model invariants (e.g., `!Send` RC cells).  
* With **async functions in traits** still unstable, prefer the `async_trait` crate or return a boxed future.

---

### **15. Cheat-Sheet**

```rust
// method disambiguation
<T as Trait>::method(&value);

// default-bound receiver trick to make a method object-safe
fn size(&self) -> usize where Self: Sized { ... }   // not callable on dyn

// blanket impl excluding one type
trait Pretty { fn pretty(&self) -> String; }
impl<T> Pretty for T where T: Display + ?Sized { ... }
impl Pretty for str { fn pretty(&self) -> String { self.into() } } // overrides default

// negative auto trait
struct !Send NonSendType(*mut u8);
```

---

**Stable vs. Nightly Quick Reference**

| Feature | Channel |
|---------|---------|
| Traits, bounds, associated types/consts | **Stable** |
| Generic Associated Types (GATs) | **Stable ≥ 1.65** |
| Trait upcasting coercions | **Stable ≥ 1.78** |
| Async fn in traits (without external crate) | **Nightly** |
| Specialization / `min_specialization` | **Nightly** |
| Trait aliases | **Nightly** |
| Negative *bounds* (`T: !Copy`) | **Nightly** (negative **impls** are stable) |

---
