**1. Concurrency Fundamentals in Rust**

* **Concurrency** vs **Parallelism**

  * **Concurrency**: structuring a program to handle multiple tasks at once (interleaved).
  * **Parallelism**: executing multiple tasks simultaneously on multiple cores.
* **Why Message Passing?**

  1. **Safety**: avoids shared mutable state.
  2. **Decoupling**: sender and receiver don’t share memory.
  3. **Predictability**: clear handoff of data.

---

**2. Spawning Threads**

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from spawned thread!");
});

// Wait for the thread to finish:
handle.join().expect("Thread panicked");
println!("Back in main thread");
```

* **`thread::spawn`** returns a `JoinHandle<T>`.
* **`join()`** blocks until completion; panics propagate as errors.

---

**3. Message Passing with `std::sync::mpsc`**
The module name stands for **m**ultiple **p**roducer, **s**ingle **c**onsumer.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

// Producer
thread::spawn(move || {
    for i in 1..=5 {
        tx.send(i).expect("Failed to send");
        println!("Sent {}", i);
    }
    // Dropping `tx` closes the channel
});

// Consumer
for received in rx {
    println!("Received {}", received);
}
println!("Channel closed, consumer done");
```

**Key Points:**

* **`tx.send(val)`** returns `Result<(), SendError<T>>`.
* **`for x in rx`** loops until all senders are dropped.
* **Edge Case**: sending after all receivers dropped ⇒ `Err(SendError)`.

---

**4. Synchronous vs. Asynchronous Channels**

| Feature                    | `std::sync::mpsc` | `crossbeam::channel` |
| -------------------------- | ----------------- | -------------------- |
| Unbounded                  | ✅                 | ✅                    |
| Bounded (capacity limit)   | ❌                 | ✅                    |
| Select over multiple chans | ❌                 | ✅                    |
| Zero-copy optimization     | ❌                 | ✅                    |
| Performance (throughput)   | Moderate          | High                 |

---

**5. Using `crossbeam::channel` for Advanced Patterns**

```rust
use crossbeam::channel::{bounded, select};
use std::thread;
use std::time::Duration;

let (tx, rx) = bounded::<&str>(2); // capacity = 2

thread::spawn(move || {
    tx.send("msg1").unwrap();
    println!("Sent msg1");
    tx.send("msg2").unwrap();
    println!("Sent msg2");
    // This next send would block until a recv happens:
    tx.send("msg3").unwrap();
    println!("Sent msg3");
});

thread::spawn(move || {
    thread::sleep(Duration::from_millis(100));
    let first = rx.recv().unwrap();
    println!("Received {}", first);
});
```

* **Blocking on full** when bounded.
* **`select!`** macro allows waiting on multiple channels:

```rust
select! {
    recv(rx1) -> msg => println!("rx1: {:?}", msg),
    recv(rx2) -> msg => println!("rx2: {:?}", msg),
    default(Duration::from_secs(1)) => println!("timeout"),
}
```

---

**6. Comparing with Shared-Memory (Mutex/RwLock)**

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..5).map(|_| {
    let ctr = Arc::clone(&counter);
    thread::spawn(move || {
        let mut num = ctr.lock().unwrap();
        *num += 1;
        println!("Incremented to {}", *num);
    })
}).collect();

for h in handles { h.join().unwrap(); }
println!("Final count: {}", *counter.lock().unwrap());
```

* **Rust’s ownership** + **Mutex** ensures safe shared mutability.
* **Message passing vs Mutex**:

  * **Message passing** favors **immutability**, easier reasoning.
  * **Mutex** might be simpler for small shared state, but prone to deadlocks.

---

**7. Error Handling & Edge Cases**

1. **Channel Closed**

   ```rust
   match tx.send(val) {
       Ok(_) => println!("Sent"),
       Err(e) => eprintln!("Send failed: {:?}", e),
   }
   ```
2. **Receiver Timeout** (with Crossbeam)

   ```rust
   if let Err(err) = rx.recv_timeout(Duration::from_secs(1)) {
       println!("Timeout or closed: {:?}", err);
   }
   ```
3. **Poisoned Mutex**: calling `.lock()` may panic if another thread panicked while holding the lock.

---

**8. Design Patterns & Use Cases**

* **Worker Pool**: multiple consumers pull from one queue.
* **Pipeline**: chained channels, each stage processes and forwards.
* **Fan-out/Fan-in**: one sender to many workers, then aggregate replies.

```text
[Main Thread] --(tx)--> [Worker1] --(tx2)--> [Aggregator]
                   \--(tx)--> [Worker2] --(tx2)--> [Aggregator]
```

---

**9. Summary of Best Practices**

* **Prefer message passing** for complex workflows.
* **Use bounded channels** to apply back-pressure.
* **Handle errors** (broken channel, timeouts).
* **Leverage `select!`** for multiplexing.
* **Compare** your needs: bounded vs unbounded, performance, ordering guarantees.

---

**Master these concepts step-by-step**: start with `std::sync::mpsc`, experiment with simple send/receive; then advance to `crossbeam::channel` for bounded, select, and timeout patterns; finally, architect real-world pipelines and worker pools with robust error handling.
