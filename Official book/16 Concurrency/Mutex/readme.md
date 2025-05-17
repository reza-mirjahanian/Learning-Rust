https://doc.rust-lang.org/book/ch16-03-shared-state.html


Mutexes have a reputation for being difficult to use because you have to remember two rules:

1.  You must attempt to acquire the lock before using the data.
2.  When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.


Note that if you are doing simple numerical operations, there are types simpler than `Mutex<T>` types provided by the [`std::sync::atomic` module of the standard library](https://doc.rust-lang.org/std/sync/atomic/index.html). These types provide safe, concurrent, atomic access to primitive types. We chose to use `Mutex<T>` with a primitive type for this example so we could concentrate on how `Mutex<T>` works.


### [Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`](https://doc.rust-lang.org/book/ch16-03-shared-state.html#similarities-between-refcelltrct-and-mutextarct)

You might have noticed that `counter` is immutable but we could get a mutable reference to the value inside it; this means `Mutex<T>` provides interior mutability, as the `Cell` family does. In the same way we used `RefCell<T>` in Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>`.

Another detail to note is that Rust can't protect you from all kinds of logic errors when you use `Mutex<T>`. Recall from Chapter 15 that using `Rc<T>` came with the risk of creating reference cycles, where two `Rc<T>` values refer to each other, causing memory leaks. Similarly, `Mutex<T>` comes with the risk of creating *deadlocks*. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever. If you're interested in deadlocks, try creating a Rust program that has a deadlock; then research deadlock mitigation strategies for mutexes in any language and have a go at implementing them in Rust. The standard library API documentation for `Mutex<T>` and `MutexGuard` offers useful information.