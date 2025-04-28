
https://doc.rust-lang.org/reference/lifetime-elision.html

Rust has rules that allow lifetimes to be elided in various places where the compiler can infer a sensible default choice.

### Lifetime elision in functions


In order to make common patterns more ergonomic, lifetime arguments can be elided in function item, function pointer, and closure trait signatures. The following rules are used to infer lifetime parameters for elided lifetimes.


It is an error to elide lifetime parameters that cannot be inferred.

The placeholder lifetime, '\_, can also be used to have a lifetime inferred in the same way. For lifetimes in paths, using '\_ is preferred.


##### Trait object lifetimes follow different rules discussed

-   Each elided lifetime in the parameters becomes a distinct lifetime parameter.


-   If there is exactly one lifetime used in the parameters (elided or not), that lifetime is assigned to *all* elided output lifetimes.


##### In method signatures there is another rule

-   If the receiver has type `&Self` or `&mut Self`, then the lifetime of that reference to `Self` is assigned to all elided output lifetime parameters.

Examples:
```rust
fn print1(s: &str);                                   // elided
fn print2(s: &'_ str);                                // also elided
fn print3<'a>(s: &'a str);                            // expanded

fn debug1(lvl: usize, s: &str);                       // elided
fn debug2<'a>(lvl: usize, s: &'a str);                // expanded

fn substr1(s: &str, until: usize) -> &str;            // elided
fn substr2<'a>(s: &'a str, until: usize) -> &'a str;  // expanded

fn get_mut1(&mut self) -> &mut dyn T;                 // elided
fn get_mut2<'a>(&'a mut self) -> &'a mut dyn T;       // expanded

fn args1<T: ToCStr>(&mut self, args: &[T]) -> &mut Command;                  // elided
fn args2<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn other_args1<'a>(arg: &str) -> &'a str;             // elided
fn other_args2<'a, 'b>(arg: &'b str) -> &'a str;      // expanded

fn new1(buf: &mut [u8]) -> Thing<'_>;                 // elided - preferred
fn new2(buf: &mut [u8]) -> Thing;                     // elided
fn new3<'a>(buf: &'a mut [u8]) -> Thing<'a>;          // expanded

type FunPtr1 = fn(&str) -> &str;                      // elided
type FunPtr2 = for<'a> fn(&'a str) -> &'a str;        // expanded

type FunTrait1 = dyn Fn(&str) -> &str;                // elided
type FunTrait2 = dyn for<'a> Fn(&'a str) -> &'a str;  // expanded
```

The following examples show situations where it is not allowed to elide the
lifetime parameter.

```rust
// Cannot infer, because there are no parameters to infer from.
fn get_str() -> &str;                                 // ILLEGAL

// Cannot infer, ambiguous if it is borrowed from the first or second parameter.
fn frob(s: &str, t: &str) -> &str;                    // ILLEGAL
```


### Default trait object lifetimes

The assumed lifetime of references held by a [trait object](https://doc.rust-lang.org/reference/types/trait-object.html) is called its *default object lifetime bound*. These were defined in [RFC 599](https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md) and amended in [RFC 1156](https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md).