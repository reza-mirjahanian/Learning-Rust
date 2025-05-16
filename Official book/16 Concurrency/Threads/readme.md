[Using Threads to Run Code Simultaneously - The Rust Programming Language](https://doc.rust-lang.org/book/ch16-01-threads.html)

This can lead to problems, such as:

-   Race conditions, in which threads are accessing data or resources in an inconsistent order
-   Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
-   Bugs that happen only in certain situations and are hard to reproduce and fix reliably


### [Waiting for All Threads to Finish Using `join` Handles](https://doc.rust-lang.org/book/ch16-01-threads.html#waiting-for-all-threads-to-finish-using-join-handles)

Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates. *Blocking* a thread means that thread is prevented from performing work or exiting.