
---

# 🔹 1. Foundational Concepts

## **What Is a Future?**

A **`Future`** represents a **value that is not yet available**, but will be **computed asynchronously**.

### Trait Definition (simplified):

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

* **`Output`**: The result of the future once it's done.
* **`poll`**: Drives the computation. It returns:

  * `Poll::Pending` → not ready yet.
  * `Poll::Ready(val)` → computation finished.

### Analogy:

A `Future` is like ordering food. It's not ready now (`Pending`), but when it's served (`Ready`), you get your `Output`.

---

## **What Is `async` in Rust?**

* **`async fn`** creates a **future**.
* **`await`** polls the future until it completes.

```rust
async fn hello() -> String {
    "Hello, World!".to_string()
}

fn main() {
    let fut = hello(); // This is a Future
}
```

---

## **Minimal Runtime Example**

To run async code, you need an **executor**:

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::executor::block_on;

async fn say_hi() -> &'static str {
    "hi"
}

fn main() {
    let result = block_on(say_hi());
    println!("{}", result);
}
```

---

# 🔹 2. Under the Hood of `async`

## **`async fn` is sugar**

```rust
async fn foo() -> u32 {
    10
}
```

Is equivalent to:

```rust
fn foo() -> impl Future<Output = u32> {
    async { 10 }
}
```

---

## **`await` is sugar**

```rust
let x = future.await;
```

Is roughly:

```rust
loop {
    match Future::poll(Pin::new(&mut future), &mut context) {
        Poll::Ready(val) => break val,
        Poll::Pending => yield execution,
    }
}
```

---

# 🔹 3. `Future` Polling Model

## **Manual Future Example**

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture;

impl Future for MyFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready("Done!")
    }
}
```

---

# 🔹 4. Executors and Runtimes

Futures **do nothing** until **polled** by an **executor**.

| Executor    | Description                          |
| ----------- | ------------------------------------ |
| `block_on`  | Simple single-thread executor        |
| `tokio`     | Full-featured async runtime          |
| `async-std` | Async runtime with standard API feel |

---

## **Tokio Example**

```rust
#[tokio::main]
async fn main() {
    say().await;
}

async fn say() {
    println!("Hello from Tokio!");
}
```

---

# 🔹 5. Chaining Futures

## **`.then()` and `.map()`**

Using `.await`:

```rust
async fn double(x: u32) -> u32 {
    x * 2
}

#[tokio::main]
async fn main() {
    let result = double(10).await;
    println!("{}", result);
}
```

Chaining manually (with `futures` crate):

```rust
use futures::future::ready;

fn main() {
    let fut = ready(10).map(|x| x * 2);
    let result = futures::executor::block_on(fut);
    println!("{}", result);
}
```

---

# 🔹 6. Concurrency vs Parallelism

| Concept     | Definition                           | Rust Tool            |
| ----------- | ------------------------------------ | -------------------- |
| Concurrency | Doing many things at once (overlap)  | `join!`, `select!`   |
| Parallelism | Doing many things **simultaneously** | Threads, rayon, etc. |

## **Concurrent Futures with `join!`**

```rust
use tokio::join;

async fn a() -> u32 { 1 }
async fn b() -> u32 { 2 }

#[tokio::main]
async fn main() {
    let (x, y) = join!(a(), b());
    println!("{} + {} = {}", x, y, x + y);
}
```

## **Race with `select!`**

```rust
use tokio::select;

#[tokio::main]
async fn main() {
    select! {
        _ = async_fn1() => println!("First completed!"),
        _ = async_fn2() => println!("Second completed!"),
    }
}
```

---

# 🔹 7. async Trait Methods

Rust does **not** support `async` in traits directly yet (without nightly or hacks).

Use:

### ✅ Return `Pin<Box<dyn Future>>`

```rust
use std::future::Future;
use std::pin::Pin;

trait MyAsyncTrait {
    fn get_data(&self) -> Pin<Box<dyn Future<Output = String> + Send>>;
}

struct MyType;

impl MyAsyncTrait for MyType {
    fn get_data(&self) -> Pin<Box<dyn Future<Output = String> + Send>> {
        Box::pin(async { "Hello".to_string() })
    }
}
```

---

# 🔹 8. Error Handling with `Result<T, E>`

```rust
async fn may_fail(success: bool) -> Result<&'static str, &'static str> {
    if success {
        Ok("Success")
    } else {
        Err("Failed")
    }
}

#[tokio::main]
async fn main() {
    match may_fail(true).await {
        Ok(val) => println!("Got: {}", val),
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

---

# 🔹 9. Common Pitfalls and Edge Cases

## ❌ Blocking in Async Code

```rust
#[tokio::main]
async fn main() {
    std::thread::sleep(std::time::Duration::from_secs(1)); // ❌
}
```

Use non-blocking alternative:

```rust
#[tokio::main]
async fn main() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
```

---

## ❌ Holding Locks Across `.await`

```rust
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let lock = Mutex::new(0);

    let guard = lock.lock().await;
    // ❌ DON'T: await while holding the lock
    some_async_func().await;

    drop(guard);
}

async fn some_async_func() {}
```

---

# 🔹 10. Comparison Table

| Feature          | Synchronous Rust | Async Rust (`async`/`await`)       |
| ---------------- | ---------------- | ---------------------------------- |
| Function returns | Value            | Future                             |
| Execution        | Blocking         | Non-blocking                       |
| Threads          | OS threads       | Lightweight tasks                  |
| Parallel         | `std::thread`    | `tokio::spawn`, `join!`, `select!` |
| Error handling   | `Result<T, E>`   | `Result<T, E>` in `Future`         |
| Traits support   | Native           | Via `dyn Future` or `async-trait`  |

---

# 🔹 11. Testing Async Code

```rust
#[tokio::test]
async fn test_async() {
    assert_eq!(get_number().await, 42);
}

