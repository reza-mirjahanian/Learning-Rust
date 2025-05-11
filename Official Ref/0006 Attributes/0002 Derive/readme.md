https://doc.rust-lang.org/reference/attributes/derive.html

The *`derive` attribute* allows new [items](https://doc.rust-lang.org/reference/items.html) to be automatically generated for data structures.


It uses the [*MetaListPaths*](https://doc.rust-lang.org/reference/attributes.html#meta-item-attribute-syntax) syntax to specify a list of traits to implement or paths to [derive macros](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros) to process.

For example, the following will create an [`impl` item](https://doc.rust-lang.org/reference/items/implementations.html) for the [`PartialEq`](https://doc.rust-lang.org/core/cmp/trait.PartialEq.html) and [`Clone`](https://doc.rust-lang.org/core/clone/trait.Clone.html) traits for `Foo`, and the type parameter `T` will be given the `PartialEq` or `Clone` constraints for the appropriate `impl`:

```
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}

```

The generated `impl` for `PartialEq` is equivalent to

```

impl<T: PartialEq> PartialEq for Foo<T> {
    fn eq(&self, other: &Foo<T>) -> bool {
        self.a == other.a && self.b == other.b
    }
}

```

You can implement `derive` for your own traits through [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros).

The *`automatically_derived` attribute* is automatically added to [implementations](https://doc.rust-lang.org/reference/items/implementations.html) created by the `derive` attribute for built-in traits. It has no direct effect, but it may be used by tools and diagnostic lints to detect these automatically generated implementations.