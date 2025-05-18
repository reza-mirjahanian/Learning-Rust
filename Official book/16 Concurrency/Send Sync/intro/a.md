
---

# 🔹 Foundational Concepts

## **1. What Are `Send` and `Sync`?**

These are **marker traits** in Rust defined in the standard library.

```rust
pub unsafe auto trait Send { }
pub unsafe auto trait Sync { }
```

* **`Send`**: A type is safe to move to another thread.
* **`Sync`**: A type is safe to be referenced from multiple threads **concurrently**.

They are:

* **`unsafe auto traits`**: Implemented automatically when safe unless explicitly opted out.
* Cannot be implemented manually for safe types unless using `unsafe`.

---

## **2. Memory Model Summary**

| Trait  | Meaning                                               | Example Types      |
| ------ | ----------------------------------------------------- | ------------------ |
| `Send` | Ownership can be transferred across threads           | `Box<T>`, `Vec<T>` |
| `Sync` | References (`&T`) can be shared across threads safely | `Arc<T>`, `&T`     |

---

## **3. Analogy**

Think of a type like a resource (e.g., file handle):

* `Send`: I can hand it over to another person (thread).
* `Sync`: We can **both** read it **at the same time** without conflict.

---

# 🔹 `Send` Trait

## **Definition**

A type `T` is `Send` if it is safe to move a value of type `T` to another thread.

### ✅ Types that are `Send`:

* All primitive types (`i32`, `bool`, etc.)
* `String`, `Vec<T>`, `Box<T>`
* `Option<T>` where `T: Send`

### ❌ Types that are **not** `Send`:

* `Rc<T>`: reference-counted pointer (not thread-safe)
* Raw pointers (`*const T`, `*mut T`) if `T` is not `Send`
* Most types containing `!Send` fields

### 🔍 Example: `Send`

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // `v` is Send, so this is allowed
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    handle.join().unwrap();
}
```

---

# 🔹 `Sync` Trait

## **Definition**

A type `T` is `Sync` if `&T` (a shared reference) is `Send`.

This means **it's safe for multiple threads to access `&T` concurrently.**

### ✅ Types that are `Sync`:

* Immutable primitives like `i32`, `bool`
* `&T` is `Sync` if `T` is `Sync`
* `Mutex<T>`, `RwLock<T>` are `Sync` (by design)

### ❌ Types that are **not** `Sync`:

* `Cell<T>` and `RefCell<T>`: allow interior mutability without thread safety
* `Rc<T>`: not thread-safe

### 🔍 Example: `Sync`

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let mut handles = vec![];

    for _ in 0..3 {
        let data_cloned = Arc::clone(&data);

        let handle = thread::spawn(move || {
            println!("{:?}", data_cloned);
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}
```

* `Arc<T>` is `Sync` when `T: Sync`

---

# 🔹 `Send + Sync` Combined

A type that is both `Send` and `Sync` can:

* Be **moved** to other threads.
* Be **shared** across threads as a reference.

```rust
fn is_send_sync<T: Send + Sync>() {}

fn main() {
    is_send_sync::<i32>();
    is_send_sync::<Vec<i32>>();
    is_send_sync::<Arc<String>>();
}
```

---

# 🔹 Interior Mutability and `Send`/`Sync`

## 🚫 `RefCell<T>` vs ✅ `Mutex<T>`

| Type         | Thread-safe? | `Send` | `Sync` |
| ------------ | ------------ | ------ | ------ |
| `RefCell<T>` | ❌ No         | ❌      | ❌      |
| `Mutex<T>`   | ✅ Yes        | ✅      | ✅      |

```rust
use std::cell::RefCell;
use std::sync::Mutex;

fn main() {
    let _a = RefCell::new(42); // Not Sync or Send
    let _b = Mutex::new(42);   // Sync and Send
}
```

---

# 🔹 Auto Traits and Negative Implementations

## Auto traits:

Rust **automatically** implements `Send` and `Sync` if:

* All fields are `Send`/`Sync`
* No unsafe interior mutation

## Opt-out with `!Send`, `!Sync`:

```rust
use std::marker::PhantomData;
use std::rc::Rc;

// Rc is not Send or Sync
struct NotSend {
    _rc: Rc<u8>,
}
```

---

# 🔹 Unsafe Manual Implementation

```rust
use std::cell::UnsafeCell;
use std::marker::Sync;

struct MyType {
    data: UnsafeCell<u32>,
}

// Force `Sync` manually (unsafe!)
unsafe impl Sync for MyType {}

fn main() {
    let a = MyType { data: UnsafeCell::new(42) };
    let a_ref = &a;
}
```

> ⚠️ Only do this if you're sure it's thread-safe! `UnsafeCell` allows mutation through `&self`.

---

# 🔹 Common Gotchas

## 🔥 `Send` is not transitive through `Rc`

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);

    std::thread::spawn(move || {
        println!("{:?}", data); // ❌ Compile error: Rc<T> is not Send
    });
}
```

Use `Arc<T>` instead.

---

# 🔹 Testing for `Send`/`Sync`

```rust
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<i32>();
    assert_sync::<Vec<i32>>();
}
```

You can test if custom types are `Send` or `Sync` using these dummy functions.

---

# 🔹 Advanced Use Cases

## 📌 Custom Types with Unsafe Fields

```rust
use std::sync::Arc;
use std::cell::UnsafeCell;

struct MySharedData {
    inner: UnsafeCell<u32>,
}

// Allow sharing reference across threads (Sync), but not sending (not Send)
unsafe impl Sync for MySharedData {}

fn main() {
    let shared = Arc::new(MySharedData { inner: UnsafeCell::new(0) });

    let threads: Vec<_> = (0..5).map(|_| {
        let s = shared.clone();
        std::thread::spawn(move || {
            // Reading shared data here is safe because we guarantee no mutation
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }
}
```

---

# 🔹 Comparison Table Summary

| Type            | `Send` | `Sync` | Notes                              |
| --------------- | ------ | ------ | ---------------------------------- |
| `i32`, `bool`   | ✅      | ✅      | Primitive types                    |
| `Vec<T>`        | ✅      | ✅      | When `T: Send + Sync`              |
| `Box<T>`        | ✅      | ✅      |                                    |
| `Rc<T>`         | ❌      | ❌      | Use `Arc<T>` instead               |
| `Arc<T>`        | ✅      | ✅      | When `T: Send + Sync`              |
| `RefCell<T>`    | ❌      | ❌      | Not thread-safe                    |
| `Mutex<T>`      | ✅      | ✅      | Provides locking                   |
| `UnsafeCell<T>` | ❌      | ❌      | Needs manual unsafe implementation |

---

# 🔹 Final Tips

* **Use `Arc<Mutex<T>>`** for shared **mutable** access across threads.
* **Prefer `Send + Sync` bounds** in thread pools and async runtimes.
* **Avoid `Rc`, `RefCell` in multithreaded code.**

---


