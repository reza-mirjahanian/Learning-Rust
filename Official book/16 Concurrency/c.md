

---

# Rust Concurrency Technical Reference Guide

This document provides a detailed reference on concurrency features in the Rust programming language, covering standard library components, core concepts, advanced usage, internals, and best practices.

## 1. Core Concepts: Threads and Data Sharing

Rust's approach to concurrency is heavily influenced by its ownership system. The goal is to prevent data races at compile time.

### 1.1. OS Threads (`std::thread`)

Rust provides a direct interface to the operating system's threads.

**Purpose:** To execute multiple sequences of instructions concurrently, typically leveraging multiple CPU cores for CPU-bound tasks or managing multiple independent tasks.

**Basic Usage:**

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        // Code executed in the new thread
        for i in 1..5 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // Sleep briefly
        }
    });

    // Code executed in the main thread
    for i in 1..3 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1)); // Sleep briefly
    }

    // Wait for the spawned thread to finish
    handle.join().expect("Spawned thread panicked");
    println!("Spawned thread finished.");
}
```

**Details:**

*   **`thread::spawn`:** Creates a new OS thread. It takes a closure (`FnOnce + Send + 'static`) as an argument.
    *   The closure must be `'static` because the thread's lifetime is not known at compile time; it could outlive the scope where it was spawned.
    *   The closure must be `Send` because the data it captures needs to be safely moved into the new thread.
*   **`thread::JoinHandle<T>`:** A handle returned by `thread::spawn`. `T` is the type of the value returned by the closure.
    *   **`handle.join()`:** Blocks the current thread until the thread represented by the handle finishes execution.
    *   Returns a `Result<T, Any>`, where `T` is the return value of the closure, and `Any` is the panic payload if the thread panicked. Joining a panicked thread propagates the panic.
*   **Detaching Threads:** If `join()` is not called, the thread will detach when the `JoinHandle` is dropped. The thread will continue executing in the background and will exit when its work is done or when the main process exits. However, it's generally recommended to `join()` to ensure threads complete and handle potential panics.
*   **Thread-Specific Data:** Threads can access data from their spawning environment by capturing variables in the closure. Captured variables must satisfy the `Send` bound (or be explicitly synchronized).
    ```rust
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || { // `move` transfers ownership of `v`
        println!("Vector from thread: {:?}", v);
    });
    // `v` is moved, cannot be used here
    handle.join().unwrap();
    ```

**Limitations/Gotchas:**

*   Spawning too many OS threads can consume significant memory and system resources, leading to performance degradation or crashes.
*   Sharing *mutable* data between threads is unsafe by default and requires explicit synchronization primitives (see below).

### 1.2. The `Send` and `Sync` Traits

These are fundamental marker traits (also known as auto traits) that enable Rust's compile-time concurrency safety. They are automatically implemented for most types unless a type contains a raw pointer, `UnsafeCell`, or explicitly opts out.

**Purpose:** To guarantee that types can be safely transferred between threads (`Send`) or safely shared between threads (`Sync`).

*   **`Send`:** A type `T` is `Send` if it is safe to move a value of type `T` to another thread. Almost all primitive types, collections like `Vec` and `HashMap`, and most types composed of `Send` types are `Send`. Types containing raw pointers (`*const T`, `*mut T`) are generally *not* `Send* unless wrapped in a synchronization primitive or safe abstraction.
*   **`Sync`:** A type `T` is `Sync` if it is safe for multiple threads to have *references* (`&T`) to a value of type `T` concurrently. This is equivalent to saying that `&T` is `Send`. Types that allow interior mutability without synchronization (like `RefCell`) are *not* `Sync`. Types composed of `Sync` types are generally `Sync`.

**Relation:** If a type `T` is `Sync`, then `&T` is `Send`.

**Auto Traits:** `Send` and `Sync` are automatically derived by the compiler for types whose components are `Send` and `Sync`.

**Manual Implementation (`unsafe impl`):**

You can manually implement `Send` or `Sync` for a type using `unsafe impl`. This is *highly unsafe* and should only be done if you are certain that the type's internal state is handled in a thread-safe manner, despite containing non-`Send`/non-`Sync` components (e.g., wrapping raw pointers that are protected by a lock).

```rust
use std::ptr::NonNull;
use std::sync::Mutex;

