

### **1. Defining Traits**
- **Basic Syntax**:  
  ```rust
  trait Greeting {
      fn greet(&self) -> String;
  }
  ```

- **Methods with Parameters**:  
  ```rust
  trait Math {
      fn add(&self, other: i32) -> i32;
  }
  ```

- **Associated Constants**:  
  ```rust
  trait Version {
      const ID: u32;
  }

  struct MyStruct;
  impl Version for MyStruct {
      const ID: u32 = 1;
  }
  ```

---

### **2. Implementing Traits**
- **Basic Implementation**:  
  ```rust
  struct Person { name: String }
  impl Greeting for Person {
      fn greet(&self) -> String {
          format!("Hello, {}!", self.name)
      }
  }
  ```

- **Multiple Traits**:  
  ```rust
  impl TraitA for MyType {}
  impl TraitB for MyType {}
  ```

---

### **3. Default Methods**
- **Default Implementation**:  
  ```rust
  trait Logger {
      fn log(&self, msg: &str) {
          println!("[LOG] {}", msg);
      }
  }
  ```

- **Override Default Method**:  
  ```rust
  struct CustomLogger;
  impl Logger for CustomLogger {
      fn log(&self, msg: &str) {
          eprintln!("[ERROR] {}", msg);
      }
  }
  ```

---

### **4. Derivable Traits**
- **Automatically Implemented Traits**:  
  Supported traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, etc.  
  ```rust
  #[derive(Debug, Clone)]
  struct Point { x: i32, y: i32 }
  ```

- **Custom Derive (Nightly)**:  
  Requires `#[feature(custom_derive)]` for non-standard traits.

---

### **5. Trait Bounds (Generics)**
- **Generic Functions**:  
  ```rust
  fn print_greeting<T: Greeting>(entity: &T) {
      println!("{}", entity.greet());
  }
  ```

- **Multiple Bounds**:  
  ```rust
  fn process<T: Clone + Debug>(value: T) { ... }
  ```

- **Where Clauses**:  
  ```rust
  fn complex_function<T, U>(t: T, u: U)
  where
      T: Iterator<Item = U>,
      U: Clone,
  { ... }
  ```

---

### **6. Associated Types**
- **Define Placeholder Types**:  
  ```rust
  trait Iterator {
      type Item;
      fn next(&mut self) -> Option<Self::Item>;
  }

  struct Counter;
  impl Iterator for Counter {
      type Item = u32;
      fn next(&mut self) -> Option<u32> { Some(1) }
  }
  ```

---

### **7. Supertraits**
- **Require Base Traits**:  
  ```rust
  trait Shape: Draw {
      fn area(&self) -> f64;
  }
  ```

- **Implementation**:  
  Any type implementing `Shape` must also implement `Draw`.

---

### **8. Trait Objects (Dynamic Dispatch)**
- **Box<dyn Trait>**:  
  ```rust
  let shapes: Vec<Box<dyn Shape>> = vec![
      Box::new(Circle { radius: 2.0 }),
      Box::new(Square { side: 3.0 }),
  ];
  ```

- **Object Safety Rules**:  
  - No generic methods.
  - No methods returning `Self`.
  - No static methods (unless using `#[receiver_for_trait]` in nightly).

---

### **9. Orphan Rule**
- **Restrictions**:  
  You can implement a trait for a type **only if**:
  - The trait is defined in your crate, **or**
  - The type is defined in your crate.  
  Invalid:  
  ```rust
  // Error: Both `ToString` and `Vec<T>` are from std.
  impl ToString for Vec<i32> { ... }
  ```

---

### **10. Operator Overloading**
- **Overload Operators via Traits**:  
  ```rust
  use std::ops::Add;

  #[derive(Debug)]
  struct Point { x: i32, y: i32 }

  impl Add for Point {
      type Output = Self;
      fn add(self, other: Self) -> Self {
          Point { x: self.x + other.x, y: self.y + other.y }
      }
  }
  ```

---

### **11. Blanket Implementations**
- **Implement for All Types**:  
  ```rust
  trait Printable {
      fn print(&self);
  }

  impl<T: Debug> Printable for T {
      fn print(&self) {
          println!("{:?}", self);
      }
  }
  ```

