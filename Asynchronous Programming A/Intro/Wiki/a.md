**PART 0 — 30-SECOND EXECUTIVE SUMMARY**  
Asynchronous programming lets a program **start a long-running task** (I/O, network, timer, etc.) **and keep the CPU busy with other work** while the task completes in the background.  
The core pattern is:  
1. **Initiate** an operation **non-blocking**.  
2. **Register** a callback / future / promise that will run **later**.  
3. **Yield** control back to the event loop / runtime.  
4. **Resume** the original code path when the result is ready.  

---

## 1. **FOUNDATIONS – WHAT “ASYNC” REALLY MEANS**

| Term                     | Blocking version                  | Async version                           |
|--------------------------|-----------------------------------|-----------------------------------------|
| Read from disk           | `read(fd, buf, len)`              | `aio_read` / `ReadFileEx` / `tokio::fs` |
| HTTP request             | `curl_easy_perform`               | `reqwest::get` / `aiohttp`              |
| Sleep                    | `sleep(3)`                        | `tokio::time::sleep` / `asyncio.sleep`  |

Key distinction: **blocking** freezes the **entire OS thread**; **async** only parks the **logical task**.

---

## 2. **CORE ABSTRACTIONS**

| Concept         | Native Primitive | Rust Future | Go Equivalent | C++20 Coroutine |
|-----------------|------------------|-------------|---------------|-----------------|
| Unit of work    | `struct task`    | `Future`    | `goroutine`   | `task<>`        |
| Poller          | `select()`       | `Waker`     | `runtime`     | `co_await`      |
| Result carrier  | `errno` / `buf`  | `Poll<T>`   | `chan T`      | `awaitable<T>`  |

---

## 3. **EVENT LOOP & RUNTIME**

A **runtime** owns:
- **Task queue** (ready-to-run futures / goroutines).  
- **Reactor** (waits on epoll/kqueue/wepoll/IOCP).  
- **Timer wheel** (wake tasks at deadline).  

---

## 4. **CONTROL FLOW PATTERNS**

### 4.1 **Single Async Operation**

#### Rust (tokio)
```rust
#[tokio::main]
async fn main() {
    let content = tokio::fs::read_to_string("Cargo.toml").await.unwrap();
    println!("{content}");
}
```

#### Go
```go
package main
import (
    "fmt"
    "io/ioutil"
    "time"
)
func main() {
    go func() {
        b, _ := ioutil.ReadFile("go.mod")
        fmt.Println(string(b))
    }()
    time.Sleep(time.Second) // crude wait
}
```

#### C++20
```cpp
#include <iostream>
#include <fstream>
#include <experimental/task>

std::experimental::task<std::string> read_file(std::string path) {
    std::ifstream f{path};
    std::stringstream buf;
    buf << f.rdbuf();
    co_return buf.str();
}

int main() {
    auto t = read_file("CMakeLists.txt");
    std::cout << t.get();
}
```

---

### 4.2 **Concurrent Launch & Join**

| Pattern  | Rust (tokio) | Go | C++20 |
|----------|--------------|----|-------|
| **spawn + join** | `tokio::spawn` returns `JoinHandle<T>` | `go f()` + `sync.WaitGroup` | `co_spawn` + `awaitable<T>` |
| **select!** | `tokio::select!` | `select` statement | `co_await when_any` |

#### Rust – **concurrent** HTTP calls
```rust
use tokio::time::{sleep, Duration};

async fn fetch(url: &str) -> String {
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

#[tokio::main]
async fn main() {
    let t1 = tokio::spawn(fetch("https://httpbin.org/delay/1"));
    let t2 = tokio::spawn(fetch("https://httpbin.org/delay/2"));

    let (r1, r2) = tokio::join!(t1, t2);
    println!("{} {}", r1.unwrap().len(), r2.unwrap().len());
}
```

#### Go – **fan-in** with channels
```go
func fetch(url string, ch chan<- string) {
    resp, _ := http.Get(url)
    b, _ := io.ReadAll(resp.Body)
    ch <- string(b)
}

func main() {
    ch := make(chan string, 2)
    go fetch("https://httpbin.org/delay/1", ch)
    go fetch("https://httpbin.org/delay/2", ch)
    fmt.Println(<-ch, <-ch)
}
```

---

## 5. **EDGE CASES & PITFALLS**