// This struct contains a raw pointer, which is !Send and !Sync by default.
// Sharing or moving this would be unsafe without external synchronization.
struct MyData {
    ptr: NonNull<u8>,
    len: usize,
}

// NOT safe to implement Send/Sync without external guarantees!
// This is just an example of the syntax.
// If this struct were *always* protected by a Mutex, the Mutex would
// make the *combination* safe, but the struct itself might still be !Send/!Sync.
/*
unsafe impl Send for MyData {} // This asserts MyData can be moved between threads
unsafe impl Sync for MyData {} // This asserts &MyData can be sent between threads
*/

// A safe way to share MyData would be:
struct SafeData {
    data: Mutex<MyData>, // Mutex is Send and Sync if MyData is Send
}

// If MyData is !Send, Mutex<MyData> is !Send.
// If MyData is !Sync, Mutex<MyData> is !Sync.
// The safety comes from the Mutex *requiring* mutable access (via lock)
// which is only granted to one thread at a time.
// The `lock()` method on `Mutex<T>` requires `T` to be `Send` to return `MutexGuard<'_, T>`.
// The `Mutex<T>` itself requires `T` to be `Send` to be `Send`, and `T` to be `Send` to be `Sync`.
// Wait, the rules are slightly subtle:
// Mutex<T> is Send if T is Send.
// Mutex<T> is Sync if T is Send.
// RwLock<T> is Send if T is Send.
// RwLock<T> is Sync if T is Send + Sync. (Because &T is accessible via read lock)
// Arc<T> is Send if T is Send + Sync.
// Arc<T> is Sync if T is Send + Sync.
// Rc<T> is !Send and !Sync.

// Correct understanding:
// To send T across threads, T must be Send.
// To share &T across threads (multiple threads holding &T), T must be Sync.
// Mutex<T> allows *mutable* access to T, but only one thread at a time.
// If T is Send, Mutex<T> is Send and Sync. You can move the Mutex between threads (Send),
// and you can have multiple threads holding *references* (`&Mutex<T>`) to the *same* Mutex (Sync).
// The safety comes from the fact that `&Mutex<T>` only allows you to call `.lock()`,
// which blocks until unique mutable access is available.

// Example of a type that is !Send and !Sync: Rc<T>
use std::rc::Rc;
// let rc_data = Rc::new(5);
// thread::spawn(move || { // ERROR: Rc<i32> is not Send
//     println!("{}", rc_data);
// });

// Example of a type that is Send but !Sync: Cell<T> (without Sync bounds)
// Cell<T> allows interior mutability but is not thread-safe.
use std::cell::Cell;
// let cell_data = Cell::new(5);
// // Can't share &Cell<i32> between threads
// thread::spawn(|| { // ERROR: Cell<i32> is not Sync (and thus &Cell<i32> is not Send)
//    // cell_data.set(10);
// });
// // Can *move* Cell<i32> if T is Send
// thread::spawn(move || { // OK if Cell<T> is Send (i32 is Send)
//     cell_data.set(10);
//     println!("{}", cell_data.get());
// }).join().unwrap();
```

**Visibility/Scoping:** `Send` and `Sync` are traits, and their implementation status for a type is part of the type's definition, not tied to specific scopes or visibility keywords (`pub`, `crate`, etc.). The compiler checks these bounds wherever concurrent operations are attempted (e.g., `thread::spawn`, `std::sync` types, `async` futures).

**Gotchas:**

*   Forgetting `move` with `thread::spawn` can lead to compiler errors if the closure captures non-`Copy` variables by reference, as the compiler cannot guarantee the references remain valid for the `'static` lifetime of the thread.
*   Using `unsafe impl Send` or `unsafe impl Sync` incorrectly is a common source of subtle, hard-to-debug data races.

