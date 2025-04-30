

---

#### **1. Trait Basics**
**Definition**  
```rust
trait Named {
    // Required method
    fn name(&self) -> String;
    
    // Default implementation
    fn shout_name(&self) {
        println!("{}!!!", self.name().to_uppercase());
    }
}
```

**Implementation**  
```rust
struct Person(String);

impl Named for Person {
    fn name(&self) -> String {
        self.0.clone()
    }
    // `shout_name` uses default implementation
}
```

**Edge Cases**  
- Default methods can be overridden in `impl` blocks.
- Traits cannot define fields (use associated types or struct composition).

---

#### **2. Trait Parameters and Associated Items**
**Method Receivers**  
```rust
trait Consumable {
    fn consume(self); // Takes ownership
    fn inspect(&self); // Immutable borrow
    fn mutate(&mut self); // Mutable borrow
}
```

**Associated Functions/Constants**  
```rust
trait Factory {
    const DEFAULT_ID: u32 = 0;
    
    fn new(id: u32) -> Self;
}

struct Widget;
impl Factory for Widget {
    const DEFAULT_ID: u32 = 100;
    
    fn new(id: u32) -> Self {
        Widget
    }
}
```

---

#### **3. Trait Bounds and Generics**
**Generic Functions**  
```rust
fn print_name<T: Named>(item: &T) {
    println!("{}", item.name());
}

// Alternative `where` syntax
fn process<T>(item: T)
where
    T: Named + Clone,
{
    // ...
}
```

**`impl Trait` Syntax**  
```rust
fn create_named() -> impl Named {
    Person("Alice".to_string())
}
```

**Edge Cases**  
- `impl Trait` in argument position is only allowed in specific cases (e.g., `fn f(item: impl Named)`).

---

#### **4. Trait Objects and Dynamic Dispatch**
**Trait Objects**  
```rust
let items: Vec<&dyn Named> = vec![&Person("Bob".to_string())];
for item in items {
    println!("{}", item.name());
}
```

**Object Safety Rules**  
A trait is object-safe if:
- No associated constants.
- No generic methods.
- Methods don’t return `Self`.
- Methods without `Self: Sized` bounds.

**Violation Example**  
```rust
trait NotObjectSafe {
    fn new() -> Self; // Returns Self, not object-safe
}
```

---

#### **5. Advanced Concepts**
**Supertraits**  
```rust
trait Super: std::fmt::Debug {
    fn super_method(&self);
}
```

**Marker Traits**  
```rust
unsafe trait SyncMarker {} // No methods, just a marker
```

**Associated Types vs Generics**  
```rust
trait Iterator {
    type Item; // Associated type
    fn next(&mut self) -> Option<Self::Item>;
}

trait GenericTrait<T> {
    fn process(&self, item: T); // Generic parameter
}
```

**Blanket Implementations**  
```rust
impl<T: std::fmt::Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}
```

**Coherence & Orphan Rules**  
- Cannot implement foreign traits on foreign types.  
**Workaround (Newtype Pattern)**  
```rust
struct MyVec<T>(Vec<T>);
impl<T> MyTrait for MyVec<T> { /* ... */ }
```

**Conditional Implementations**  
```rust
impl<T: PartialEq> PartialEq for Pair<T> {
    // Compare `a` and `b` fields
}
```

**Method Name Disambiguation**  
```rust
<MyType as TraitA>::method();
<MyType as TraitB>::method();
```

---

#### **6. Derivable and Sealed Traits**
**Derivable Traits**  
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

**Sealed Traits**  
```rust
mod sealed {
    pub trait Sealed {}
}
trait MyTrait: sealed::Sealed {}
// Only crate-local types can implement `Sealed`.
```

---

#### **7. Edge Cases and Tricky Parts**
**Orphan Rule Conflict**  
```rust
// ERROR: Can't implement `Display` for `Vec<T>` (both foreign)
impl<T> std::fmt::Display for Vec<T> { /* ... */ }
```

**Object Safety Violation**  
```rust
trait BadTrait {
    const CONST: usize; // Associated constants aren't object-safe
}
```

**Overlapping Implementations**  
- Rust’s coherence rules prevent ambiguous `impl` blocks.  
- Use `#[fundamental]` for opt-in relaxation (advanced).

**Bypassing Orphan Rules with Newtype**  
```rust
struct MyType<T>(T);
impl<T> ForeignTrait for MyType<T> { /* ... */ }
```

**Self: Sized in Traits**  
```rust
trait SafeTrait {
    fn safe_method(&self) where Self: Sized;
}
// `safe_method` is excluded from trait objects.
```