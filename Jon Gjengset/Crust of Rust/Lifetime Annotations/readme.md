
https://gist.github.com/jonhoo/2a7fdcf79be03e51a5f95cd326f2a1e8

### Key Learning Challenge Identified

*   **Lifetimes:** Many users expressed confusion.
*   **Desired Approach:** Users prefer seeing *code examples* using lifetimes rather than just abstract explanations.

### Stream Plan & Goals

*   **Content Focus:**
    *   Write practical Rust code.
    *   Cover multiple lifetimes.
    *   Discuss strings (common confusion point).
    *   Touch on generics (time permitting).
*   **Format:**
    *   Target duration: Approximately **90 minutes** (shorter than usual).
    *   Style: Aim to be less verbose.
    *   Interactive: Take questions from live chat, beneficial for both live and replay viewers.
*   **Project:** Build a string splitting utility library (`str_split`).

### Project Setup (`str_split` library)

1.  **Initialization:**
    *   `cargo new --lib str_split`
2.  **Metadata & Configuration (`Cargo.toml`):**
    *   Define crate metadata.
3.  **Prelude Lints (`src/lib.rs`):**    *   Add `#![warn(...)]` for:
        *   `missing_debug_implementations`
        *   `rust_2018_idioms`
        *   `missing_docs`
    *   *Rationale for `warn` vs `deny`*: Avoids build breakage due to future compiler lint changes.
    *   *Note*: Lints might be disabled during early prototyping to reduce noise.

### Initial `strSplit` API Design

*   **Struct:**
    ```rust
    struct strSplit {
        // fields to be added
    }
    ```
*   **Constructor:**
    ```rust
    impl strSplit {
        fn new(haystack: &str, delimiter: &str) -> Self {
            // implementation
        }
    }
    // Note: Using Self is preferred for easier renaming.
    ```
*   **Iterator Implementation:**
    ```rust
    impl Iterator for strSplit {
        type Item = &str; // The type of item yielded by the iterator

        fn next(&mut self) -> Option<Self::Item> {
            // implementation
        }
    }
    // Note: `for` loops use the `next` method internally.
    ```
*   **Basic Test Case:**
    ```rust    let haystack = "a b c d e";
    let letters = strSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    ```

### Implementation V1: Single Lifetime

*   **Struct Fields:**
    ```rust
    struct strSplit<'a> {
        remainder: &'a str,
        delimiter: &'a str,
    }
    ```
*   **`new` Implementation:**
    ```rust
    fn new<'a>(haystack: &'a str, delimiter: &'a str) -> Self<'a> {
        Self {
            remainder: haystack,
            delimiter, // Field init shorthand
        }
    }
    ```
*   **`next` Implementation Logic:**
    1.  Find the next occurrence of `delimiter` in `remainder` using `find()`.
    2.  If found:
        *   Slice `remainder` up to the delimiter (`until_delimiter`).
        *   Update `remainder` to be the part *after* the delimiter.
        *   Return `Some(until_delimiter)`.
    3.  If not found:
        *   Handle the remaining part (potential bug addressed later).
        *   Return `Some(rest)` or `None`.
*   **Compilation Issues & Lifetime Introduction:**
    *   **Error:** `missing lifetime specifier`.
    *   **Solution:** Add lifetime parameter `'a` to the struct, impl blocks, and references.
        ```rust
        struct strSplit<'a> { ... }
        impl<'a> strSplit<'a> { ... }
        impl<'a> Iterator for strSplit<'a> {
            type Item = &'a str; // Crucial: returned item lives as long as input
            ...
        }
        ```
    *   **Error:** `lifetime of reference outlives lifetime of borrowed content` in `new`.
    *   **Explanation:** The lifetimes of the input `haystack` and `delimiter` were not explicitly tied to the lifetime `'a` of the returned `strSplit<'a>`.
    *   **Solution:** Update `new` signature: `fn new<'a>(haystack: &'a str, delimiter: &'a str) -> Self<'a>`. This ensures the input references live *at least as long* as the struct instance.
*   **`'static` Lifetime:**
    *   String literals (e.g., `""`) have a `'static` lifetime (live for the entire program duration).
    *   *Subtyping*: A longer lifetime (like `'static`) can be used where a shorter lifetime (`'a`) is expected.
*   **Bug Fix (Trailing Delimiter):**
    *   Issue: Doesn't produce an empty string if the input ends with the delimiter.
    *   Solution: Change `remainder` field to `Option<&'a str>` and use `Option::take()` in the `else` branch of `next` to handle the final empty segment correctly.
    *   Introduced `ref mut` for matching mutable references inside the `Option`.
    *   Used `Option::as_mut()` and the `?` operator for concise handling.

### Implementation V2: Multiple Lifetimes

