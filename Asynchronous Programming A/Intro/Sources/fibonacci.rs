use std::thread;

fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }
    fibonacci(n - 1) + (n - 2)
}

fn main() {
    let mut threads = Vec::new();

    for i in 0..8 {
        //Rust enforces this via the move keyword: it moves ownership of captured variables into the closure so they can live long enough to be used safely by the new thread.
        let handle = thread::spawn(move || {
            let result = fibonacci(4000);
            println!("Thread {}, result: {}", i, result);
        });
        threads.push(handle);
    }

    //handle.join() This joins the thread — meaning it blocks the main thread until the spawned thread is done.
    // If the thread panicked, .unwrap() will crash the program with an error.
    for handle in threads {
        handle.join().unwrap(); // Ensures all threads complete before main() exits
    }
}
