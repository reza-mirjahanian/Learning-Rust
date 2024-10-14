### Summary of the `?` Operator:

-   **For `Result<T, E>`**:

    -   If the result is `Ok(T)`, the value is unwrapped.
    -   If the result is `Err(E)`, the error is returned from the current function.
-   **For `Option<T>`**:

    -   If the option is `Some(T)`, the value is unwrapped.
    -   If the option is `None`, `None` is returned from the current function.
-   **Error conversion**: If the error types differ, the `?` operator tries to convert them using the `From` trait.
  
----------------