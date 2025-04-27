**Lifetime Annotations in Rust: Complete Reference**

---

### **1. Basic Syntax**
- Annotate lifetimes with `'` followed by a lowercase name (e.g., `'a`).
- Used in function signatures, structs, enums, traits, and `impl` blocks.

```rust
// Function with explicit lifetime
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Struct with lifetime
struct Wrapper<'a> {
    data: &'a str,
}
```

---

### **2. Lifetime Elision Rules**
Rust automatically infers lifetimes in common cases:
1. **Rule 1**: Each parameter gets its own lifetime.
   ```rust
   fn elided(s: &str) -> &str { ... } // Equivalent to fn<'a>(s: &'a str) -> &'a str
   ```
2. **Rule 2**: If there’s exactly one input lifetime, it’s assigned to all output lifetimes.
   ```rust
   fn first_word(s: &str) -> &str { ... } // Inferred: s and return share the same lifetime
   ```
3. **Rule 3**: For methods with `&self` or `&mut self`, the `self` lifetime is assigned to all outputs.
   ```rust
   impl<'a> Wrapper<'a> {
       fn get_data(&self) -> &str { self.data } // Inferred: return tied to struct's lifetime
   }
   ```

---

### **3. Structs and Lifetimes**
- Structs holding references must declare lifetimes to ensure the struct doesn’t outlive the data it references.
```rust
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Book<'a> {
    fn new(title: &'a str, author: &'a str) -> Self {
        Book { title, author }
    }
}
```

---

### **4. Lifetime Bounds**
- Use `where` clauses or inline bounds to enforce relationships between lifetimes.
```rust
// Using a where clause
fn compare<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'a: 'b, // 'a must outlive 'b
{
    x
}

// Inline bound
fn compare_inline<'a: 'b, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
```

---

### **5. The `'static` Lifetime**
- Indicates data lives for the entire program duration (e.g., string literals).
```rust
let static_str: &'static str = "Hello, world!";
```

---

### **6. Common Errors and Fixes**
| **Error** | **Cause** | **Fix** |
|-----------|-----------|---------|
| `missing lifetime specifier` | Lifetime omitted where required | Add explicit lifetime annotations. |
| `lifetime mismatch` | Output lifetime not tied to inputs | Ensure output references derive from input parameters. |
| `struct outlives reference` | Struct lifetime longer than its data | Annotate the struct with the correct lifetime. |

---

### **7. Advanced Patterns**
- **Higher-Ranked Trait Bounds (HRTB)**: For closures/functions accepting any lifetime.
  ```rust
  fn apply<F>(f: F) where F: for<'a> Fn(&'a str) -> &'a str {
      // ...
  }
  ```
- **Lifetime Subtyping**: Enforce one lifetime outliving another.
  ```rust
  fn process<'long: 'short, 'short>(long: &'long str, short: &'short str) { ... }
  ```

---

### **8. Traits and Lifetimes**
- Traits can have lifetime parameters.
```rust
trait Processor<'a> {
    fn process(&self, data: &'a str) -> &'a str;
}

impl<'a> Processor<'a> for Book<'a> {
    fn process(&self, data: &'a str) -> &'a str {
        self.title
    }
}
```

---

### **9. Testing Lifetime Validity**
- Use the compiler’s borrow checker to verify references.
- Example of invalid code:
  ```rust
  let result;
  {
      let s = String::from("temp");
      result = &s; // Error: `s` does not live long enough
  }
  println!("{}", result);
  ```

---

### **10. Key Tips**
- Use explicit lifetimes only when elision rules fail.
- Prefer tying output lifetimes to inputs to avoid dangling references.
- Leverage `'static` for global data, but avoid overusing (e.g., for heap-allocated data).