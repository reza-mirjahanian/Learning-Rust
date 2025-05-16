

---

## 🧱 **1. Foundations of Threads in Rust**

Rust provides **native support for threads** via the `std::thread` module. Threads allow concurrent execution of code.

### ✅ Creating a Thread

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a new thread!");
    });

    println!("Hello from the main thread!");

    handle.join().unwrap();
}
```

* `thread::spawn`: creates a new thread.
* `join()`: waits for the thread to finish.

---

## 🧠 **2. Closures and Move Semantics in Threads**

Rust threads are **safe and strict** about data ownership.

### 🟡 Problem Without `move`

```rust
fn main() {
    let v = vec![1, 2, 3];

    let handle = std::thread::spawn(|| {
        println!("{:?}", v); // ❌ ERROR: cannot access `v`
    });

    handle.join().unwrap();
}
```

### ✅ Correct with `move`

```rust
fn main() {
    let v = vec![1, 2, 3];

    let handle = std::thread::spawn(move || {
        println!("{:?}", v); // ✅ `v` moved into thread
    });

    handle.join().unwrap();
}
```

---

## 🧩 **3. Synchronization Primitives**

Rust provides **safe and powerful tools** to synchronize threads:

### 🔐 Mutex (Mutual Exclusion)

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

* `Arc`: Atomically Reference Counted pointer (for shared ownership across threads).
* `Mutex`: Ensures **only one thread can access the data** at a time.

### ⚠️ Deadlock Example

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(1));
    let b = Arc::new(Mutex::new(2));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);

    let t1 = thread::spawn(move || {
        let _a = a1.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(50));
        let _b = b1.lock().unwrap(); // 🚨 Deadlock possibility
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);

    let t2 = thread::spawn(move || {
        let _b = b2.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(50));
        let _a = a2.lock().unwrap(); // 🚨 Deadlock possibility
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
```

---

## 📬 **4. Channels (Thread Communication)**

### 📨 Sending Values

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("Hello")).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

* `mpsc`: multiple producer, single consumer.
* `send` and `recv` for passing values between threads.

### 📦 Multiple Sends (Cloning Transmitters)

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..5 {
        let val = rx.recv().unwrap();
        println!("Got: {}", val);
    }
}
```

---

## 🕹️ **5. Thread Lifecycle and Panic Handling**

### ✅ Handling Thread Panic Gracefully

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        panic!("Oops!");
    });

    match handle.join() {
        Ok(_) => println!("Thread finished successfully"),
        Err(e) => println!("Thread panicked: {:?}", e),
    }
}
```

---

## ⚖️ **6. Threads vs Async**

| Feature        | `std::thread`      | `async/await`                            |
| -------------- | ------------------ | ---------------------------------------- |
| Model          | OS thread per task | Single-threaded/event loop               |
| Context switch | Costly             | Cheap                                    |
| Parallelism    | Yes                | No (needs `tokio`/`async-std` + threads) |
| Best for       | CPU-bound          | IO-bound                                 |

---

## ⚙️ **7. Advanced Thread Patterns**

### 🧵 Thread Pools

Use `rayon` or `threadpool` crates for efficient job dispatching.

```rust
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let pool = ThreadPool::new(4);
    let (tx, rx) = channel();

    for i in 0..8 {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(i * i).unwrap();
        });
    }

    for _ in 0..8 {
        println!("Received: {}", rx.recv().unwrap());
    }
}
```

---

## 🔬 **8. Edge Cases and Pitfalls**

### 🔄 Shared Mutability without Mutex (Unsafe)

```rust
static mut COUNTER: u32 = 0;

fn main() {
    let handle = std::thread::spawn(|| {
        unsafe {
            COUNTER += 1;
        }
    });

    handle.join().unwrap();
    unsafe {
        println!("Counter: {}", COUNTER);
    }
}
```

⚠️ Avoid this unless you're doing FFI or low-level performance-sensitive code. Always prefer `Arc<Mutex<...>>`.

---

## 🧠 **9. Expert Concepts**

### ✅ Poisoned Mutex

When a thread panics while holding a lock:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let d1 = Arc::clone(&data);
    let _ = thread::spawn(move || {
        let _guard = d1.lock().unwrap();
        panic!("uh oh");
    }).join();

    let result = data.lock();
    match result {
        Ok(val) => println!("Value: {}", val),
        Err(poisoned) => {
            println!("Recovering from poisoned mutex");
            let val = poisoned.into_inner();
            println!("Recovered value: {}", val);
        }
    }
}
```

---

## 📦 **10. Recommended Crates for Threading**

| Crate        | Use Case                          |
| ------------ | --------------------------------- |
| `crossbeam`  | Advanced channels & atomics       |
| `rayon`      | Data parallelism, easy map/reduce |
| `tokio`      | Async runtime with multithreading |
| `threadpool` | Simple thread pooling             |

---

Next, we can go deeper into:

* Crossbeam threads and scoped threading
* Designing lock-free structures
* Using atomics (`AtomicUsize`, etc.)
* Building thread-safe abstractions
* Comparison with other languages’ threading models


