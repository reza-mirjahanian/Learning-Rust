# Static in Rust - Complete Reference

## 1. Static Variables (`static`)

### Basic Syntax
```rust
static VARIABLE_NAME: type = value;
static mut MUTABLE_VAR: type = value;
```

### Key Properties
- Lives for entire program duration
- Has fixed memory location
- Must be initialized with constant expressions
- Must have explicit type annotation
- Global scope
- Must be `'static` lifetime

### Examples
```rust
static MAX_POINTS: u32 = 100_000;
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;
```

## 2. Safety Rules

### Immutable Statics
- Safe to access from any thread
- Cannot be modified
- Can be used in constant contexts

### Mutable Statics
- Must use `unsafe` block to access/modify
- Risk of data races in multithreaded contexts
- Not allowed in constant contexts
```rust
unsafe {
    COUNTER += 1;
}
```

## 3. Common Use Cases

### Configuration
```rust
static CONFIG: Config = Config {
    max_connections: 100,
    timeout: 30,
};
```

### Global Counters
```rust
static mut GLOBAL_COUNTER: u32 = 0;

unsafe fn increment_counter() {
    GLOBAL_COUNTER += 1;
}
```

### String Constants
```rust
static APP_NAME: &str = "My Application";
static ERROR_MESSAGE: &'static str = "An error occurred";
```

## 4. Advanced Patterns

### Lazy Static Initialization
Using `lazy_static` crate:
```rust
use lazy_static::lazy_static;

lazy_static! {
    static ref HASHMAP: HashMap<u32, String> = {
        let mut m = HashMap::new();
        m.insert(0, "foo".to_string());
        m
    };
}
```

### Thread-Safe Mutable Statics
Using `std::sync`:
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

// Safe to use across threads
fn increment() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}
```

## 5. Best Practices

### Do's
- ✅ Use for truly global state
- ✅ Use atomic types for thread-safe mutable statics
- ✅ Consider `lazy_static` for complex initialization
- ✅ Use const items instead when possible

### Don'ts
- ❌ Avoid mutable statics when possible
- ❌ Don't use for large amounts of data
- ❌ Don't use for values that could be passed as parameters
- ❌ Don't use unsafe blocks unnecessarily

## 6. Static vs Const

### Key Differences
| Feature | Static | Const |
|---------|---------|--------|
| Memory Location | Fixed | Inlined |
| Mutability | Possible (unsafe) | Never |
| References | Allowed | Limited |
| Initialization | Runtime possible | Compile-time only |

## 7. Memory Considerations

### Memory Layout
- Stored in `.data` or `.bss` segments
- Takes up space in binary
- Zero-initialized statics go to `.bss`
- Non-zero initialized statics go to `.data`

### Performance Impact
- No allocation overhead at runtime
- Direct memory access
- Potential cache implications
- Memory always reserved even if unused

## 8. Common Patterns with Static

### Singleton Pattern
```rust
static INSTANCE: OnceCell<MyType> = OnceCell::new();

impl MyType {
    fn instance() -> &'static MyType {
        INSTANCE.get_or_init(|| MyType::new())
    }
}
```

### Error Messages
```rust
static ERRORS: [&str; 3] = [
    "Error 1",
    "Error 2",
    "Error 3"
];
```

### Module Constants
```rust
pub static MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");
```

## 9. Thread Safety

### Safe Patterns
```rust
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SAFE_COUNTER: Mutex<u32> = Mutex::new(0);
}
```

### Atomic Operations
```rust
use std::sync::atomic::{AtomicBool, Ordering};

static FLAG: AtomicBool = AtomicBool::new(false);

fn set_flag() {
    FLAG.store(true, Ordering::SeqCst);
}
```