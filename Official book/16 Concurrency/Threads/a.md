**Threading in Rust: Comprehensive Reference**

---

## 1. Core Concepts & API Overview

* **`std::thread` Module**

  * **Spawn Threads**: `thread::spawn`
  * **Join Handles**: `JoinHandle<T>`
  * **Thread-local Storage**: `thread_local!` macro
  * **Panics & Propagation**: Join handle’s `join()` returns `Result<T, Box<dyn Any + Send + 'static>>`

* **Key Traits**

  * **`Send`**: Types safe to transfer ownership across threads
  * **`Sync`**: Types safe to share references across threads
  * **`'static` Bound**: Spawned threads require closures to own data with `'static` lifetime

---

## 2. Spawning Threads

```rust
use std::thread;

let handle: thread::JoinHandle<()> = thread::spawn(|| {
    println!("Hello from a new thread!");
});

// Wait for the thread to finish
handle.join().expect("Thread panicked");
```

* **Edge Cases**

  * **Closure Move**: Data captured must be `Send + 'static`
  * **Thread Panic**: `join()` returns `Err`, must handle

**Complexity**:

* Thread spawn: *O(1)*
* `join()`: *O(1)*

---

## 3. Moving Data Into Threads

```rust
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    // `v` moved here; safe
    println!("{:?}", v);
});
handle.join().unwrap();
```

* **Tip**: Use `move` to transfer ownership explicitly.
* **Tricky Part**: Borrowed references without `'static` will not compile.

---

## 4. Returning Values from Threads

```rust
let handle = thread::spawn(|| -> i32 {
    42
});
let result = handle.join().unwrap();
assert_eq!(result, 42);
```

* **Edge Case**: Return type must satisfy `Send + 'static`.

---

## 5. Scoped Threads (via `crossbeam`)

```rust
crossbeam::scope(|s| {
    s.spawn(|_| {
        // Can borrow local stack variables safely
    });
}).unwrap();
```

* **Pros vs. `std::thread`**

| Aspect           | `std::thread`            | `crossbeam::scope`              |
| ---------------- | ------------------------ | ------------------------------- |
| Borrowing locals | ❌ Requires `'static`     | ✅ Safe scoped borrows           |
| Panic handling   | Panics propagate to join | Scope panics abort all children |
| API dependency   | std only                 | External crate (crossbeam)      |

---

## 6. Thread Local Storage

```rust
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
}

COUNTER.with(|c| *c.borrow_mut() += 1);
```

* **Use Case**: Per-thread caches, counters.
* **Tricky Part**: Cannot share between threads; each thread has independent copy.

---

## 7. Synchronization Primitives

### 7.1 Channels (`std::sync::mpsc`)

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send("message").unwrap();
});
println!("{}", rx.recv().unwrap());
```

* **Complexity**: send/recv *O(1)* amortized.
* **Edge Cases**:

  * Sending after receiver dropped → `send()` returns `Err`.
  * Blocking `recv()` vs. non-blocking `try_recv()`.

### 7.2 Mutex & RwLock

```rust
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..10).map(|_| {
    let cnt = Arc::clone(&counter);
    thread::spawn(move || {
        let mut num = cnt.lock().unwrap();
        *num += 1;
    })
}).collect();

for h in handles { h.join().unwrap(); }
println!("Result: {}", *counter.lock().unwrap());
```

* **Pros/Cons**

| Primitive | Pros                               | Cons                              |
| --------- | ---------------------------------- | --------------------------------- |
| `Mutex`   | Simple mutual exclusion            | Potential for deadlocks, blocking |
| `RwLock`  | Concurrent reads, exclusive writes | Writers starve if many readers    |

---

## 8. Thread Pools & High-level Abstractions

* **`rayon` Crate** for data-parallelism:

  ```rust
  use rayon::prelude::*;
  let sum: i32 = (1..1000).into_par_iter().sum();
  ```

  * **Pros**: Easy parallel iterator, work-stealing
  * **Cons**: Less control over threads

* **Custom Thread Pools**: use `threadpool` or `tokio` for async tasks.

---

## 9. Panics & Unwind Safety

* By default Rust unwinds panics across threads.
* **`join()`** returns `Err` if panicked.
* To abort on panic, set `RUST_BACKTRACE=1` or use `std::panic::set_hook`.

---

## 10. Comparing Threads vs. Async

| Aspect       | Threads (`std::thread`)      | Async (`async`/`await`)             |
| ------------ | ---------------------------- | ----------------------------------- |
| OS Resources | One stack per thread (\~2MB) | Single-threaded, small tasks        |
| Blocking     | Blocks OS thread             | Non-blocking via executor           |
| Use Case     | CPU-bound work               | I/O-bound concurrency               |
| Complexity   | Simpler model                | Requires runtime (Tokio, async-std) |

---

## 11. Advanced Tips & Tricks

* **Name Your Threads**:

  ```rust
  thread::Builder::new()
      .name("worker-thread".into())
      .spawn(|| { /* ... */ })
      .unwrap();
  ```

* **Adjust Stack Size**:

  ```rust
  thread::Builder::new()
      .stack_size(4 * 1024 * 1024) // 4 MB
      .spawn(|| { /* deep recursion */ })
      .unwrap();
  ```

* **Affinity & NUMA**: Use OS-specific crates (`affinity`, `numa`) to bind threads to cores.

* **Thread Parking**: `std::thread::park` / `unpark` for low-level blocking.

---

## 12. Common Pitfalls

* **Data Races**: Rust’s ownership prevents this unless using `unsafe`.
* **Deadlocks**: Lock ordering must be consistent.
* **Resource Leaks**: Threads that never join can keep the process alive.
* **Stack Overflows**: Beware deep recursion → increase stack size.

---

