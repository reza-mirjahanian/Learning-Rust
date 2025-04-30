A blanket implementation is an implementation of a trait on a generic parameter:

```
impl<T> Trait for T

```

They usually also have `where` clauses involved since it is very hard to do anything useful to an unconstrained `T`. This does not cover things like `impl<T> Trait for Vec<T>`, which is a *generic* implementation but not a *blanket* implementation.

They are documented separately since they are applied without any particularity and may or may not be relevant to the type you're looking at. Whereas for the normal [*"Trait Implementations"*](https://doc.rust-lang.org/std/vec/struct.Vec.html#trait-implementations) section, all those traits at least had some thought for the specific type in mind (usually).

They are patently useful since it implements the trait for *anything*, in the entire ecosystem! If something satisfies the constraints, then it is able to take advantage of the implementation without needing to implement it themselves. You do not need to do anything to "opt-in" besides bringing the trait into scope.

Some notable ones:

-   `From<T>` is implemented for all `T` (the identity implementation)
-   `Into<U>` is implemented for all `T` where `T: From<U>` (the reflexive implementation that allows you to call `.into()` when a matching `From` implementation exists)
-   `Any` is implemented for all `T` where `T: 'static`

They are also needed to implement "trait aliases" where you create a trait that is constrained over multiple other traits (handy for reducing boilerplate and needed for multi-trait trait objects). You use a blanket implementation that the trait is implemented for any type satisfying the super-traits:

```rust
trait MyCoolAsyncTrait: AsyncRead + AsyncWrite + AsyncSeek + 'static {}

impl<T> MyCoolAsyncTrait for T
where
    T: AsyncRead + AsyncWrite + AsyncSeek + 'static
{ }

```

Be careful when adding them to your types though. Because of their extensive scope, they can easily conflict with other trait implementations that may be desirable. You can only define one blanket implementation per type ([even if the constraints are non-overlapping](https://stackoverflow.com/questions/40392524/conflicting-trait-implementations-even-though-associated-types-differ)).

See also:

-   [What are blanket implementations?](https://users.rust-lang.org/t/what-are-blanket-implementations/49904) (Rust forum)
-   [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods) in the Rust book
-   [Is there any way to create a type alias for multiple traits?](https://stackoverflow.com/q/26070559/2189130)
-   [How is there a conflicting implementation of \`From\` when using a generic type?](https://stackoverflow.com/q/37347311/2189130)
-   [Why do blanket implementations for two different traits conflict?](https://stackoverflow.com/q/73782573/2189130)