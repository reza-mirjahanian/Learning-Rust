 # Threads in Rust

## Introduction to Rust Threads

Threads in Rust provide a way to execute code concurrently. Rust's standard library offers tools for creating and managing threads through the `std::thread` module.

## Basic Thread Creation

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        println!("Hello from the spawned thread!");
        thread::sleep(Duration::from_millis(1000));
        println!("Spawned thread finished!");
    });
    
    println!("Hello from the main thread!");
    
    // Wait for the spawned thread to finish
    handle.join().unwrap();
    
    println!("Main thread finished!");
}
```

The `thread::spawn` function takes a closure and returns a `JoinHandle`, which can be used to wait for the thread to finish using the `join` method.

## Thread::Builder

For more control over thread creation:

```rust
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("custom-thread".to_string())
        .stack_size(4 * 1024 * 1024); // 4MB stack
    
    let handle = builder.spawn(|| {
        println!("Running in a thread with name: {:?}", thread::current().name());
    }).unwrap();
    
    handle.join().unwrap();
}
```

## Sharing Data Between Threads

### Moving Ownership

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("Vector in spawned thread: {:?}", v);
    });
    
    // v is no longer accessible here as ownership was moved to the spawned thread
    // println!("Vector in main thread: {:?}", v); // This would cause a compile error
    
    handle.join().unwrap();
}
```

### Using Message Passing with Channels

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send("Hello from the thread!").unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
```

### Multiple Producers

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    for i in 0..5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(i).unwrap();
        });
    }
    
    // Drop the original sender
    drop(tx);
    
    // Receive all messages
    while let Ok(received) = rx.recv() {
        println!("Received: {}", received);
    }
}
```

### Multiple Values

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![1, 2, 3, 4, 5];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}
```

## Shared State Concurrency

### Using Mutex for Synchronization

```rust
use std::thread;
use std::sync::{Mutex, Arc};

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

### Using RwLock for Read/Write Access

```rust
use std::thread;
use std::sync::{RwLock, Arc};

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // Spawn reader threads
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let values = data.read().unwrap();
            println!("Reader {}: {:?}", i, *values);
        }));
    }
    
    // Spawn writer thread
    {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut values = data.write().unwrap();
            values.push(4);
            println!("Writer: {:?}", *values);
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Thread-Local Storage

```rust
use std::thread;
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
}

fn main() {
    COUNTER.with(|c| {
        *c.borrow_mut() += 1;
        println!("Main thread counter value: {}", *c.borrow());
    });
    
    let handle = thread::spawn(|| {
        COUNTER.with(|c| {
            *c.borrow_mut() += 1;
            println!("Spawned thread counter value: {}", *c.borrow());
        });
    });
    
    handle.join().unwrap();
    
    COUNTER.with(|c| {
        println!("Final main thread counter value: {}", *c.borrow());
    });
}
```

## Thread Parking

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let parked_thread = thread::spawn(|| {
        println!("Thread will park");
        thread::park();
        println!("Thread unparked!");
    });
    
    // Give the thread time to park
    thread::sleep(Duration::from_millis(500));
    
    println!("Unparking the thread");
    parked_thread.thread().unpark();
    
    parked_thread.join().unwrap();
}
```

## Thread Synchronization with Barriers