### 1.3. Sharing Immutable Data: `Arc<T>`

When multiple threads need to read the *same* data, but not modify it, shared immutable access is sufficient. `Arc<T>` (Atomic Reference Counted) is the thread-safe version of `Rc<T>`.

**Purpose:** To allow multiple threads to share ownership of a single, read-only value.

**Basic Usage:**

```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let numbers: Vec<i32> = (1..100).collect();
    // Wrap the data in Arc for shared ownership
    let shared_numbers = Arc::new(numbers);
    let mut handles = vec![];

    for i in 0..4 {
        // Create a new Arc reference for each thread
        let shared_numbers_clone = Arc::clone(&shared_numbers);
        let handle = thread::spawn(move || {
            // Threads can access the shared data immutably
            let chunk_size = shared_numbers_clone.len() / 4;
            let start = i * chunk_size;
            let end = (i + 1) * chunk_size;
            let sum: i32 = shared_numbers_clone[start..end].iter().sum();
            println!("Thread {} sum: {}", i, sum);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // The original Arc still exists until all clones are dropped
    println!("Total numbers: {}", shared_numbers.len());
}
```

**Details:**

*   **`Arc<T>`:** Provides shared ownership via reference counting. The count is atomic, making it safe for concurrent access.
*   **`Arc::clone(&arc)`:** Increments the atomic reference count and returns a new `Arc` pointing to the same data. This new `Arc` can be moved to another thread.
*   **Dropping `Arc`:** When an `Arc` is dropped, the atomic reference count is decremented. If the count reaches zero, the underlying data is dropped.
*   **`T` must be `Send` and `Sync`:** For `Arc<T>` to be `Send` (so you can move clones to other threads) and `Sync` (so you can share `&Arc<T>` between threads), the inner type `T` must be both `Send` and `Sync`. This ensures that multiple immutable references (`&T`) or a single owned `T` are safe across threads.

**Internals:** `Arc` contains a pointer to the data and a separate pointer to a shared allocation holding the atomic reference count and potentially other metadata (like a weak count). `Arc::clone` and `Drop` use atomic operations (`fetch_add`, `fetch_sub`) on the reference count.

**Comparison with `Rc`:**

| Feature          | `Rc<T>`                      | `Arc<T>`                          |
| :--------------- | :--------------------------- | :-------------------------------- |
| Thread Safety    | No (`!Send`, `!Sync`)        | Yes (`Send`, `Sync` if `T` is)    |
| Overhead         | Less (non-atomic count)      | More (atomic count)               |
| Use Case         | Single-threaded shared ownership | Multi-threaded shared ownership   |
| Interior Mutability | `RefCell<T>` (`Rc<RefCell<T>>`) | `Mutex<T>` or `RwLock<T>` (`Arc<Mutex<T>>`, `Arc<RwLock<T>>`) |

### 1.4. Sharing Mutable Data: `Mutex<T>` and `Arc<Mutex<T>>`

To safely share *mutable* data between threads, a synchronization primitive is required. `Mutex` (Mutual Exclusion) is the most common.

**Purpose:** To allow only one thread at a time to access and modify a shared resource.

**Basic Usage:**

```rust
use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    // Wrap the data in Mutex for mutable access control
    // Wrap the Mutex in Arc for shared ownership across threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the Arc for each thread
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock the mutex to get mutable access
            let mut num = counter_clone.lock().expect("Mutex was poisoned");
            // MutexGuard provides mutable access to the inner data
            *num += 1;
            // MutexGuard is dropped when `num` goes out of scope, releasing the lock
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Lock the mutex in the main thread to read the final value
    println!("Result: {}", *counter.lock().unwrap()); // Expect 10
}
```

**Details:**

*   **`Mutex<T>`:** A mutual exclusion primitive. It wraps a value of type `T`.
*   **`mutex.lock()`:** Attempts to acquire the lock.
    *   If the lock is available, it is acquired, and a `MutexGuard<'_, T>` is returned. This guard provides exclusive, mutable access (`&mut T`) to the wrapped data.
    *   If the lock is held by another thread, the current thread blocks until the lock is released.
    *   Returns a `Result<MutexGuard<'_, T>, PoisonError>`.
