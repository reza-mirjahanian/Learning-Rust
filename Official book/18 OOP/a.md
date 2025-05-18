# Object-Oriented Programming in Rust (OOP in Rust)

Rust is not an object-oriented language in the traditional sense (like Java or C++), but it supports **many object-oriented programming (OOP) principles**:

* **Encapsulation**
* **Abstraction**
* **Inheritance** (via trait composition)
* **Polymorphism**

Let’s go from basics to expert level.

---

## 🧱 Level 1: Foundational Concepts

### 1. **Structs** (Data encapsulation)

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}
```

* Like classes without methods.
* Encapsulate related data.

### 2. **Impl Blocks** (Methods on structs)

```rust
impl User {
    fn is_active(&self) -> bool {
        self.active
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}
```

### 3. **Associated Functions (Static methods)**

```rust
impl User {
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: true,
        }
    }
}
```

---

## 🧰 Level 2: Traits (Interfaces & Abstraction)

### 1. **Trait Definition**

```rust
trait Drawable {
    fn draw(&self);
}
```

### 2. **Trait Implementation**

```rust
struct Circle;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}
```

### 3. **Trait Bound on Function**

```rust
fn render(object: &impl Drawable) {
    object.draw();
}
```

Or:

```rust
fn render<T: Drawable>(object: &T) {
    object.draw();
}
```

---

## 🔁 Level 3: Dynamic Dispatch (Polymorphism)

### 1. **Trait Objects with `dyn`**

```rust
fn render_all(objects: &[Box<dyn Drawable>]) {
    for object in objects {
        object.draw();
    }
}
```

* Enables **runtime polymorphism**
* `Box<dyn Trait>`: trait object

### 2. **Example**

```rust
struct Square;

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];

    render_all(&shapes);
}
```

---

## 🧬 Level 4: Inheritance via Trait Composition

Rust doesn’t support class-based inheritance. Instead, it uses **trait composition**.

### 1. **Base Trait + Supertrait**

```rust
trait Shape {
    fn area(&self) -> f64;
}

trait Colored: Shape {
    fn color(&self) -> String;
}
```

### 2. **Implement Both**

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Colored for Rectangle {
    fn color(&self) -> String {
        "blue".into()
    }
}
```

---

## ⚡ Level 5: Encapsulation & Visibility

Rust provides fine-grained visibility controls:

| Keyword      | Meaning                       |
| ------------ | ----------------------------- |
| `pub`        | Public                        |
| `pub(crate)` | Visible only in current crate |
| `pub(super)` | Visible to parent module      |
| *default*    | Private                       |

### Example

```rust
pub struct Account {
    balance: f64,
}

impl Account {
    pub fn new() -> Self {
        Self { balance: 0.0 }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }
}
```

Here `balance` is **encapsulated**, only accessible via methods.

---

## 🧠 Level 6: Object Safety

Not all traits can be turned into trait objects (`dyn Trait`).

### A trait is **object-safe** if:

1. All methods have a receiver (`self`, `&self`, or `&mut self`)
2. No generic methods

### ❌ Not Object-Safe

```rust
trait NotSafe {
    fn generic<T>(&self, value: T);
}
```

### ✅ Object-Safe

```rust
trait Safe {
    fn execute(&self);
}
```

---

## 🧰 Level 7: Trait-Based Design Patterns

### 1. **Strategy Pattern**

```rust
trait Compression {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
}

struct Zip;
struct Rar;

impl Compression for Zip {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        println!("Compressing with ZIP");
        data.to_vec()
    }
}

impl Compression for Rar {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        println!("Compressing with RAR");
        data.to_vec()
    }
}

fn compress_file(data: &[u8], algo: &dyn Compression) {
    let _ = algo.compress(data);
}
```

---

## 📚 Level 8: Comparison with OOP in Other Languages

| Feature             | Rust                              | Java/C++                     |
| ------------------- | --------------------------------- | ---------------------------- |
| Inheritance         | Trait composition                 | Class inheritance            |
| Interfaces          | Traits                            | Interfaces/Abstract classes  |
| Method Overloading  | ❌ Not supported                   | ✅ Supported                  |
| Constructors        | `new()` method convention         | `constructor()` keyword      |
| Polymorphism        | Dynamic dispatch with `dyn Trait` | Virtual methods / interfaces |
| Visibility          | Fine-grained module visibility    | `private`, `protected`, etc. |
| Abstract Base Class | Traits with default methods       | Abstract base classes        |

---

## 🧪 Level 9: Edge Cases and Advanced Use

### 1. **Default Trait Implementations**

```rust
trait Logger {
    fn log(&self, message: &str) {
        println!("Log: {}", message);
    }
}
```

### 2. **Overriding Defaults**

```rust
struct FileLogger;

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        println!("Writing to file: {}", message);
    }
}
```

### 3. **Returning Trait Objects**

```rust
fn get_logger(file: bool) -> Box<dyn Logger> {
    if file {
        Box::new(FileLogger)
    } else {
        Box::new(ConsoleLogger)
    }
}
```

### 4. **Multiple Trait Bounds**

```rust
fn process<T: Drawable + Clone>(item: T) {
    item.draw();
}
```

---

## 🧩 Level 10: Composition Over Inheritance

Instead of class hierarchies, Rust uses **composition**.

```rust
struct Engine;
struct Wheels;

struct Car {
    engine: Engine,
    wheels: Wheels,
}
```

Each field can implement its own trait logic, which you **compose** in the higher-level struct.

---


