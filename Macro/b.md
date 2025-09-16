**Rust Macros: Complete Reference**

---

### **1. Macro Types in Rust**

Rust has two main macro systems:

- **Declarative Macros** (`macro_rules!`) — pattern-matching, compile-time code generation.
- **Procedural Macros** — custom code generators written as functions (three kinds):
  - `derive` macros
  - `attribute` macros
  - `function-like` macros

---

### **2. Declarative Macros (`macro_rules!`)**

#### **Syntax & Structure**
```rust
macro_rules! name {
    (pattern1 => expansion1;
     pattern2 => expansion2;
     ...)
}
```

#### **Basic Example**
```rust
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

say_hello!(); // Expands to: println!("Hello, world!");
```

#### **Matching Tokens and Repetition**
```rust
macro_rules! vec {
    ($($x:expr),* $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

let v = vec![1, 2, 3,]; // Trailing comma allowed thanks to $(,)?!
```

> ✅ `$($x:expr),*` → zero or more expressions separated by commas  
> ✅ `$(,)?` → optional trailing comma (supports `vec![1, 2,]`)

#### **Token Tree Matchers**
| Matcher | Meaning |
|--------|---------|
| `$x:expr` | any expression |
| `$x:stmt` | any statement |
| `$x:ty` | any type |
| `$x:ident` | any identifier |
| `$x:path` | any path (e.g., `std::vec::Vec`) |
| `$x:item` | any item (fn, struct, mod, etc.) |
| `$x:tt` | any token tree (including braces/brackets) |
| `$x:vis` | any visibility modifier (`pub`, `pub(crate)`, etc.) |

#### **Repetition Operators**
| Operator | Meaning |
|----------|---------|
| `$e:*` | zero or more |
| `$e:+` | one or more |
| `$e:?` | zero or one |

#### **Ambiguity & Precedence**
```rust
macro_rules! bad_macro {
    ($x:ident) => { println!("{}", $x); };
    ($x:literal) => { println!("{}", $x); };
}

bad_macro!(42); // ERROR: $x:ident matches `42` (it's an ident in parser sense)
```
> ❗ **Problem**: `42` is parsed as a literal *token*, but `ident` matcher also accepts it if not disambiguated.

**Fix with precedence ordering**:
```rust
macro_rules! safe_macro {
    ($x:literal) => { println!("Literal: {}", $x); }; // First — more specific
    ($x:ident) => { println!("Ident: {}", $x); };     // Second — fallback
}

safe_macro!(42);   // Literal: 42
safe_macro!(foo);  // Ident: foo
```

> 🔑 **Rule**: Place *more specific* patterns *before* general ones.

#### **Hygiene and Name Capture**
Macros are **hygienic** — names defined inside macros don’t collide with outer scope.

```rust
macro_rules! define_var {
    () => {
        let x = 5;
        println!("{}", x);
    };
}

fn main() {
    let x = 10;
    define_var!(); // prints 5 — does NOT conflict with outer x
    println!("{}", x); // prints 10
}
```

> ✅ Hygiene prevents accidental variable capture — a major safety feature.

#### **Escaping Hygiene: `$(#[$meta])*` + `paste` crate**
To break hygiene intentionally, use `paste` crate:

```toml
[dependencies]
paste = "1.0"
```

```rust
use paste::paste;

macro_rules! make_fn {
    ($name:ident) => {
        paste! {
            fn $name() {
                println!("Called {}", stringify!($name));
            }
        }
    };
}

make_fn!(my_function);
my_function(); // Prints: Called my_function
```

> 💡 Use `paste` for generating identifiers dynamically (e.g., `foo_bar`, `FooBar` from `foo-bar`).

#### **Real-World Usage: `println!`, `vec!`, `format!`**
All built-in macros are `macro_rules!`:

```rust
println!("{} {}", a, b);           // Expands to std::io::_print
vec![1, 2, 3];                     // Expands to Vec::from_iter(...)
format!("Value: {}", value);       // Uses internal formatting logic
```

#### **Edge Case: Matching Empty Sequences**
```rust
macro_rules! empty_tuple {
    () => { () };
    ($($x:expr),*) => { ($($x),*) };
}

let t1 = empty_tuple!();        // ()
let t2 = empty_tuple!(1, 2, 3); // (1, 2, 3)
```

