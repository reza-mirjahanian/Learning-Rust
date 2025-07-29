

---

## 🧠 **Core Concepts of Asynchronous Programming**

**Asynchronous programming** enables execution of tasks independently of the main flow — improving **performance**, **responsiveness**, and **resource utilization**, especially for **I/O-bound** or **network-bound** operations.

### 🔄 Key Concepts
- **Concurrency**: multiple tasks progressing at once.
- **Parallelism**: multiple tasks executing simultaneously.
- **Blocking vs Non-Blocking**:
  - **Blocking**: waits for task to complete.
  - **Non-blocking**: initiates task, continues executing other code.

### 🆚 Comparison Table

| Concept        | Synchronous               | Asynchronous                   |
|----------------|---------------------------|--------------------------------|
| Execution      | Sequential                | Event-driven / Concurrent      |
| Efficiency     | Lower (esp. I/O-bound)    | High (non-blocking I/O)        |
| Complexity     | Simple control flow       | Requires careful coordination  |
| Use Case       | Scripts, small utilities  | Web servers, networking        |

---

## ⚙️ **Go (Golang)**: Async via Goroutines & Channels

### ✅ Basics
Use **goroutines** (lightweight threads) and **channels** (communication primitives).

```go
package main

import (
    "fmt"
    "time"
)

func say(msg string) {
    for i := 0; i < 3; i++ {
        fmt.Println(msg)
        time.Sleep(100 * time.Millisecond)
    }
}

func main() {
    go say("async")
    say("sync")
}
```

### ☑️ Use Cases
- Web servers (handle thousands of requests)
- Parallel computations
- Streaming data

### 🧪 Edge Case: Deadlock

```go
func main() {
    ch := make(chan int)
    ch <- 1 // blocks forever, no receiver
}
```

✅ **Fix**: Add a goroutine to receive.

---

## 🦀 **Rust**: Async via `async` / `await` and `tokio`

### ✅ Basics

```rust
use tokio::time::{sleep, Duration};

async fn greet() {
    sleep(Duration::from_millis(500)).await;
    println!("Hello after delay");
}

#[tokio::main]
async fn main() {
    greet().await;
}
```

### ☑️ Use Cases
- High-performance servers
- File I/O or HTTP clients
- Game engines

### 🧪 Edge Case: Missing `.await`

```rust
async fn fetch_data() {
    // ...
}

fn main() {
    fetch_data(); // no .await, task not run
}
```

✅ **Fix**: `.await` is mandatory to execute an async block.

---

## 💻 **C++**: Async via `std::async`, `std::future`, `std::thread`

### ✅ Basics

```cpp
#include <iostream>
#include <future>

int compute() {
    return 42;
}

int main() {
    std::future<int> result = std::async(std::launch::async, compute);
    std::cout << "Result: " << result.get() << std::endl;
}
```

### ☑️ Use Cases
- Parallel algorithms
- GUI responsiveness
- Heavy calculations off main thread

### 🧪 Edge Case: Lazy Evaluation

```cpp
auto result = std::async(std::launch::deferred, compute);
// compute() only runs when get() is called
```

✅ **Fix**: Use `std::launch::async` for immediate execution.

---

## 🔍 **Comparisons with Similar Concepts**

| Feature         | Async Programming   | Multithreading      | Event Loop            |
|------------------|--------------------|----------------------|------------------------|
| Cost             | Lightweight         | Higher (OS threads)  | Minimal                |
| Use Case         | I/O-bound tasks     | CPU-bound tasks      | GUIs, Node.js servers  |
| Complexity       | Moderate            | High (synchronization)| Medium                 |

---

## 📚 Patterns & Design

### 🔹 Common Patterns
- **Async/Await**
- **Future/Promise**
- **Event Loop**
- **Message Passing**

### 🔸 Best Practices
- Avoid blocking in async code
- Handle errors gracefully (`Result`, `try/catch`, etc.)
- Use timeouts and cancellation mechanisms
- Prevent race conditions with synchronization (mutexes, channels)

---
