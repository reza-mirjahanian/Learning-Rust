### **Basic Syntax**  
- **Single Item Import**  
  ```rust
  use std::collections::HashMap;
  fn main() {
      let map = HashMap::new();
  }
  ```

- **Multiple Items (Braces)**  
  ```rust
  use std::{collections::HashMap, io::Result};
  fn main() -> Result<()> {
      let map = HashMap::new();
      Ok(())
  }
  ```

- **Absolute vs. Relative Paths**  
  ```rust
  // Absolute path (from crate root)
  use crate::my_mod::MyType;

  // Relative path (from current module)
  use super::parent_mod::MyType;
  use self::sub_mod::MyType;
  ```

---

### **Renaming with `as`**  
- **Resolve Name Conflicts**  
  ```rust
  use std::io::Result as IoResult;
  use crate::io::Result as MyResult;

  fn main() -> IoResult<()> {
      let _ = MyResult::Ok(());
      Ok(())
  }
  ```

- **Avoid Shadowing**  
  ```rust
  use crate::utils::Logger as AppLogger;
  fn main() {
      let logger = AppLogger::new();
  }
  ```

---

### **Glob Imports (`*`)**  
- **Import All Public Symbols**  
  ```rust
  use std::collections::*;
  fn main() {
      let map = HashMap::new();  // HashMap is in scope
      let set = HashSet::new();  // HashSet is also in scope
  }
  ```

- **Edge Case: Name Collisions**  
  ```rust
  // Conflict if both define `Foo`
  use crate::mod1::*;
  use crate::mod2::*;
  // Error: `Foo` is ambiguous
  ```

---

### **Nested Paths**  
- **Combine Common Prefixes**  
  ```rust
  use std::collections::{HashMap, HashSet};
  fn main() {
      let map = HashMap::new();
      let set = HashSet::new();
  }
  ```

- **Deep Nesting**  
  ```rust
  use crate::utils::{self, parser::{Json, Xml}};
  // Imports: `utils`, `utils::parser::Json`, `utils::parser::Xml`
  ```

---

### **`pub use` for Re-exporting**  
- **Expose Symbols in Public API**  
  ```rust
  // lib.rs
  pub use self::details::PublicType;

  mod details {
      pub struct PublicType;
  }
  ```

- **Edge Case: Private Re-export**  
  ```rust
  // Not allowed: `PrivateType` is private
  pub use self::details::PrivateType;
  ```

---

### **Scoping Rules**  
- **Function-Level Scope**  
  ```rust
  fn process() {
          use std::io::Result;
          // Only visible inside `process`
      }
  ```

- **Module-Level Scope**  
  ```rust
  // Visible in entire module
  use std::io::Result;

  mod sub {
          // Must re-import or use `pub use`
      }
  ```

---

### **Shadowing**  
- **Overriding Imports**  
  ```rust
  use crate::utils::Logger;  // Shadowed below
  fn main() {
          use crate::debug::Logger;
          let logger = Logger::new();  // Uses `debug::Logger`
      }
  ```

- **Prevent with `as`**  
  ```rust
  use crate::utils::Logger as UtilsLogger;
  use crate::debug::Logger as DebugLogger;
  ```

---

### **Importing Macros**  
- **Require `#[macro_use]` Attribute**  
  ```rust
  #[macro_use]
  extern crate my_macros;

  fn main() {
      my_macro!();  // Now visible
  }
  ```

- **Scoped Macro Import (Rust 2018+)**  
  ```rust
  use my_macros::my_macro;
  fn main() {
      my_macro!();  // Works in Rust 2018+
  }
  ```

---

### **Edge Cases**  
- **Name Collision Between Imports**  
  ```rust
  use crate::mod1::Foo;
  use crate::mod2::Foo;  // Error: `Foo` is ambiguous
  ```

- **Empty Use Declaration**  
  ```rust
  use std::collections::{};  // Valid but does nothing
  ```

- **Importing `self`**  
  ```rust
  use crate::my_mod::{self, SubMod};  // Imports `my_mod` and `my_mod::SubMod`
  ```

---

### **Comparisons with Other Languages**  
| Feature              | Rust (`use`)              | Python (`import`)         | JavaScript (`import`)     |
|----------------------|---------------------------|----------------------------|----------------------------|
| **Renaming**         | `use A as B;`             | `import A as B`           | `import {A as B} from ""` |
| **Glob Import**      | `use *;`                  | `from X import *`         | `import * as X from ""`   |
| **Scoped Import**    | `use` inside block        | Valid                     | Valid                     |
| **Re-export**        | `pub use`                 | `__all__` (convention)    | `export {A} from ""`      |

---

### **Best Practices**  
- **Avoid Glob Imports** in large projects to prevent ambiguity.  
- **Prefer Explicit Paths**: `use std::collections::HashMap` instead of glob.  
- **Use `as` for Clarity**: Rename common types (e.g., `Result as IoResult`).  
- **Group Nested Imports**: Reduce redundancy with `{A, B}` syntax.  
- **Re-export Public APIs**: Use `pub use` to simplify external imports.