#### **Pros and Cons of `macro_rules!`**

| Pros | Cons |
|------|------|
| ✅ Compile-time only — no runtime cost | ❌ Limited expressiveness — no Turing completeness |
| ✅ Hygienic — safe naming | ❌ Hard to debug — error messages often opaque |
| ✅ Built into compiler — no deps | ❌ No access to AST — can't inspect types |
| ✅ Excellent for DSLs (e.g., `sqlx!`, `serde_json!`) | ❌ Repetition logic verbose — hard to DRY |
| ✅ Supports complex token matching | ❌ No recursion or loops — must use repetition |

---

### **3. Procedural Macros**

Procedural macros are **functions** that run at compile time and generate Rust code. They require `proc-macro` crate type.

#### **Setup**
In `Cargo.toml`:
```toml
[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"
```

> ⚠️ Must be in a separate crate — cannot be in binary crate.

#### **Derive Macros**

##### Example: Custom `Serialize` Derive
```rust
// lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MySerialize)]
pub fn derive_my_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl MySerialize for #name {
            fn serialize(&self) -> String {
                format!("{{\"type\": \"{}\"}}", stringify!(#name))
            }
        }
    };

    TokenStream::from(expanded)
}

// Trait definition (must be defined separately)
pub trait MySerialize {
    fn serialize(&self) -> String;
}
```

Usage:
```rust
#[derive(MySerialize)]
struct Point { x: i32, y: i32 }

let p = Point { x: 1, y: 2 };
println!("{}", p.serialize()); // {"type": "Point"}
```

#### **Attribute Macros**

##### Example: `#[route("/hello")]`
```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path: String = syn::parse_macro_input!(attr as syn::LitStr).value();
    let item = syn::parse_macro_input!(item as syn::ItemFn);

    let name = &item.sig.ident;
    let expanded = quote! {
        #item

        #[no_mangle]
        pub extern "C" fn #name##_route() -> &'static str {
            #path
        }
    };

    TokenStream::from(expanded)
}
```

Usage:
```rust
#[route("/hello")]
fn hello() {
    println!("Hello");
}

// Generated: hello_route() returns "/hello"
```

#### **Function-Like Macros**

##### Example: `sqlx::query!`
```rust
#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    let sql: String = syn::parse_macro_input!(input as syn::LitStr).value();
    // Parse SQL, validate schema, generate struct at compile time
    // Returns generated code like:
    quote! {
        struct QueryResult { /* inferred fields */ }
        QueryResult { /* ... */ }
    }
    .into()
}
```

Usage:
```rust
let user = sqlx::query!("SELECT id, name FROM users WHERE id = ?", 1)
    .fetch_one(&pool)
    .await?;
// Compiler checks SQL validity at compile time!
```

#### **Comparison: Declarative vs Procedural Macros**

| Feature | `macro_rules!` | Procedural Macros |
|--------|----------------|-------------------|
| **Power** | Low — pattern matching | High — full AST manipulation |
| **Complexity** | Simple | Complex (requires `syn`, `quote`, `proc-macro2`) |
| **Type Awareness** | None — tokens only | Full type system access via `syn` |
| **Debugging** | Poor — cryptic errors | Better — you control output |
| **Performance** | Fast — simple expansion | Slower — runs as external program |
| **Use Cases** | Simple templates, DSLs | ORM, serialization, codegen, domain-specific languages |
| **Dependencies** | None | Required: `syn`, `quote`, `proc-macro2` |
| **Hygiene** | Automatic | Manual control via `quote!` |

#### **Procedural Macro Edge Cases**

##### **Handling Generics**
```rust
fn expand_with_generics(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics MyTrait for #name #ty_generics #where_clause {
            fn do_something(&self) {
                println!("Doing something with {:?}", self);
            }
        }
    };
    TokenStream::from(expanded)
}
```

##### **Error Reporting**
```rust
if !valid_sql(sql) {
    return Error::new_spanned(
        sql_token,
        "Invalid SQL syntax: expected SELECT"
    ).to_compile_error();
}
```

