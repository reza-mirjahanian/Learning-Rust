Concurrency in Rust – Technical Reference Guide  
===============================================  

This document covers Rust’s concurrency model from basics to advanced topics. Each section includes:  
- Feature description  
- Memory‐layout/implementation notes  
- Code examples  
- Attributes/modifiers, visibility & scoping  
- Gotchas, tips & edge cases  
- Comparison tables where relevant  

Table of Contents  
-----------------  
1. Fundamentals  
  1.1 Ownership, Borrowing & Data Races  
  1.2 Marker Traits: Send & Sync  
2. Native Threads (`std::thread`)  
  2.1 Spawning & Joining  
  2.2 `std::thread::Builder` options  
  2.3 Detach & Named Threads  
  2.4 Stack size & Thread‐local storage  
3. Shared‐State Concurrency  
  3.1 `Mutex<T>`  
  3.2 `RwLock<T>`  
  3.3 `Condvar`  
  3.4 `Arc<T>` & Reference Counting  
4. Message Passing  
  4.1 `std::sync::mpsc` channels  
  4.2 Bounded vs Unbounded  
  4.3 `crossbeam::channel`  
5. Atomics & Memory Ordering  
  5.1 `AtomicBool`/`AtomicUsize`/…  
  5.2 Orderings: Relaxed, Acquire, Release, AcqRel, SeqCst  
  5.3 Fences  
6. Unsafe & Low‐Level Concurrency  
  6.1 Implementing `Send`/`Sync`  
  6.2 Raw Pointers & Data Races  
  6.3 `unsafe` blocks  
7. Asynchronous Concurrency (`async`/`await`)  
  7.1 `Future` trait & state machine  
  7.2 Executors (`tokio`, `async-std`)  
  7.3 `Waker`, `Context`, `Pin`, `Unpin`  
  7.4 `spawn`, `spawn_local`, `LocalSet`  
8. Advanced Topics  
  8.1 Lock‐Free Data Structures & `crossbeam`  
  8.2 Real‐Time & `no_std` environments  
  8.3 Thread Affinity & CPU‐pinning  
9. Comparison with Other Languages  

---  

1. Fundamentals  
---------------  

1.1 Ownership, Borrowing & Data Races  
*Rust guarantees data‐race freedom at compile time.* A _data race_ requires three conditions simultaneously:  
  • Two or more threads access the same memory location  
  • At least one access is a write  
  • No synchronization to prevent simultaneous access  

Rust’s _ownership_ and _borrowing_ rules disallow aliased mutable references. Shared immutable references (`&T`) can exist but only if `T: Sync`. Mutable references (`&mut T`) require unique ownership, excluding races.  

1.2 Marker Traits: Send & Sync  
  • `Send` means ownership of a type can be transferred between threads.  
  • `Sync` means `&T` is safe to share between threads.  

By default:  
  • Primitive types, `Arc<T>`, atomics: `Send + Sync`  
  • `Rc<T>`: neither `Send` nor `Sync`  
  • `MutexGuard<T>`: `!Send` if it holds `&mut T`  
  • Raw pointers: neither  

Implementation note:  
```rust
// declaration in std:
unsafe auto trait Send { }    // all types are Send unless opt‐out
unsafe auto trait Sync { }    // all types &T are Sync unless opt‐out
// opt‐out via negative impl:
impl !Sync for *const T {}
```
Edge cases & tips:  
  - Watch closures capturing non-`Send` data when passing to `thread::spawn`.  
  - Manually impl `Send`/`Sync` only if you ensure invariants in unsafe code.  

---  

2. Native Threads (`std::thread`)  
---------------------------------  

2.1 Spawning & Joining  
```rust
use std::thread;
let handle: thread::JoinHandle<u32> = thread::spawn(|| {
    // moves owned data into new OS thread
    compute_something()
});
let result: u32 = handle.join().expect("Thread panicked");
```
Visibility & scoping: captured variables move by default; use `move` closures.  

2.2 `std::thread::Builder` options  
```rust
use std::thread;
let builder = thread::Builder::new()
    .name("worker".into())
    .stack_size(4 * 1024 * 1024); // 4MiB
let handle = builder.spawn(|| {
    // …
}).unwrap();
```
Attributes/Modifiers:  
  #[thread_local] for defining thread‐local statics:  
```rust
#[thread_local]
static mut TLS_DATA: u32 = 0;
```
Memory layout: `JoinHandle<T>` owns a pointer to the thread’s stack and return slot.  

2.3 Detach & Named Threads  
  • `handle.join()` waits; dropping handle detaches thread.  
  • `.name()` sets OS‐level thread name (visible in debuggers).  

