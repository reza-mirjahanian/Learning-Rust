[Using Threads to Run Code Simultaneously - The Rust Programming Language](https://doc.rust-lang.org/book/ch16-01-threads.html)

This can lead to problems, such as:

-   Race conditions, in which threads are accessing data or resources in an inconsistent order
-   Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
-   Bugs that happen only in certain situations and are hard to reproduce and fix reliably


### [Waiting for All Threads to Finish Using `join` Handles](https://doc.rust-lang.org/book/ch16-01-threads.html#waiting-for-all-threads-to-finish-using-join-handles)

Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates. *Blocking* a thread means that thread is prevented from performing work or exiting.


We'll often use the `move` keyword with closures passed to `thread::spawn` because the closure will then take **ownership** of the values it uses from the environment, thus transferring ownership of those values from one thread to another.

--------------

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```

correct:
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```