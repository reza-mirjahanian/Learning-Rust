# Cold and Inline Attributes in Rust: Comprehensive Technical Reference

## 1. Basic Concepts

### 1.1 Definition and Purpose

Cold and inline attributes in Rust are function attributes that provide hints to the compiler about optimization strategies:

- `#[cold]`: Suggests the function is unlikely to be called
- `#[inline]`: Suggests the function should be inlined at call sites

```rust
#[cold]
fn unlikely_error_handler() {
    // Rarely executed error handling
}

#[inline]
fn fast_path_computation(x: i32) -> i32 {
    x * x + 1
}
```

### 1.2 Memory Representation

| Attribute | Code Size Impact | Call Overhead | Cache Behavior |
|-----------|------------------|---------------|----------------|
| `#[cold]` | May reduce size   | Normal        | Better locality|
| `#[inline]`| May increase size | Eliminated    | Potentially worse |

## 2. Cold Attribute

### 2.1 Basic Usage

```rust
#[cold]
fn handle_error(err: Error) -> ! {
    eprintln!("Fatal error: {}", err);
    std::process::exit(1);
}
```

### 2.2 Effects on Code Generation

- Places the function in a cold section (`.text.unlikely` in LLVM)
- May affect branch prediction hints
- Optimizer may prioritize other paths

### 2.3 Advanced Patterns

```rust
#[cold]
#[track_caller]
fn cold_panic(msg: &str) -> ! {
    panic!("{}", msg);
}
```

## 3. Inline Attribute

### 3.1 Basic Usage

```rust
#[inline]
fn add_one(x: i32) -> i32 {
    x + 1
}
```

### 3.2 Variants

```rust
#[inline]       // Hint to inline
#[inline(always)] // Force inline (may fail)
#[inline(never)] // Never inline
```

### 3.3 When to Use Inline

| Scenario | Recommendation |
|----------|---------------|
| Small hot functions | `#[inline]` or `#[inline(always)]` |
| Large functions | Avoid or use `#[inline(never)]` |
| Cross-crate visibility | `#[inline]` may be needed |

## 4. Combined Usage

```rust
#[inline(always)]
fn fast_path() {
    // Optimized path
}

#[cold]
#[inline(never)]
fn slow_path() {
    // Error handling
}
```

## 5. Visibility and Scoping Rules

### 5.1 Cross-Crate Behavior

- `#[inline]` is required for cross-crate inlining
- `#[cold]` affects optimization even across crates

### 5.2 Interaction with Linkage

```rust
#[inline]
#[no_mangle]
pub extern "C" fn exported_function() {
    // Can be inlined within Rust code
    // But also exported with stable symbol
}
```

## 6. Implementation Details

### 6.1 LLVM Translation

- `#[cold]` translates to LLVM's `cold` attribute
- `#[inline]` becomes LLVM's `inlinehint`

### 6.2 MIR Representation

In Mid-Level IR:
- Cold functions marked with `#[cold]`
- Inline hints preserved until codegen

## 7. Edge Cases and Limitations

### 7.1 Non-Obvious Behaviors

1. Recursive functions with `#[inline(always)]` may fail to compile
2. `#[cold]` on generic functions affects all monomorphizations
3. Inline hints may be ignored in debug mode

### 7.2 Gotchas

```rust
// This may not inline due to complexity
#[inline]
fn large_function() {
    // 100+ lines of code
}
```

## 8. Comparison with Other Languages

| Language | Cold Equivalent | Inline Equivalent |
|----------|-----------------|-------------------|
| C/C++    | `__attribute__((cold))` | `inline` |
| Java     | No equivalent | JVM decides automatically |
| Go       | No equivalent | Compiler decides |
| Swift    | `@_cold` | `@inline(__always)` |

## 9. Performance Considerations

### 9.1 Benchmark Data (Hypothetical)

| Scenario | No Attribute | Cold | Inline | Both |
|----------|-------------|------|--------|------|
| Error path (ns) | 50 | 45 | 55 | 40 |
| Hot path (ns) | 10 | 12 | 5 | 5 |

### 9.2 Code Size Impact

```text
Binary Size:
- Baseline: 100KB
- With cold: 98KB
- With inline: 105KB
- Both: 103KB
```

## 10. Advanced Patterns

### 10.1 Profile-Guided Optimization Interaction

```rust
#[cold]
#[cfg(not(pgo_generated))]
fn cold_path() { /* ... */ }
```

### 10.2 Macro Integration

```rust
macro_rules! unlikely {
    ($expr:expr) => {
        if std::intrinsics::unlikely($expr) {
            #[cold]
            fn cold_branch() { /* ... */ }
            cold_branch();
        }
    }
}
```

## 11. Attribute Combinations

### 11.1 With Other Attributes

```rust
#[cold]
#[must_use]
fn critical_result() -> Result<(), Error> {
    // ...
}

#[inline]
#[target_feature(enable = "avx2")]
fn simd_operation() {
    // ...
}
```

### 11.2 With FFI

```rust
#[inline]
#[no_mangle]
pub extern "C" fn ffi_call() {
    // Can be inlined in Rust code
    // But also callable from C
}
```

## 12. Debugging and Inspection

### 12.1 Checking Effects

```bash
# View LLVM IR to see effects
rustc --emit=llvm-ir -C opt-level=2 file.rs
```

### 12.2 Profile Counter Verification

```rust
#[cold]
fn counted_cold() {
    #[cfg(profile)]
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    
    #[cfg(profile)]
    COUNTER.fetch_add(1, Ordering::Relaxed);
}
```

## 13. Historical and Compatibility Notes

- `#[cold]` stabilized in Rust 1.0
- `#[inline]` behavior changed in Rust 1.54 (more aggressive)
- Cross-crate inlining improved in 2018 edition

## 14. Best Practices Summary

1. Use `#[cold]` for error handlers and uncommon paths
2. Prefer `#[inline]` for small, frequently called functions
3. Avoid `#[inline(always)]` unless profiling shows benefit
4. Combine with other attributes carefully
5. Measure actual performance impact

## 15. Complete Attribute Reference

### 15.1 Full Syntax Specification

```ebnf
inline_attribute = "#[inline]" | "#[inline(always)]" | "#[inline(never)]"
cold_attribute = "#[cold]"
```

### 15.2 Valid Locations

| Attribute | Valid Targets |
|-----------|--------------|
| `#[cold]` | Functions only |
| `#[inline]` | Functions, methods, closures |

## 16. Compiler Internals Deep Dive

### 16.1 MIR Pass Behavior

1. Inlining decisions made during MIR optimization
2. Cold functions marked early in pipeline
3. Final decision in LLVM backend

### 16.2 Interaction with LTO

- Link-time optimization can override some decisions
- Cold sections still preserved in LTO

## 17. Unstable and Nightly Features

```rust
#![feature(optimize_attribute)]

#[optimize(size)]
fn compact_code() {
    // May interact with cold/inline
}

#[optimize(speed)]
fn fast_code() {
    // May force different inlining
}
```