> ✅ Always use `syn::Error` + `.to_compile_error()` for clean compiler diagnostics.

##### **Span Preservation**
```rust
let span = input.ident.span();
let err = Error::new(span, "this field must be pub");
err.to_compile_error()
```
> Ensures compiler points to the exact source location.

#### **Real-World Projects Using Procedural Macros**

| Project | Macro Type | Purpose |
|--------|------------|---------|
| **Serde** | `derive(Serialize, Deserialize)` | Auto-generate serialization code |
| **Diesel** | `table!`, `infer_schema!` | Generate database schema structs |
| **Sqlx** | `query!`, `query_as!` | Compile-time SQL validation |
| **Axum** | `#[route]`, `#[extractor]` | Web routing and request parsing |
| **Tauri** | `#[command]` | Bind Rust functions to frontend JS |
| **Embassy** | `#[embassy::task]` | Async task generation for embedded |

---

### **4. Advanced Macro Patterns**

#### **Recursive Macro Expansion (Limited)**
```rust
macro_rules! countdown {
    (0) => { println!("Liftoff!"); };
    ($n:expr) => {
        println!("{}", $n);
        countdown!($n - 1);
    };
}

// This DOES NOT WORK — Rust macros are not recursive in the traditional sense.
// Instead, use repetition or procedural macros.
```

> ❌ `macro_rules!` cannot recurse directly. Workaround: use repetition or procedural macros.

#### **Macro Inside Macro**
```rust
macro_rules! outer {
    () => {
        macro_rules! inner {
            () => { println!("Inner!"); };
        }
        inner!();
    };
}

outer!(); // Prints "Inner!"
```

> ✅ Valid — but `inner` is only visible within `outer!` expansion scope.

#### **Conditional Compilation with Macros**
```rust
macro_rules! log {
    ($msg:expr) => {
        #[cfg(debug_assertions)]
        {
            println!("DEBUG: {}", $msg);
        }
        #[cfg(not(debug_assertions))]
        {
            // No-op in release
        }
    };
}
```

> ⚠️ Doesn’t work — `#[cfg]` must be at item level.

**Correct way**:
```rust
#[cfg(debug_assertions)]
macro_rules! log {
    ($msg:expr) => {
        println!("DEBUG: {}", $msg);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! log {
    ($msg:expr) => {};
}
```

#### **Stringification and Debug Output**
```rust
macro_rules! dbg_expr {
    ($expr:expr) => {
        {
            let val = $expr;
            println!("{} = {:?} ({}:{})", stringify!($expr), val, file!(), line!());
            val
        }
    };
}

let x = 42;
dbg_expr!(x + 1); // "x + 1 = 43 (main.rs:12)"
```

> `stringify!` — converts token to string literal  
> `file!()` — current file name  
> `line!()` — current line number  
> `column!()` — current column  

---

### **5. Comparison Tables**

#### **Macro vs Function**
| Feature | Macro | Function |
|--------|-------|----------|
| **Expansion Time** | Compile-time | Runtime |
| **Type Safety** | Partial — depends on generated code | Full |
| **Code Size** | Can bloat binary | Reusable |
| **Debugging** | Hard — expands to raw code | Easy |
| **Overhead** | Zero | Minimal |
| **Flexibility** | High — can generate arbitrary code | Fixed signature |

#### **Procedural Macro Types**
| Type | Syntax | Use Case |
|------|--------|----------|
| `derive` | `#[derive(MyTrait)]` | Auto-implement traits |
| `attribute` | `#[route("/api")]` | Annotate items with behavior |
| `function-like` | `sqlx::query!()` | DSL-style invocation |

---

### **6. Common Pitfalls & Troubleshooting**

| Issue | Solution |
|-------|----------|
| `error: expected one of ... found $x` | Check token matcher order — put specific before general |
| `cannot find macro ... in this scope` | Ensure `macro_rules!` is in scope; check `pub` visibility |
| `unresolved import` in macro | Import needed types *inside* the macro body using `::std::...` |
| `cannot find type` in derived macro | Use `syn::Type` and `quote! { #ty }` — ensure correct generics |
| `macro expansion ignores private items` | Use `pub(crate)` or `pub` in generated code if needed |
| `expansion too large` | Split into smaller macros or use procedural macros |
| `cannot use `self` in macro` | Use `Self` (capitalized) when referring to implementing type |