2.4 Stack size & Thread‐local storage  
  • Default stack size ~2 MiB (platform dependent).  
  • `#[thread_local]` static must be `Sync` or accessed only via main thread.  
  • In `no_std`, `thread::spawn` unavailable—use RTOS APIs or `bare-metal` crates.  

Gotcha: panicking thread poisons borrowed mutex guards.  

---  

3. Shared‐State Concurrency  
----------------------------  

3.1 `Mutex<T>`  
  • API: `lock() -> LockResult<MutexGuard<T>>`  
  • Poisoning: if thread panics while holding, future `lock()` returns `Err(PoisonError)`.  
  • Fairness: generally FIFO on Unix, unspecified on Windows.  
  • Memory layout (linux/pthreads):  
```text
struct Mutex<T> {
    inner: RawMutex,  // OS handle + lock state bits
    data: UnsafeCell<T>,
}
```
Example:  
```rust
use std::sync::{Arc, Mutex};
let m = Arc::new(Mutex::new(0));
let m2 = m.clone();
let t = thread::spawn(move || {
    let mut guard = m2.lock().unwrap();
    *guard += 1;
});
t.join().unwrap();
assert_eq!(*m.lock().unwrap(), 1);
```
Edge cases:  
  • Deadlock if two mutexes locked in different order.  
  • Recursive locking: `Mutex` is non‐recursive; leads to deadlock.  
Modifiers & attrs: none specific.  

3.2 `RwLock<T>`  
  • API: `read() -> RwLockReadGuard<T>`, `write() -> RwLockWriteGuard<T>`.  
  • Multiple readers allowed, one writer exclusive.  
  • Writer starvation possible under heavy read load.  
Memory layout: similar to `Mutex` but tracks reader count (`u32`) + writer bit.  

3.3 `Condvar`  
```rust
let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = pair.clone();
let t = thread::spawn(move || {
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_one();
});
let (lock, cvar) = &*pair;
let mut started = lock.lock().unwrap();
while !*started {
    started = cvar.wait(started).unwrap();
}
```
Spurious wakeups: always use in a loop.  

3.4 `Arc<T>` & Reference Counting  
  • Thread-safe RC: atomic refcounts.  
  • Memory layout:  
```text
struct ArcInner<T> {
    strong: AtomicUsize,
    weak: AtomicUsize,
    data: T,
}
```
  • `Arc::downgrade`, `Weak<T>`  
Gotchas: cycles → memory leaks.  

Trade‐offs:  
| Primitive    | Contentious? | Reader/Writer fairness | Poisoning |  
|--------------|--------------|------------------------|-----------|  
| Mutex<T>     | Yes          | N/A                    | Yes       |  
| RwLock<T>    | Readers only | Unfair (writer starve)  | Yes       |  

---  

4. Message Passing  
------------------  

4.1 `std::sync::mpsc` channels  
```rust
use std::sync::mpsc;
let (tx, rx) = mpsc::channel::<i32>();
thread::spawn(move || {
    tx.send(42).unwrap();
});
assert_eq!(rx.recv().unwrap(), 42);
```
  • Multi‐producer, single‐consumer.  
  • `try_recv()`, `recv_timeout()`.  

4.2 Bounded vs Unbounded  
```rust
let (tx, rx) = mpsc::sync_channel(0); // zero‐capacity (rendezvous)
tx.send(1)?; // blocks until recv
```

4.3 `crossbeam::channel`  
  • MPMC (multi‐producer, multi‐consumer)  
  • `select!` macro for pattern‐matching on multiple channels  
  • Higher throughput, no poisoning  

Example:  
```rust
use crossbeam::channel::unbounded;
let (s1, r1) = unbounded();
let (s2, r2) = unbounded();
crossbeam::select! {
    recv(r1) -> msg => println!("Got {:?}", msg),
    recv(r2) -> msg => println!("Got {:?}", msg),
}
```
Memory & performance: uses intrusive linked lists + atomics.  

Trade‐off table:  
| Channel     | Prod/Cons | Bounded | Blocking Sends | Poisoning | Throughput |  
|-------------|-----------|---------|----------------|-----------|------------|  
| std mpsc    | MPSC      | Yes     | Yes            | No        | Low        |  
| crossbeam   | MPMC      | Yes     | Yes            | No        | High       |  

---  

5. Atomics & Memory Ordering  
-----------------------------  

