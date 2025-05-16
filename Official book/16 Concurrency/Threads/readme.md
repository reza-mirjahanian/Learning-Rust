[Using Threads to Run Code Simultaneously - The Rust Programming Language](https://doc.rust-lang.org/book/ch16-01-threads.html)

This can lead to problems, such as:

-   Race conditions, in which threads are accessing data or resources in an inconsistent order
-   Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
-   Bugs that happen only in certain situations and are hard to reproduce and fix reliably