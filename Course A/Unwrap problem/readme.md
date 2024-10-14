`unwrap` is fine in many cases, but it shouldn't be the first intuition for dealing with unexpected situations. Especially when you're writing a library or a function that is part of a larger codebase, you should strive to handle such situations gracefully. And in production code, it sets a bad example: one `unwrap` attracts another and the codebase becomes more fragile as you continue down this path

## The Problem
Very commonly, people write code like this:

```rust
// Assume that this fetches the user from somewhere
fn get_user() -> Option<String> {
    None
}

fn get_user_name() -> Result<String> {
    let user = get_user()?;
    // Do something with `user`
    // ...
    Ok(user)
}

```

The goal here is to return early if you encounter  `None`  in an  `Option`, so they use the  `?`  operator to propagate errors.

Alas, this code doesn't compile. Instead, you get a dreaded error message:

```rust
error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
  --> src/lib.rs:10:26
   |
9  | fn get_user_name() -> Result<String> {
   | ------------------------------------ this function returns a `Result`
10 |     let user = get_user()?;
   |                          ^ use `.ok_or(...)?` to provide an error compatible with `std::result::Result<String, Box<dyn std::error::Error>>`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `std::result::Result<String, Box<dyn std::error::Error>>`
   = help: the following other types implement trait `FromResidual<R>`:
             <std::result::Result<T, F> as FromResidual<Yeet<E>>>
             <std::result::Result<T, F> as FromResidual<std::result::Result<Infallible, E>>>

```

([Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7001ff6af6c0bcbf44249691d65b086f))



There's a lot of visual noise in this error message. The  `FromResidual`  and  `Yeet`  are implementation details which could be confusing to a new user, and the relevant details are somewhat obscured.

**And all we did was try to use the `?` operator for our `Option`.**

My main gripe with this error message is that it doesn't explain *why* the `?` operator doesn't work with `Option` in that case... just that it doesn't.

---------

Solution 1 (Not OK!):
```
fn get_user_name() -> Result<String, String> {
    let user = get_user().unwrap();
    // Do something with `user`
    Ok(user)
}
```
`unwrap` is fine in many cases, but it shouldn't be the first intuition for dealing with unexpected situations. Especially when you're writing a library or a function that is part of a larger codebase, you should strive to handle such situations gracefully. And in production code, it sets a bad example: one `unwrap` attracts another and the codebase becomes more fragile as you continue down this path


---------