*   **`MutexGuard<'_, T>`:** A smart pointer that dereferences to `&mut T`. It implements `DerefMut`.
*   **Lock Release:** The lock is automatically released when the `MutexGuard` is dropped (thanks to RAII - Resource Acquisition Is Initialization). This happens when the variable holding the guard goes out of scope.
*   **`PoisonError`:** If a thread holding a lock panics, the mutex becomes "poisoned". Subsequent attempts to `lock()` will return a `PoisonError`. This indicates that the data protected by the mutex might be in an inconsistent state due to the panic. You can choose to ignore the poisoning (`.unwrap_or_else(|e| e.into_inner())`) but it's often safer to propagate the error or let the program exit.
*   **`Arc<Mutex<T>>`:** The standard pattern for sharing a mutable value controlled by a mutex across multiple threads. `Arc` provides shared ownership of the `Mutex`, and the `Mutex` provides exclusive mutable access to the inner `T`.

**Internals:** `std::sync::Mutex` is typically a wrapper around the operating system's mutex primitive (e.g., `pthread_mutex_t` on Unix, `CRITICAL_SECTION` on Windows). This involves system calls for locking and unlocking.

**Visibility/Scoping:** Standard Rust visibility rules apply to the `Mutex` itself and the data it contains. The *lock* acquired via `lock()` is tied to the scope of the `MutexGuard` variable due to RAII.

**Limitations/Gotchas:**

*   **Deadlocks:** If threads attempt to acquire multiple locks in different orders, they can deadlock.
    ```rust
    // Thread 1: lock A, then lock B
    // Thread 2: lock B, then lock A
    // Potential deadlock: Thread 1 holds A, waits for B. Thread 2 holds B, waits for A.
    ```
*   **Lock Granularity:** Holding a lock for too long can reduce concurrency. Try to minimize the time spent inside a critical section.
*   **Poisoning:** Unhandled `PoisonError` can prevent access to the protected data.

### 1.5. Sharing Data with Multiple Readers or a Single Writer: `RwLock<T>` and `Arc<RwLock<T>>`

`RwLock` (Read-Write Lock) allows multiple readers or a single writer to access the data.

**Purpose:** To allow concurrent read access while ensuring exclusive write access. This can offer better performance than `Mutex` when reads are much more frequent than writes.

**Basic Usage:**

```rust
use std::thread;
use std::sync::{RwLock, Arc};

fn main() {
    let data = Arc::new(RwLock::new(String::from("initial data")));
    let mut handles = vec![];

    // Spawn reader threads
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            // Acquire a read lock
            let reader = data_clone.read().expect("RwLock poisoned");
            println!("Reader {}: {}", i, *reader);
            // Read lock is released when `reader` goes out of scope
        }));
    }

    // Spawn a writer thread
    let data_clone = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        // Acquire a write lock
        let mut writer = data_clone.write().expect("RwLock poisoned");
        writer.push_str(" modified");
        println!("Writer: data modified");
        // Write lock is released when `writer` goes out of scope
    }));

    // Spawn more reader threads after the writer might have started
     for i in 5..10 {
        let data_clone = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let reader = data_clone.read().expect("RwLock poisoned");
            println!("Reader {}: {}", i, *reader);
        }));
    }


    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final data: {}", *data.read().unwrap());
}
```

**Details:**