5.1 Atomic Types  
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
let a = AtomicUsize::new(0);
a.store(1, Ordering::Release);
let v = a.load(Ordering::Acquire);
```
Types: `AtomicBool`, `AtomicIsize`, `AtomicPtr<T>`, …  

5.2 Orderings  
  • Relaxed: only atomicity, no ordering.  
  • Acquire: prevents reordering after load.  
  • Release: prevents reordering before store.  
  • AcqRel: both.  
  • SeqCst: global total order.  

Example: release/acquire fence pattern:  
```rust
// writer
data.store(val, Ordering::Relaxed);
flag.store(1, Ordering::Release);
// reader
while flag.load(Ordering::Acquire) == 0 { }
assert_eq!(data.load(Ordering::Relaxed), val);
```

5.3 Fences  
```rust
use std::sync::atomic::{fence, Ordering};
fence(Ordering::SeqCst);
```

Implementation: maps to `LOCK` prefix on x86 or `dmb` on ARM.  

Gotchas:  
  - `Relaxed` can reorder; be careful.  
  - `compare_exchange_weak` may spuriously fail.  

---  

6. Unsafe & Low‐Level Concurrency  
---------------------------------  

6.1 Implementing `Send`/`Sync`  
```rust
unsafe impl Send for MyType {}      // only if all fields safe to send
unsafe impl Sync for MyType {}      // only if &MyType is thread‐safe
```
Ensure no data races; use proper atomics or locks.  

6.2 Raw Pointers & Data Races  
  • `*mut T` allows aliasing mutable.  
  • Unsafe code must ensure exclusive access or atomic ops.  

6.3 `unsafe` blocks  
  • In `unsafe` you may invoke race‐prone APIs, but must uphold invariants manually.  

---  

7. Asynchronous Concurrency (`async`/`await`)  
----------------------------------------------  

7.1 `Future` trait & state machine  
```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```
Each `async fn` desugars to a state machine struct with pinned fields.  

7.2 Executors (`tokio`, `async-std`)  
  • `spawn(fut)` requires `Fut: Send + 'static` on multithreaded reactor.  
  • `LocalSet` for `!Send` futures on single‐threaded runtime.  

7.3 `Waker`, `Context`, `Pin`, `Unpin`  
  • `Waker::wake()` enqueues task.  
  • `Pin<P>` prohibits moving.  
  • `Unpin` auto‐trait unless struct contains `!Unpin` fields (e.g. futures).  

7.4 `spawn`, `spawn_local`, `LocalSet`  
```rust
// Tokio example
tokio::spawn(async { /* must be Send */ });
// Local task
tokio::task::spawn_local(async { /* !Send ok */ });
```
Edge cases:  
  - Holding a `MutexGuard` across .await may deadlock.  
  - Use `futures::lock::Mutex` or scope lock outside `async`.  

---  

8. Advanced Topics  
------------------  

8.1 Lock‐Free Structures & `crossbeam`  
  • `SegQueue<T>`, `TreiberStack<T>`, epoch GC (`crossbeam_epoch`).  
  • Memory layout uses atomic pointers + hazard pointers.  

8.2 Real‐Time & `no_std`  
  • On embedded, use `cortex_m::interrupt::free` for critical sections.  
  • `RTIC` framework for pure‐Rust real‐time concurrency.  

8.3 Thread Affinity & CPU‐pinning  
  • `libc::sched_setaffinity` on Linux.  
  • `thread_priority` crate for setting priorities.  

---  

9. Comparison with Other Languages  
----------------------------------  

| Concept         | Rust                          | Go                          | C++ (std)              | Java                     |  
|-----------------|-------------------------------|-----------------------------|------------------------|--------------------------|  
| Native threads  | OS threads (`std::thread`)    | M:N on runtime (goroutines)| OS threads             | OS threads               |  
| Message‐passing | `mpsc`, `crossbeam`          | built‐in `chan`            | none (custom)          | none (custom libs)       |  
| Shared‐state    | `Mutex`, `RwLock`, `Arc`      | `sync.Mutex` (rare)        | `std::mutex`, `std::shared_mutex` | `synchronized`, `ReentrantLock` |  
| Async/await     | zero‐cost, state machine      | goroutines + channels      | `std::future` (C++20)  | `CompletableFuture`      |  
| Data races      | compile‐time eliminated       | possible at runtime        | possible               | possible                 |  

Trade‐off summary:  
- Rust’s ownership + types enforce safe concurrency with zero runtime cost.  
- Go’s goroutines easy but rely on garbage collection + runtime scheduler.  
- C++ offers similar features but lacks compile‐time data‐race checks.  
- Java has mature concurrency libs but GC pauses can impact real‐time.  