```rust
use std::thread;
use std::sync::{Arc, Barrier};
use std::time::Duration;

fn main() {
    let mut handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5));
    
    for i in 0..5 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Thread {} started", i);
            thread::sleep(Duration::from_millis(i * 200));
            println!("Thread {} waiting at barrier", i);
            
            b.wait();
            
            println!("Thread {} passed barrier", i);
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Thread-Safe Reference Counting with Arc

```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            println!("Thread {}: data[0] = {}", i, data[0]);
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Scoped Threads (Rust 1.63+)

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    thread::scope(|s| {
        // We can borrow `v` here because the scoped thread is guaranteed
        // to terminate before `thread::scope` returns
        s.spawn(|| {
            println!("Vector in scoped thread: {:?}", v);
        });
        
        s.spawn(|| {
            println!("Also accessing vector: {:?}", v);
        });
        
        // No need to call join() on the spawned threads,
        // it happens automatically when the scope ends
    });
    
    // v is still accessible here
    println!("Vector in main thread: {:?}", v);
}
```

## Comparing Different Synchronization Primitives

| Primitive | Thread-Safety | Use Case | Pros | Cons |
|-----------|---------------|----------|------|------|
| `Mutex<T>` | ✓ | Exclusive access | Simple to use | Potential deadlocks, blocking |
| `RwLock<T>` | ✓ | Read-heavy workloads | Multiple readers | Potential writer starvation |
| `mpsc::channel` | ✓ | Producer-consumer patterns | No shared state | Limited to one-way communication |
| `Arc<T>` | ✓ | Shared immutable data | No locking overhead | Can't modify data without interior mutability |
| `Barrier` | ✓ | Synchronization points | Coordinate multiple threads | All threads must reach barrier |
| `Once` | ✓ | One-time initialization | Thread-safe singletons | Can only execute once |
| `Condvar` | ✓ | Waiting for conditions | Efficient waiting | Complex to use correctly |

## Thread Pools with Rayon

```rust
use rayon::prelude::*;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Parallel iteration
    let sum: i32 = v.par_iter().sum();
    println!("Sum: {}", sum);
    
    // Parallel map
    let squares: Vec<_> = v.par_iter()
                           .map(|&x| x * x)
                           .collect();
    println!("Squares: {:?}", squares);
}
```

## Thread Safety and Send/Sync Traits

Rust's type system ensures thread safety through two marker traits:

- `Send`: Types that can be transferred across thread boundaries
- `Sync`: Types that can be shared between threads (i.e., `&T` is `Send`)

```rust
use std::thread;
use std::rc::Rc; // Not thread-safe
use std::sync::Arc; // Thread-safe

fn main() {
    let rc = Rc::new(5);
    
    // This would fail to compile because Rc is not Send
    // let handle = thread::spawn(move || {
    //     println!("rc in thread: {}", rc);
    // });
    
    let arc = Arc::new(5);
    
    // This works because Arc is Send
    let handle = thread::spawn(move || {
        println!("arc in thread: {}", arc);
    });
    
    handle.join().unwrap();
}
```

## Performance Considerations

### Thread Creation Overhead

Thread creation has significant overhead. For many short-lived tasks, a thread pool is more efficient:

```rust
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);
    
    let (tx, rx) = channel();
    
    for i in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(i).expect("channel will be there waiting for the pool");
        });
    }
    
    drop(tx);
    
    for result in rx.iter() {
        println!("Job {} finished", result);
    }
}
```

### Thread Stack Size

Default thread stack size varies by platform. You can customize it:

```rust
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .stack_size(32 * 1024 * 1024); // 32MB stack
    
    let handle = builder.spawn(|| {
        // Deep recursion or large local variables
        let large_array = [0u8; 10 * 1024 * 1024]; // 10MB array
        println!("Array size: {} bytes", large_array.len());
    }).unwrap();
    
    handle.join().unwrap();
}
```

## Time Complexity Analysis

| Operation | Time Complexity | Space Complexity |
|-----------|-----------------|------------------|
| Thread creation | $O(1)$ | $O(1)$ + stack size |
| Thread join | $O(1)$ | $O(1)$ |
| Mutex lock/unlock | $O(1)$ amortized | $O(1)$ |
| RwLock read lock | $O(1)$ amortized | $O(1)$ |
| RwLock write lock | $O(n)$ where n is readers | $O(1)$ |
| Channel send | $O(1)$ | $O(1)$ |
| Channel receive | $O(1)$ | $O(1)$ |
| Arc clone | $O(1)$ | $O(1)$ |

## Error Handling with Threads

```rust
use std::thread;
use std::sync::mpsc;
use std::panic;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let handle = thread::spawn(move || {
        // This will panic
        panic!("Thread panicking!");
        
        // This code will never run
        tx.send("Message after panic").unwrap();
    });
    
    // This will contain the panic info
    let result = handle.join();
    
    match result {
        Ok(_) => println!("Thread completed successfully"),
        Err(e) => println!("Thread panicked: {:?}", e),
    }
    
    // Alternative way to catch panics
    let handle = thread::spawn(|| {
        let result = panic::catch_unwind(|| {
            panic!("Panic inside catch_unwind");
        });
        
        println!("Caught panic: {:?}", result);
        
        // Continue execution
        "Thread result"
    });
    
    println!("Thread returned: {:?}", handle.join().unwrap());
}
```

## Advanced Thread Communication Patterns

### Bounded Channels

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // Create a bounded channel with a capacity of 3
    let (tx, rx) = mpsc::sync_channel(3);
    
    thread::spawn(move || {
        for i in 0..10 {
            println!("Sending {}", i);
            tx.send(i).unwrap();
            println!("Sent {}", i);
        }
    });
    
    thread::sleep(Duration::from_secs(1));
    
    for received in rx {
        println!("Received {}", received);
        thread::sleep(Duration::from_millis(500));
    }
}
```

