https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html

### [Allowing Transference of Ownership Between Threads with `Send`](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html#allowing-transference-of-ownership-between-threads-with-send)

The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads. Almost every Rust type is `Send`, but there are some exceptions, including `Rc<T>`: this cannot implement `Send` because if you cloned an `Rc<T>` value and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time. For this reason, `Rc<T>` is implemented for use in single-threaded situations where you don't want to pay the thread-safe performance penalty.

Therefore, Rust's type system and trait bounds ensure that you can never accidentally send an `Rc<T>` value across threads unsafely. When we tried to do this in Listing 16-14, we got the error `the trait Send is not implemented for Rc<Mutex<i32>>`. When we switched to `Arc<T>`, which does implement `Send`, the code compiled.


### [Allowing Access from Multiple Threads with `Sync`](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html#allowing-access-from-multiple-threads-with-sync)

The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. In other words, any type `T` implements `Sync` if `&T` (an immutable reference to `T`) implements `Send`, meaning the reference can be sent safely to another thread. Similar to `Send`, primitive types all implement `Sync`, and types composed entirely of types that implement `Sync` also implement `Sync`.

The smart pointer `Rc<T>` also doesn't implement `Sync` for the same reasons that it doesn't implement `Send`. The `RefCell<T>` type (which we talked about in Chapter 15) and the family of related `Cell<T>` types don't implement `Sync`. The implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe. The smart pointer `Mutex<T>` implements `Sync` and can be used to share access with multiple threads as you saw in ["Sharing a `Mutex<T>` Between Multiple Threads"](https://doc.rust-lang.org/book/ch16-03-shared-state.html#sharing-a-mutext-between-multiple-threads).


### [Implementing `Send` and `Sync` Manually Is Unsafe](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html#implementing-send-and-sync-manually-is-unsafe)

Because types composed entirely of other types that implement the `Send` and `Sync` traits also automatically implement `Send` and `Sync`, we don't have to implement those traits manually. As marker traits, they don't even have any methods to implement. They're just useful for enforcing invariants related to concurrency.

Manually implementing these traits involves implementing unsafe Rust code. We'll talk about using unsafe Rust code in Chapter 20; for now, the important information is that building new concurrent types not made up of `Send` and `Sync` parts requires careful thought to uphold the safety guarantees. ["The Rustonomicon"](https://doc.rust-lang.org/nomicon/index.html) has more information about these guarantees and how to uphold them.