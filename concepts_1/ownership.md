**ownership**

*   Each value in Rust has an _owner_.
*   There can only be one owner at a time.
*   When the owner goes out of scope, the value will be dropped.

In languages with a _garbage collector (GC)_, the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one `allocate` with exactly one `free`.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here’s a version of our scope example from Listing 4-1 using a `String` instead of a string literal:

```plaintext
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
```


There is a natural point at which we can return the memory our `String` needs to the allocator: when `s` goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called [`drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop), and it’s where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

> Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called _Resource Acquisition Is Initialization (RAII)_. The `drop` function in Rust will be familiar to you if you’ve used RAII patterns.

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does

The ownership rules are as follows:

Each value in Rust has a variable that is its owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
In addition to the ownership rules, Rust also employs a borrowing system that allows you to temporarily lend ownership of a value to another part of your code without transferring ownership permanently. This can be useful for passing values to functions or for implementing data structures that need to share ownership of a value between multiple parts of the code.