---

### **12. Auto Traits (Send, Sync)**
- **Marker Traits**:  
  Automatically implemented by compiler. Cannot implement manually.  
  ```rust
  fn is_send<T: Send>() {} // Ensure T is thread-safe
  ```

- **Negative Impls**:  
  Not directly supported. Use `!Send`/`!Sync` in trait bounds.

---

### **13. Specialization (Nightly Only)**
- **Override Generic Impl**:  
  ```rust
  #![feature(specialization)]

  trait Foo { fn bar(); }
  impl<T> Foo for T { fn bar() { println!("Generic"); } }
  impl Foo for i32 { fn bar() { println!("Specialized"); } }
  ```

---

### **14. Common Traits**
| Trait | Purpose |
|-------|---------|
| `Debug` | Formatting for debugging (`{:?}`). |
| `Clone` | Deep copy (`clone()`). |
| `Copy` | Shallow bitwise copy. |
| `PartialEq` | Equality comparison (`==`). |
| `Ord` | Total ordering. |
| `Default` | Default value (`Default::default()`). |

---

### **15. Deref and Drop Traits**
- **Deref Coercion**:  
  ```rust
  use std::ops::Deref;

  struct MyBox<T>(T);
  impl<T> Deref for MyBox<T> {
      type Target = T;
      fn deref(&self) -> &Self::Target { &self.0 }
  }
  ```

- **Drop Trait**:  
  ```rust
  struct CustomDrop;
  impl Drop for CustomDrop {
      fn drop(&mut self) { println!("Dropping!"); }
  }
  ```

---

### **16. Function Traits (Closures/Fn)**
| Trait | Signature |
|-------|-----------|
| `FnOnce` | `fn call_once(self, args)` |
| `FnMut` | `fn call_mut(&mut self, args)` |
| `Fn` | `fn call(&self, args)` |

Example:  
```rust
fn apply<F>(f: F) where F: FnOnce() { f(); }
```

---

### **17. Higher-Ranked Trait Bounds (HRTB)**
- **Lifetime Parameters**:  
  ```rust
  fn process<F>(f: F)
  where
      F: for<'a> Fn(&'a str) -> String,
  { ... }
  ```

---

### **18. Negative Traits**
- **Mark Unwanted Behavior**:  
  Currently unstable. Used internally for `!Send`/`!Sync`.

---

### **19. Edge Cases & Gotchas**

#### **Conflicting Implementations**
```rust
// Error: Overlapping impls
impl<T> MyTrait for T { ... }
impl MyTrait for i32 { ... } // Conflicts with generic impl
```

#### **Object Safety Violation**
```rust
trait BadTrait {
    fn new() -> Self; // Not object-safe
}
```

#### **Bounded Wildcards**
```rust
// Accepts any type that implements TraitA and TraitB
fn foo<T>(x: T) where T: TraitA + TraitB { ... }
```

#### **?Sized and Unsized Types**
```rust
trait MyTrait {}
impl MyTrait for dyn Any + Send {} // Allow unsized types
```

#### **Self in Traits**
```rust
trait Factory {
    fn create() -> Self; // Valid
    fn compare(a: Self, b: Self); // Valid
}
```

---

### **20. Advanced Code Examples**

#### **Trait with Generic Method**
```rust
trait Container {
    fn contains(&self, item: &dyn Any) -> bool;
}
```

#### **Trait Object with Lifetime**
```rust
fn returns_trait() -> Box<dyn Debug + 'static> {
    Box::new(42)
}
```

#### **Trait with Const Generics**
```rust
trait ArraySize { const SIZE: usize; }

struct MyArray<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> MyArray<T, N>
where
    T: Default,
{
    fn new() -> Self {
        Self { data: [T::default(); N] }
    }
}
```

#### **Ambiguous Method Resolution**
```rust
trait A { fn foo() { println!("A"); } }
trait B { fn foo() { println!("B"); } }

struct S;
impl A for S {}
impl B for S {}

// ERROR: Ambiguous
// S::foo();

// Disambiguate with fully qualified syntax:
<A as A>::foo();
<B as B>::foo();
```