| Problem | Symptom | Fix |
|---------|---------|-----|
| **Blocking the runtime** | 100 % CPU but no progress | Never call `std::thread::sleep` inside async; use `tokio::time::sleep`. |
| **Unbounded channel** | Memory leak / OOM | Use `tokio::sync::mpsc::channel(cap)`. |
| **Cancel safety** | Task keeps running after drop | Wrap in `tokio::select! { _ = &mut task => … }`. |
| **Pin requirement** | `Future` cannot be moved | `Pin<Box<dyn Future>>` or `tokio::pin!`. |
| **Stack overflow** | Deep recursion | Use `futures::stream::try_unfold`. |

---

## 6. **BACKPRESSURE & FLOW CONTROL**

| Technique | Rust | Go |
|-----------|------|----|
| Semaphore | `tokio::sync::Semaphore` | `golang.org/x/sync/semaphore` |
| Rate limiter | `governor` crate | `golang.org/x/time/rate` |
| Bounded channel | `mpsc::channel(n)` | `make(chan T, n)` |

---

## 7. **CANCELLATION PROPAGATION**

Rust leverages **drop**; Go leverages **context**.

```rust
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let child = token.child_token();
    let h = tokio::spawn(async move {
        tokio::select! {
            _ = child.cancelled() => println!("cancelled"),
            _ = tokio::time::sleep(Duration::from_secs(10)) => println!("done"),
        }
    });
    token.cancel();
    h.await.unwrap();
}
```

```go
ctx, cancel := context.WithCancel(context.Background())
go func() {
    select {
    case <-ctx.Done():
        fmt.Println("cancelled")
    case <-time.After(10 * time.Second):
        fmt.Println("done")
    }
}()
cancel()
```

---

## 8. **CPU-BOUND WORK OFFLOAD**

| Language | Primitive | Example |
|----------|-----------|---------|
| Rust | `tokio::task::spawn_blocking` | `spawn_blocking(|| heavy_hash(data))` |
| Go | `runtime.GOMAXPROCS` | goroutines automatically multiplexed |
| C++20 | `co_spawn(io_context, asio::thread_pool)` | `co_await asio::post(pool, use_awaitable)` |

---

## 9. **DEBUGGING & OBSERVABILITY**

| Tool | Rust | Go | C++ |
|------|------|----|-----|
| **Trace** | `tokio-console`, `tracing` | `go tool trace` | `co_await` + ASAN |
| **Metrics** | `metrics` crate + Prometheus | `expvar` + Prometheus | custom counters |
| **Stack dump** | `tokio::task::dump` | `GODEBUG=gctrace=1` | `gdb` + libstdc++ |

---

## 10. **PERFORMANCE COMPARISON (1 Mio Tasks)**

| Metric | Rust/tokio | Go | C++20 |
|--------|------------|----|-------|
| Memory per task | ≈ 1 kB | ≈ 2 kB | ≈ 1.2 kB |
| Context switch | 17 ns | 200 ns | 30 ns |
| Syscalls | 1 epoll per batch | 1 epoll per batch | 1 epoll per batch |

---

## 11. **CHEAT-SHEET — CONVERTING BLOCKING → ASYNC**

| Blocking | Async Rust | Async Go | Async C++ |
|----------|------------|----------|-----------|
| `std::fs::read` | `tokio::fs::read` | `io/ioutil.ReadFile` wrapped in goroutine | `co_await async_read` |
| `std::thread::sleep` | `tokio::time::sleep` | `time.Sleep` in goroutine | `co_await async_sleep` |
| `std::sync::Mutex` | `tokio::sync::Mutex` | `sync.Mutex` | `std::mutex` (still blocking!) → use `asio::strand` |

---

## 12. **MINI CAPSTONE EXAMPLE — CONCURRENT WEB CRAWLER**

```rust
use tokio::{fs, sync::Semaphore};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sem = std::sync::Arc::new(Semaphore::new(20)); // max 20 in flight
    let root = Url::parse("https://example.com")?;
    crawl(&root, sem).await?;
    Ok(())
}

async fn crawl(url: &Url, sem: Arc<Semaphore>) -> Result<(), Box<dyn std::error::Error>> {
    let _permit = sem.acquire().await.unwrap();
    let body = reqwest::get(url.clone()).await?.text().await?;
    fs::write(format!("{}.html", url.domain().unwrap()), &body).await?;
    // extract links & spawn more tasks...
    Ok(())
}
```

---

**Mastery checklist**  
- [ ] Understand **Future vs Promise vs async/await** syntax sugar.  
- [ ] Can spawn **100 k tasks** without blocking the runtime.  
- [ ] Implement **cancellation propagation** across multiple layers.  
- [ ] Measure **latency & throughput** under load.  
- [ ] Diagnose **deadlocks, leaks, starvation** via debugger/trace.