### Select-like Behavior with Multiple Channels

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx1.send("Message from channel 1").unwrap();
    });
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        tx2.send("Message from channel 2").unwrap();
    });
    
    // Poor man's select - poll multiple channels
    loop {
        let mut received = false;
        
        if let Ok(msg) = rx1.try_recv() {
            println!("Received on channel 1: {}", msg);
            received = true;
        }
        
        if let Ok(msg) = rx2.try_recv() {
            println!("Received on channel 2: {}", msg);
            received = true;
        }
        
        if !received {
            thread::sleep(Duration::from_millis(100));
        }
        
        // Exit when both channels are empty and senders are dropped
        if rx1.try_recv().is_err() && rx2.try_recv().is_err() {
            let is_disconnected1 = rx1.recv().is_err();
            let is_disconnected2 = rx2.recv().is_err();
            
            if is_disconnected1 && is_disconnected2 {
                break;
            }
        }
    }
}
```

## Thread-Safety Design Patterns

### Mutex Guards and Avoiding Deadlocks

```rust
use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));
    
    // Potential deadlock
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);
    let thread1 = thread::spawn(move || {
        let _guard1 = m1.lock().unwrap();
        println!("Thread 1: Locked mutex1");
        
        // Sleep to increase chance of deadlock
        thread::sleep(Duration::from_millis(100));
        
        let _guard2 = m2.lock().unwrap();
        println!("Thread 1: Locked mutex2");
    });
    
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);
    let thread2 = thread::spawn(move || {
        // Better practice: always lock in the same order
        let _guard1 = m1.lock().unwrap();
        println!("Thread 2: Locked mutex1");
        
        let _guard2 = m2.lock().unwrap();
        println!("Thread 2: Locked mutex2");
    });
    
    thread1.join().unwrap();
    thread2.join().unwrap();
}
```

### Actor Model with Crossbeam Channels

```rust
use std::thread;
use crossbeam_channel::{unbounded, Sender, Receiver};

enum Message {
    Increment,
    Decrement,
    GetValue(Sender<i32>),
    Exit,
}

struct Counter {
    value: i32,
    receiver: Receiver<Message>,
}

impl Counter {
    fn new(receiver: Receiver<Message>) -> Self {
        Counter {
            value: 0,
            receiver,
        }
    }
    
    fn run(&mut self) {
        loop {
            match self.receiver.recv().unwrap() {
                Message::Increment => {
                    self.value += 1;
                    println!("Incremented to {}", self.value);
                }
                Message::Decrement => {
                    self.value -= 1;
                    println!("Decremented to {}", self.value);
                }
                Message::GetValue(response) => {
                    response.send(self.value).unwrap();
                }
                Message::Exit => {
                    println!("Exiting counter thread");
                    break;
                }
            }
        }
    }
}

fn main() {
    let (s, r) = unbounded();
    
    let counter_thread = thread::spawn(move || {
        let mut counter = Counter::new(r);
        counter.run();
    });
    
    s.send(Message::Increment).unwrap();
    s.send(Message::Increment).unwrap();
    s.send(Message::Decrement).unwrap();
    
    let (response_s, response_r) = unbounded();
    s.send(Message::GetValue(response_s)).unwrap();
    
    let value = response_r.recv().unwrap();
    println!("Current value: {}", value);
    
    s.send(Message::Exit).unwrap();
    counter_thread.join().unwrap();
}
```

## Common Pitfalls and Solutions

### Holding a Mutex Across await Points

```rust
use std::sync::{Mutex, Arc};
use tokio::time::{sleep, Duration};

async fn process_data_problematic(data: Arc<Mutex<Vec<i32>>>) {
    // PROBLEM: Holding mutex across .await point
    let mut locked_data = data.lock().unwrap();
    sleep(Duration::from_secs(1)).await; // Other tasks cannot access data during this time
    locked_data.push(42);
}

async fn process_data_proper(data: Arc<Mutex<Vec<i32>>>) {
    // Better approach: Get the lock, do work, release it
    {
        let mut locked_data = data.lock().unwrap();
        locked_data.push(42);
    } // Lock released here
    
    // Do async work after releasing the lock
    sleep(Duration::from_secs(1)).await;
}
```

### Leaking Thread Handles

```rust
use std::thread;
use std::time::Duration;

