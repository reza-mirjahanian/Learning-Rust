// Import the I/O module from the standard library, including the module itself and the BufRead trait.
// The io module provides basic input/output functionality.
// BufRead trait provides methods for reading buffered data efficiently.
use std::io::{self, BufRead};

// Import the process module from the standard library.
// This module contains functions related to process management.
use std::process;

// The main function is the entry point of every Rust executable program.
fn main() {
    // Get the current process ID using the id() function from the process module.
    // This returns a unique identifier for the currently running process.
    let pid = process::id();
    
    // Print the process ID to standard output.
    println!("process ID: {}", pid);

    // Get a handle to the standard input stream.
    // This allows the program to read input from the user or from piped data.
    let stdin = io::stdin();
    
    // Lock the standard input handle to get a buffered reader.
    // Locking prevents other threads from accessing stdin simultaneously.
    // The lines() method returns an iterator over the lines of input.
    let mut lines = stdin.lock().lines();

    // Start an infinite loop to continuously read input.
    loop {
        // Attempt to read the next line from the input iterator.
        // The next() method returns an Option<Result<String, io::Error>>:
        // - Some(Ok(line)): Successfully read a line
        // - Some(Err(e)): Error occurred while reading
        // - None: End of input reached (EOF)
        let line = match lines.next() {
            // If a line was successfully read, use it
            Some(Ok(line)) => line,
            // If there was an error or end of input, handle it
            _ => {
                // Print an error message to standard error (stderr)
                eprintln!("Failed to read from stdin");
                // Exit the loop, which will end the program
                break;
            }
        };
        
        // Print the received line to standard output
        println!("Received: {}", line);
    }
}



// You’d need to handle the result explicitly, like this:
// match lines.next() {
//     Some(result) => match result {
//         Ok(line) => println!("Received: {}", line),
//         Err(err) => {
//             eprintln!("Error reading line: {}", err);
//             break;
//         }
//     },
//     None => break,
// }