*   **`RwLock<T>`:** A read-write lock primitive. Wraps a value of type `T`.
*   **`rwlock.read()`:** Attempts to acquire a read lock. Multiple threads can hold a read lock concurrently. Blocks if a write lock is currently held. Returns `Result<RwLockReadGuard<'_, T>, PoisonError>`.
*   **`rwlock.write()`:** Attempts to acquire a write lock. Only one thread can hold a write lock at a time. Blocks if any read or write lock is held. Returns `Result<RwLockWriteGuard<'_, T>, PoisonError>`.
*   **`RwLockReadGuard<'_, T>`:** Provides shared, immutable access (`&T`) to the wrapped data. Implements `Deref`.
*   **`RwLockWriteGuard<'_, T>`:** Provides exclusive, mutable access (`&mut T`) to the wrapped data. Implements `DerefMut`.
*   **Lock Release:** Locks are released when their respective guards (`RwLockReadGuard`, `RwLockWriteGuard`) are dropped (RAII).
*   **`PoisonError`:** Similar to `Mutex`, `RwLock` can be poisoned if a thread panics while holding a write lock. Read locks are not poisoned by panics.
*   **`Arc<RwLock<T>>`:** The standard pattern for sharing a mutable value controlled by an `RwLock` across multiple threads. `Arc` provides shared ownership of the `RwLock`, and the `RwLock` provides concurrent read access or exclusive write access.
*   **`T` must be `Send` and `Sync`:** For `Arc<RwLock<T>>` to be `Send` and `Sync`, the inner type `T` must be both `Send` and `Sync`.

**Internals:** Similar to `Mutex`, `std::sync::RwLock` is usually a wrapper around OS read-write lock primitives.

**Visibility/Scoping:** Standard Rust visibility rules apply. Locks are tied to the scope of the guards.

**Limitations/Gotchas:**

*   **Starvation:** Depending on the OS implementation, writers might be starved if there's a constant stream of readers acquiring read locks. Similarly, readers might be starved by a constant stream of writers.
*   **Deadlocks:** Possible with multiple `RwLock`s, similar to `Mutex`. Also possible if a thread tries to acquire a read lock while already holding a write lock on the same `RwLock` (or vice versa, depending on reentrancy support, which `std::sync::RwLock` does *not* guarantee).
*   **Poisoning:** Only occurs on writer panic.

**Comparison of Synchronization Primitives:**

| Primitive      | Access Model                | Best Use Case                     | Overhead      | Poisoning Risk |
| :------------- | :-------------------------- | :-------------------------------- | :------------ | :------------- |
| `Mutex<T>`     | Exclusive (1 reader OR 1 writer) | General mutable sharing           | Moderate (OS call) | Yes (on panic) |
| `RwLock<T>`    | Concurrent Readers OR 1 Writer | Frequent reads, infrequent writes | Moderate (OS call) | Yes (on writer panic) |
| `Atomic<T>`    | Exclusive (via atomic ops)  | Simple numeric/boolean updates    | Low (CPU instr) | No             |

### 1.6. Coordinating Threads: `Condvar`

`Condvar` (Condition Variable) is used in conjunction with a `Mutex` to allow threads to wait for a certain condition to become true.

**Purpose:** To block a thread until notified by another thread that some state protected by a mutex has changed.

**Basic Usage:**

```rust
use std::thread;
use std::sync::{Mutex, Condvar, Arc};
use std::collections::VecDeque;

fn main() {
    // Shared state: a queue and a condition variable
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let not_empty = Arc::new(Condvar::new());

    // Consumer thread
    let queue_c = Arc::clone(&queue);
    let not_empty_c = Arc::clone(&not_empty);
    let consumer = thread::spawn(move || {
        let mut q = queue_c.lock().expect("Consumer mutex poisoned");
        loop {
            // Wait while the queue is empty
            while q.is_empty() {
                // Wait releases the lock and blocks until notified.
                // When woken, it re-acquires the lock.
                // The loop is necessary to handle spurious wakeups.
                q = not_empty_c.wait(q).expect("Consumer condvar poisoned");
            }
            // Queue is not empty, pop an element
            let item = q.pop_front().unwrap();
            println!("Consumed: {}", item);

            if item == "stop" {
                break;
            }
        }
    });

    // Producer thread
    let queue_p = Arc::clone(&queue);
    let not_empty_p = Arc::clone(&not_empty);
    let producer = thread::spawn(move || {
        let items = vec!["data1", "data2", "data3", "stop"];
        for item in items {
            let mut q = queue_p.lock().expect("Producer mutex poisoned");
            q.push_back(item);
            println!("Produced: {}", item