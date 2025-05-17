### **Message Passing (Threads) in Rust**

#### **1. Foundational Concepts**
- **Channels**: Primary mechanism for message passing (`std::sync::mpsc`).
- **Ownership**: Transferred via `move` keyword when spawning threads.
- **Thread Safety**: Guaranteed by Rust’s type system (no data races).

---

#### **2. Basic Usage**
```rust
use std::sync::mpsc;
use std::thread;

// Create an asynchronous channel (unbounded)
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("Hello from thread!").unwrap();
});

let received = rx.recv().unwrap();
println!("{}", received); // Output: "Hello from thread!"
```

---

#### **3. Multiple Producers (MPSC)**
```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone(); // Clone sender for second thread

// Producer 1
thread::spawn(move || {
    tx.send(1).unwrap();
});

// Producer 2
thread::spawn(move || {
    tx1.send(2).unwrap();
});

// Collect results
assert_eq!(rx.recv().unwrap(), 1);
assert_eq!(rx.recv().unwrap(), 2);
```

---

#### **4. Sync vs Async Channels**
| Feature               | `mpsc::channel()` (Async) | `mpsc::sync_channel(n)` (Sync) |
|-----------------------|---------------------------|--------------------------------|
| Buffer Size           | Unbounded                 | Bounded (`n` capacity)         |
| Blocking Behavior     | Never blocks sender       | Blocks sender when full        |
| Use Case              | High-throughput           | Backpressure management        |

**Example (Sync Channel):**
```rust
let (tx, rx) = mpsc::sync_channel(2); // Buffer size = 2

tx.send(1).unwrap();
tx.send(2).unwrap();
// tx.send(3).unwrap(); // Would block until space is freed
```

---

#### **5. Error Handling & Edge Cases**
- **Disconnected Channels**: 
  ```rust
  let (tx, rx) = mpsc::channel::<i32>();
  drop(rx); // Explicitly drop receiver
  assert!(tx.send(42).is_err()); // SendError
  ```
  
- **Non-Blocking Receives**:
  ```rust
  match rx.try_recv() {
      Ok(msg) => println!("Got {}", msg),
      Err(mpsc::TryRecvError::Empty) => println!("No messages"),
      Err(mpsc::TryRecvError::Disconnected) => println!("All senders dropped"),
  }
  ```

- **Timeout Receives**:
  ```rust
  use std::time::Duration;
  match rx.recv_timeout(Duration::from_secs(1)) {
      Ok(msg) => println!("{}", msg),
      Err(mpsc::RecvTimeoutError::Timeout) => println!("Timed out"),
      Err(mpsc::RecvTimeoutError::Disconnected) => println!("Channel closed"),
  }
  ```

---

#### **6. Advanced Patterns**
**Iterating Over Messages**:
```rust
for msg in rx {
    println!("Received: {}", msg);
} // Exits when all senders are dropped
```

**Channel Selection (Using `crossbeam`)**: 
```rust
use crossbeam::channel::{bounded, select};

let (tx1, rx1) = bounded(1);
let (tx2, rx2) = bounded(1);

select! {
    recv(rx1) -> msg => println!("From rx1: {:?}", msg),
    recv(rx2) -> msg => println!("From rx2: {:?}", msg),
    default => println!("No messages"),
}
```

---

#### **7. Comparison with Shared Memory**
| Aspect                | **Message Passing**              | **Shared Memory**               |
|-----------------------|-----------------------------------|----------------------------------|
| Safety                | No data races by design           | Requires locks (e.g., `Mutex`)  |
| Performance           | Copy overhead per message         | In-place access                 |
| Use Case              | Isolated tasks                   | High-frequency data sharing     |

**Shared Memory Example**:
```rust
use std::sync::{Arc, Mutex};

let data = Arc::new(Mutex::new(0));
let data_clone = Arc::clone(&data);

thread::spawn(move || {
    *data_clone.lock().unwrap() += 1;
});

println!("Result: {}", *data.lock().unwrap()); // 1
```

---

#### **8. Key Takeaways**
- **Use `mpsc::channel`** for fire-and-forget messaging.
- **Use `sync_channel`** for backpressure-sensitive systems.
- **Prefer message passing** when tasks are independent; use **shared memory** for micro-optimizations.
- Always handle **disconnection errors** gracefully.