*   **Motivation:** Create a helper function `until_char(&str, char) -> &str` using `strSplit`.
    ```rust
    fn until_char(s: &str, c: char) -> &str {
        let delimiter = format!("{}", c); // Creates a temporary String
        strSplit::new(s, &delimiter) // Pass reference to temporary String
            .next()
            .expect("strSplit always gives at least one result")
    }
    ```
*   **Problem:** Causes "cannot return value referencing temporary value" error.
*   **Explanation:**
    *   The `delimiter` `String` only lives within the `until_char` function scope.
    *   The single lifetime `'a` forces `s` (potentially long-lived) and `&delimiter` (short-lived) to adopt the *shorter* lifetime.
    *   `strSplit::next()` returns `&'a str`, which is now tied to the short lifetime of `delimiter`, but the function signature wants to return a reference tied to the lifetime of `s`.
*   **Solution: Introduce Named Lifetimes:**
    *   Use two distinct lifetimes: `'haystack` and `'delimiter`.
    *   Update struct and impls:
        ```rust
        struct strSplit<'haystack, 'delimiter> {
            remainder: &'haystack str,
            delimiter: &'delimiter str,
        }        impl<'haystack, 'delimiter> strSplit<'haystack, 'delimiter> {
            fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self { ... }
        }

        impl<'haystack, 'delimiter> Iterator for strSplit<'haystack, 'delimiter> {
            type Item = &'haystack str; // Return type ONLY tied to haystack lifetime

            fn next(&mut self) -> Option<Self::Item> { ... }
        }
        ```
    *   Now the returned slice's lifetime (`'haystack`) is independent of the delimiter's lifetime (`'delimiter`), resolving the error in `until_char`.
*   **Lifetime Elision (`'_`):** Where a lifetime parameter isn't needed in the output (like `'delimiter` in the `Iterator` impl), it can often be elided or replaced with `'_`.

### Implementation V3: Generics & Traits

*   **Motivation:** Avoid the heap allocation caused by `format!("{}", c)` in `until_char`.
*   **Solution:** Make the `delimiter` type generic.
    *   Introduce a `Delimiter` trait:
        ```rust
        pub trait Delimiter {
            fn find_next(&self, s: &str) -> Option<(usize, usize)>; // Returns start and end byte indices
        }
        ```
    *   Update `strSplit` struct:
        ```rust
        struct strSplit<'haystack, D> { // D is the generic delimiter type
            remainder: Option<&'haystack str>, // Remainder still tied to haystack
            delimiter: D,
        }
        ```
    *   Add trait bound to impls: `impl<'haystack, D: Delimiter> ...`
    *   Implement `Delimiter` for necessary types:
        *   `impl<'a> Delimiter for &'a str { ... }` (using `s.find(self)`)
        *   `impl Delimiter for char { ... }` (using `s.char_indices().find_map(...)`)
*   **Benefit:** `until_char` can now pass the `char` directly to `strSplit::new` without allocating a `String`.

### The "Big Secret"

*   The functionality built (`strSplit`, `Delimiter` trait) largely replicates existing Rust standard library features:
    *   `str::split()` method.
    *   The `Pattern` trait (which `&str` and `char` implement).
*   The exercise was intended to demonstrate *how* these features could be built and *why* concepts like multiple lifetimes and generics are necessary for such APIs.

### Q&A Highlights & Concepts Explained

*   **Lifetimes vs. Scope:** Lifetimes track how long a *reference* is valid, which relates to but isn't identical to the scope where a *value* is declared. Values live until dropped or moved.
*   **`&str` vs `String`:**
    *   `&str`: A reference (**fat pointer**: address + length) to a sequence of bytes (usually UTF-8), can point anywhere (stack, heap, static memory). *Does not own* the data.
    *   `String`: An owned, heap-allocated, growable buffer. *Owns* the data.
    *   Conversion: `&String` -> `&str` (cheap), `&str` -> `String` (expensive, requires allocation + copy).
*   **`ref` and `ref mut`:** Used in patterns to bind to a reference *into* the matched value, rather than moving/copying the value itself.
*   **`Option::take()`:** Takes the `Some(value)` out of an `Option`, leaving `None` behind.
*   **`Option::as_mut()`:** Converts `&mut Option<T>` to `Option<&mut T>`.
*   **`?` Operator:** Works on `Option` (and `Result`), returning `None` early if the `Option` is `None`.
*   **Anonymous Lifetimes (`'_`):** Tell the compiler to infer the lifetime; usable when unambiguous.
*   **Lifetime Subtyping (`'long: 'short`):** A reference with a longer lifetime (`'long`) can be used where a shorter one (`'short`) is required. `'static` is the longest lifetime.
*   **Generic Code:** Writing code that works with multiple types (using `<T>`) or lifetimes (using `<'a>`).