---

### **7. Real-World Project Snippets**

#### **Serde-style Serialize Derive (Simplified)**
```rust
#[proc_macro_derive(Serialize, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let mut s = serializer.serialize_struct(stringify!(#name), 1)?;
                #(
                    s.serialize_field(stringify!(#field), &self.#field)?;
                )*
                s.end()
            }
        }
    };
    TokenStream::from(gen)
}
```

#### **Custom `assert_eq!` with better error reporting**
```rust
macro_rules! assert_eq_verbose {
    ($left:expr, $right:expr) => {
        {
            let left_val = $left;
            let right_val = $right;
            if left_val != right_val {
                panic!(
                    "Assertion failed: {} != {}\nLeft: {:?}\nRight: {:?}",
                    stringify!($left),
                    stringify!($right),
                    left_val,
                    right_val
                );
            }
        }
    };
}
```

#### **SQLX-like Compile-Time Query Validation (Conceptual)**
```rust
// Pseudo-code — real sqlx uses SQLite/MYSQL parser
macro_rules! query {
    ($sql:expr) => {
        {
            const _: () = {
                // Parse $sql at compile time
                // Validate against DB schema
                // Generate struct with correct field names/types
            };
            // Return generated struct
            struct QueryResult { /* inferred */ }
            QueryResult { /* ... */ }
        }
    };
}
```

---

### **8. Data Table: Macro Features Summary**

| Feature | `macro_rules!` | `derive` | `attribute` | `function-like` |
|--------|----------------|----------|-------------|-----------------|
| **Can generate structs** | ❌ | ✅ | ✅ | ✅ |
| **Can generate impl blocks** | ❌ | ✅ | ✅ | ✅ |
| **Can validate syntax** | ❌ | ✅ | ✅ | ✅ |
| **Can access types** | ❌ | ✅ | ✅ | ✅ |
| **Can read file system** | ❌ | ✅ | ✅ | ✅ |
| **Can parse SQL/JSON/YAML** | ❌ | ✅ | ✅ | ✅ |
| **Compile-time execution** | ✅ | ✅ | ✅ | ✅ |
| **Runtime cost** | 0 | 0 | 0 | 0 |
| **Debuggability** | Low | Medium | Medium | Medium |
| **Learning curve** | Low | High | High | High |

---

### **9. Performance Notes**

- All macros are **zero-cost abstractions** — expanded at compile time.
- Procedural macros slow compilation slightly due to external process invocation.
- Avoid overusing macros for trivial cases — prefer functions unless codegen is essential.
- Large macro expansions increase binary size and compile times.

---

### **10. Tooling & Debugging**

- **`cargo expand`** — shows expanded macro output:
  ```bash
  cargo install cargo-expand
  cargo expand --lib
  ```
- **`rust-analyzer`** — highlights macro expansions inline in IDEs.
- **`--explain`** — for macro-related errors:
  ```bash
  rustc --explain E0425
  ```

---

### **Next Step: Synthesize AST with `syn` and `quote` to Build a Custom Domain-Specific Language (DSL) for Configuration Parsing**

> 🚀 **Advanced Topic**: **Build a compile-time configuration DSL using procedural macros that parses TOML/YAML at compile time and generates strongly-typed Rust structs with validated defaults, field constraints, and dependency resolution — without runtime overhead.**

Example goal:
```rust
#[config]
struct AppConfig {
    port: u16 = 8080,
    host: String = "localhost",
    #[required]
    api_key: String,
    #[min(10)]
    timeout_ms: u32 = 1000,
}
```

→ At compile time:  
- Parses `config.toml`  
- Validates all fields exist and match types  
- Generates `impl AppConfig { fn load() -> Result<Self, ConfigError> }`  
- Fails build if required field missing or type mismatch  

This requires:  
- `syn` to parse attribute args  
- `serde` + `toml` to decode config  
- `proc-macro2` + `quote` to emit code  
- `proc-macro-crate` to access external crates in proc-macro context  