fn leak_thread() {
    // PROBLEM: Thread handle not stored, can't join later
    thread::spawn(|| {
        // This thread will keep running even if the function returns
        loop {
            println!("Leaked thread still running");
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn proper_thread_handling() -> thread::JoinHandle<()> {
    // Better approach: Return the handle so caller can join
    thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread running");
            thread::sleep(Duration::from_secs(1));
        }
    })
}

fn main() {
    leak_thread();
    println!("Leaked thread function returned");
    
    let handle = proper_thread_handling();
    println!("Got thread handle, waiting for it to finish");
    handle.join().unwrap();
    println!("Thread completed");
}
```

### Using Thread-Safe Types in Multi-Threaded Contexts

```rust
use std::thread;
use std::sync::{Arc, Mutex};
use std::cell::RefCell; // Not thread-safe
use std::rc::Rc; // Not thread-safe

fn main() {
    // PROBLEM: RefCell and Rc are not thread-safe
    let shared_data_not_safe = Rc::new(RefCell::new(vec![1, 2, 3]));
    
    // let data_clone = shared_data_not_safe.clone();
    // This would fail to compile:
    // thread::spawn(move || {
    //     data_clone.borrow_mut().push(4);
    // });
    
    // SOLUTION: Use thread-safe alternatives
    let shared_data_safe = Arc::new(Mutex::new(vec![1, 2, 3]));
    
    let data_clone = shared_data_safe.clone();
    let handle = thread::spawn(move || {
        data_clone.lock().unwrap().push(4);
    });
    
    handle.join().unwrap();
    
    println!("Final data: {:?}", *shared_data_safe.lock().unwrap());
}
```

## Comparison with Other Concurrency Models

| Feature | Rust Threads | Async Tasks | Rayon Work-Stealing |
|---------|-------------|-------------|-------------------|
| Overhead | Higher | Lower | Medium |
| Parallelism | True parallelism | Concurrent execution | True parallelism |
| Use Case | CPU-bound, I/O intensive | I/O-bound, many connections | Data parallelism |
| Memory Usage | Higher (separate stacks) | Lower (shared stack) | Medium |
| Creation Cost | Higher | Lower | Medium |
| Blocking | Can block | Should not block | Can block |
| Programming Model | Imperative | Async/await | Parallel iterators |

## CPU-Bound vs I/O-Bound Workloads

### CPU-Bound Example with Threads

```rust
use std::thread;
use std::time::Instant;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let start = Instant::now();
    
    // Sequential execution
    let count_seq = (1000000..1100000).filter(|&n| is_prime(n)).count();
    println!("Sequential: found {} primes in {:?}", count_seq, start.elapsed());
    
    // Parallel execution with threads
    let start = Instant::now();
    
    let num_threads = 4;
    let chunk_size = 25000;
    let mut handles = Vec::new();
    
    for i in 0..num_threads {
        let start_num = 1000000 + i * chunk_size;
        let end_num = start_num + chunk_size;
        
        handles.push(thread::spawn(move || {
            (start_num..end_num).filter(|&n| is_prime(n)).count()
        }));
    }
    
    let count_par: usize = handles.into_iter()
                                  .map(|h| h.join().unwrap())
                                  .sum();
    
    println!("Parallel: found {} primes in {:?}", count_par, start.elapsed());
}
```

### I/O-Bound Example with Async

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        
        // For each connection, spawn a new async task
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return, // Connection closed
                    Ok(n) => {
                        // Echo back the data
                        if let Err(e) = socket.write_all(&buf[0..n]).await {
                            eprintln!("Failed to write to socket: {}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        return;
                    }
                }
            }
        });
    }
}
```

## Thread Safety Guarantees in Rust

Rust's type system ensures thread safety through both compile-time checks and runtime support:

```rust
use std::thread;
use std::cell::Cell; // Not Sync

fn main() {
    let not_sync = Cell::new(42);
    
    // This won't compile because Cell is not Sync
    // thread::spawn(move || {
    //     not_sync.set(43);
    // });
    
    // Making a non-thread-safe type thread-safe with Mutex
    let thread_safe = std::sync::Mutex::new(Cell::new(42));
    
    let handle = thread::spawn(move || {
        thread_safe.lock().unwrap().set(43);
    });
    
    handle.join().unwrap();
}
```

