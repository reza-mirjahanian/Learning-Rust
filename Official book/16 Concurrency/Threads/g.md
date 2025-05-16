**Creating Threads**

- **Basic Thread Creation**
  ```rust
  use std::thread;
  thread::spawn(|| {
      println!("Hello from a thread!");
  });
  ```

- **Moving Data into Threads**
  ```rust
  let data = vec![1, 2, 3];
  thread::spawn(move || {
      println!("Data: {:?}", data);
  });
  ```

- **Scoped Threads (Stabilized in 1.63)**
  ```rust
  thread::scope(|s| {
      let mut data = vec![1, 2, 3];
      s.spawn(|| {
          data.push(4); // Safe reference to parent's stack
      });
  });
  ```

- **Joining Threads**
  ```rust
  let handle = thread::spawn(|| "Result");
  let result = handle.join().unwrap(); // Returns "Result"
  ```

- **Panics in Threads**
  ```rust
  let handle = thread::spawn(|| {
      panic!("Oops!");
  });
  assert!(handle.join().is_err()); // Handle panics
  ```

---

**Communication Between Threads**

- **Channels (`mpsc`)**  
  ```rust
  use std::sync::mpsc;
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
      tx.send("Hello").unwrap();
  });
  let msg = rx.recv().unwrap(); // "Hello"
  ```

- **Shared State with `Mutex`**
  ```rust
  use std::sync::{Arc, Mutex};
  let counter = Arc::new(Mutex::new(0));
  let counter_clone = Arc::clone(&counter);
  thread::spawn(move || {
      let mut num = counter_clone.lock().unwrap();
      *num += 1;
  });
  ```

- **Shared State with `RwLock`**
  ```rust
  use std::sync::{Arc, RwLock};
  let data = Arc::new(RwLock::new(0));
  let data_clone = Arc::clone(&data);
  thread::spawn(move || {
      let mut num = data_clone.write().unwrap();
      *num += 1;
  });
  ```

- **Atomic Types**
  ```rust
  use std::sync::atomic::{AtomicUsize, Ordering};
  let atomic = AtomicUsize::new(0);
  let atomic_clone = &atomic;
  thread::spawn(move || {
      atomic_clone.fetch_add(1, Ordering::Relaxed);
  });
  ```

---

**Synchronization Primitives**

- **Deadlock Example**  
  ```rust
  let a = Arc::new(Mutex::new(0));
  let b = Arc::new(Mutex::new(0));
  let a1 = Arc::clone(&a); let b1 = Arc::clone(&b);
  thread::spawn(move || {
      let _guard_a = a1.lock().unwrap();
      let _guard_b = b1.lock().unwrap(); // Deadlock if main thread holds b
  });
  ```

- **Condition Variables**
  ```rust
  use std::sync::Condvar;
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let pair_clone = Arc::clone(&pair);
  thread::spawn(move || {
      let mut started = false;
      loop {
          let mut guard = pair_clone.0.lock().unwrap();
          if *guard {
              break;
          }
          guard = pair_clone.1.wait(guard).unwrap();
      }
  });
  ```

---

**Thread Safety and Traits**

- **`Send` and `Sync` Traits**
  - `Send`: Data can be moved to another thread.
  - `Sync`: Data can be shared between threads.
  - **Examples**: `Arc<T>` is `Sync` if `T: Send + Sync`.

- **Non-`Send` Types (e.g., `Rc<T>)`**
  ```rust
  // Fails to compile: `Rc` is not `Send`
  let rc = Rc::new(0);
  thread::spawn(move || {
      println!("{}", *rc);
  });
  ```

---

**Performance Considerations**

| **Aspect**       | **Pros**                     | **Cons**                          |
|-------------------|------------------------------|-----------------------------------|
| **Threads**       | True parallelism             | High memory usage (~1MB per thread) |
| **Async**         | Low overhead, high concurrency | Complex, less intuitive         |

- **Time Complexity**
  - Thread creation: **O(1)** (but costly OS-level).
  - Mutex lock/unlock: **O(1)** (but may block).
  - Channel send/receive: **O(1)** (but depends on contention).

---

**Advanced Features**

- **Thread-local Storage**
  ```rust
  thread_local! {
      static COUNTER: RefCell<u32> = RefCell::new(0);
  }
  COUNTER.with(|c| *c.borrow_mut() += 1);
  ```

- **Thread Naming and Identification**
  ```rust
  let builder = thread::Builder::new().name("worker".into());
  builder.spawn(|| {}).unwrap();
  ```

- **Panic Handling**
  ```rust
  let handle = thread::spawn(|| panic!("Abort!"));
  match handle.join() {
      Err(e) => println!("Thread panicked: {:?}", e),
      _ => {}
  }
  ```

---

**Comparisons with Similar Concepts**

| **Feature**         | **Threads**                   | **Async/await**                  |
|----------------------|-------------------------------|-----------------------------------|
| **Use Case**         | CPU-bound tasks               | I/O-bound tasks                   |
| **Overhead**         | High (OS threads)             | Low (event loop)                  |
| **Complexity**       | Simple model                  | Complex state machines            |

| **Primitive**        | **Mutex**                     | **Channels**                      |
|----------------------|-------------------------------|-----------------------------------|
| **Safety**           | Risk of deadlocks             | Safer message-passing model       |
| **Use Case**         | Shared mutable state          | Task coordination                 |

---

**Edge Cases and Gotchas**

- **Borrowing in Threads**
  ```rust
  let data = vec![1, 2, 3];
  thread::spawn(|| {
      // Fails: `data` not moved
      println!("{:?}", data);
  });
  ```

- **Poisoned Mutex**
  ```rust
  let mutex = Arc::new(Mutex::new(0));
  let mutex_clone = Arc::clone(&mutex);
  let handle = thread::spawn(move || {
      let _ = mutex_clone.lock().unwrap();
      panic!("Poison!");
  });
  handle.join().unwrap_err();
  let result = mutex.lock(); // Returns `Err`, poisoned
  ```

- **Detached Threads**
  ```rust
  let _handle = thread::spawn(|| {});
  // No `join()` -> Thread may exit early
  ```

---

