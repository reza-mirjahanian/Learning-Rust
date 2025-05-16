https://doc.rust-lang.org/book/ch16-00-concurrency.html#fearless-concurrency

-   How to create threads to run multiple pieces of code at the same time
-   *Message-passing* concurrency, where channels send messages between threads
-   *Shared-state* concurrency, where multiple threads have access to some piece of data
-   The `Sync` and `Send` traits, which extend Rust's concurrency guarantees to user-defined types as well as types provided by the standard library