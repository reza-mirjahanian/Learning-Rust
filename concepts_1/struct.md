### [Where’s the `->` Operator?](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator)

In C and C++, two different operators are used for calling methods: you use `.` if you’re calling a method on the object directly and `->` if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if `object` is a pointer, `object->something()` is similar to `(*object).something()`.

Rust doesn’t have an equivalent to the `->` operator; instead, Rust has a feature called _automatic referencing and dereferencing_. Calling methods is one of the few places in Rust that has this behavior.

Here’s how it works: when you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so `object` matches the signature of the method. In other words, the following are the same:

```plaintext
p1.distance(&p2);
(&p1).distance(&p2);
```

## The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of `self`. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

---

```plaintext
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}        
```

`impl Struct ...` adds some methods to `Struct`. These methods aren't available to other types or traits.

`impl Trait for Struct ..` implements the trait `Trait` for the struct `Struct`. This results in the methods of the trait being available for `Struct`.

So, even though these two syntaxes look similar, they do 2 completely different things. `impl Struct ...` adds new (not previously defined) methods to the type, while the other adds previously defined methods (from the trait) to the type.