async fn get_number() -> u32 {
    42
}
```

---

# 🔹 12. async move

Captures **variables by move** into the future:

```rust
#[tokio::main]
async fn main() {
    let x = String::from("hello");
    let fut = async move {
        println!("{}", x); // x is moved into future
    };

    fut.await;
}
```

---



---

# 🔹 13. Anatomy of a `Future`: `Pin`, `Waker`, `Context`, `Poll`

## 🔸 `Poll`

The central building block of async is the **polling mechanism**.

```rust
enum Poll<T> {
    Ready(T),
    Pending,
}
```

* `Ready(val)`: value is available.
* `Pending`: still needs work. The executor should **schedule the future again** when it's ready.

---

## 🔸 `Context` and `Waker`

A **`Waker`** is used to tell the executor:

> “I’m ready to make progress — wake me up!”

```rust
pub struct Context<'a> {
    waker: &'a Waker,
}
```

### `Waker` has:

* `wake()`: notifies executor to poll again.
* It avoids busy-waiting.

### Custom Future Example with `Poll` and `Context`

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::time::sleep;

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready("Done")
        } else {
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let future = Delay { when: Instant::now() + Duration::from_secs(2) };
    println!("{}", future.await);
}
```

In practice, you wouldn’t implement delays like this; you’d use `tokio::time::sleep`, which uses a proper **waker** to notify the executor.

---

## 🔸 `Pin` Explained

**Problem:** Futures can **self-reference** (e.g., async blocks capture variables), and moving them can invalidate memory.

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Output>;
```

* `Pin` ensures the future’s memory **will not be moved** after being polled.

### Creating a pinned box manually:

```rust
use std::pin::Pin;
let fut = async { 42 };
let mut fut = Box::pin(fut); // type: Pin<Box<dyn Future<...>>>
```

---

# 🔹 14. How Async/Await Works Internally: Generator-Based Desugaring

Rust implements async/await using **generators** behind the scenes.

## Desugaring Example

```rust
async fn add_one(x: i32) -> i32 {
    x + 1
}
```

Is compiled to a **state machine** like:

```rust
enum State {
    Start,
    Done,
}

struct AddOneFuture {
    state: State,
    x: i32,
}

impl Future for AddOneFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<i32> {
        match self.state {
            State::Start => {
                self.state = State::Done;
                Poll::Ready(self.x + 1)
            }
            State::Done => panic!("polled after completion"),
        }
    }
}
```

* `await` translates to: “check state, pause/resume here, poll sub-futures”.

---

# 🔹 15. Writing a Minimal Async Runtime

A micro runtime shows how async Rust is built.

```rust
use std::future::Future;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::pin::Pin;

fn dummy_raw_waker() -> RawWaker {
    fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}

    RawWaker::new(std::ptr::null(), &RawWakerVTable::new(clone, wake, wake_by_ref, drop))
}

fn run<F: Future>(mut fut: F) -> F::Output {
    let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(val) => break val,
            Poll::Pending => continue,
        }
    }
}
```

Usage:

```rust
fn main() {
    let result = run(async { 1 + 2 });
    println!("{}", result); // prints 3
}
```

---

# 🔹 16. `async-trait` for Trait Support

Async methods in traits need boxing or `async-trait` crate.

```toml
# Cargo.toml
[dependencies]
async-trait = "0.1"
```

```rust
use async_trait::async_trait;

#[async_trait]
trait DataFetcher {
    async fn fetch(&self) -> String;
}

struct MyFetcher;

#[async_trait]
impl DataFetcher for MyFetcher {
    async fn fetch(&self) -> String {
        "data".to_string()
    }
}
```

Under the hood, this desugars to returning a `Pin<Box<dyn Future<Output = String> + Send + 'async>>>`.

---

# 🔹 17. Pinning + Projection

Sometimes, futures store internal pinned fields. To access them safely, use **pin projection**.

Use the `pin-project` crate:

```rust
use pin_project::pin_project;

#[pin_project]
struct MyFuture {
    #[pin]
    inner: SomeFuture,
}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.project(); // this.inner is pinned
        this.inner.poll(cx)
    }
}
```

---

# 🔹 18. Lifetime Challenges

Rust async functions can return futures with **captured references**.

```rust
async fn get_ref<'a>(s: &'a str) -> &'a str {
    s
}
```

This is fine, but you **can’t store the future unless it’s `'static`**.

```rust
fn store_it<'a>(fut: impl Future<Output = &'a str> + 'a) {
    // Can't make this `'static` unless fut doesn't borrow!
}
```

Use `.to_owned()` to avoid lifetime issues.

---

# 🔹 19. Cancellation

Rust futures are **cooperative** — if dropped, they’re cancelled.

```rust
let handle = tokio::spawn(async {
    loop {
        println!("Working...");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
});

tokio::time::sleep(Duration::from_secs(3)).await;
handle.abort(); // Cancels the task
```

No `finally` block: use `Drop` or `tokio::select!` with a `cancel` signal.

---

# 🔹 20. Summary Table

| Concept       | Description                                  |
| ------------- | -------------------------------------------- |
| `async fn`    | Returns a `Future`                           |
| `await`       | Polls the future, suspends if `Pending`      |
| `Poll`        | `Ready` or `Pending`                         |
| `Waker`       | Wake up task when progress can be made       |
| `Pin`         | Prevents memory relocation                   |
| `Context`     | Holds the waker                              |
| `join!`       | Run multiple futures concurrently (join all) |
| `select!`     | Run multiple futures, return first to finish |
| `async-trait` | Enables `async` in trait methods             |
| Cancellation  | Drop = cancel, use `Drop` or `select!`       |

---

