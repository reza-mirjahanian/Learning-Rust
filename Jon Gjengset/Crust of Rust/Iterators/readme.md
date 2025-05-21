https://gist.github.com/jonhoo/dd63b720fa4a220ea845a77e2d75831e


```rust
(0..) // It represents 0 to infinity. You can iterate over it as long as you want (or until overflow).

let evens = (0..).step_by(2).take(5);  // 0, 2, 4, 6, 8

//////////////////


let mut iter = (0..).map(|i| 0..i);

let r0 = iter.next().unwrap();  // 0..0
let r1 = iter.next().unwrap();  // 0..1
let r2 = iter.next().unwrap();  // 0..2
``` 


```rust
std::iter::once(value)
// Yields the value once, then ends.
//  Inject a single item into a larger iterator chain
let v = vec![1, 2, 3];

let extended: Vec<_> = v.into_iter()
    .chain(iter::once(4))
    .collect();

assert_eq!(extended, vec![1, 2, 3, 4]);
//

let mut it = iter::once(10);

assert_eq!(it.next(), Some(10));
assert_eq!(it.next(), None);  // Only once!
``` 
| Feature  | `std::iter::once(x)` | `vec![x]`             |
| -------- | -------------------- | --------------------- |
| Type     | Iterator             | Vector                |
| Lazy?    | ✅ Yes                | ❌ No (eager)          |
| Chaining | Easy with `.chain()` | Requires `vec.iter()` |



| Syntax         | Meaning                                                    |
| -------------- | ---------------------------------------------------------- |
| `collect()`    | Turns an iterator into a collection                        |
| `::<Vec<_>>()` | Type hint: collect into a `Vec<T>`, where `T` is inferred  |
| `Vec<_>`       | Rust infers what each item in the vector is (e.g., `&str`) |
