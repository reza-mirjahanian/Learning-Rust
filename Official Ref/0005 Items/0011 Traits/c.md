

# Traits in Rust

## Defining and Implementing Traits  
- A **trait** is declared with the `trait` keyword and groups method signatures and associated types together ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=Here%2C%20we%20declare%20a%20trait,String)). For example, `pub trait Summary { fn summarize(&self) -> String; }` defines a `Summary` trait with one method. Inside the trait’s braces we list method signatures (ending with `;`) and default implementations if any ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=Here%2C%20we%20declare%20a%20trait,String)).  
- To use a trait, types must **implement** it via `impl TraitName for Type { ... }`. For instance:  

  ```rust
  pub trait Summary {
      fn summarize(&self) -> String;
  }

  struct NewsArticle { /* fields omitted */ }

  impl Summary for NewsArticle {
      fn summarize(&self) -> String {
          format!("{}, by {} ({})", self.headline, self.author, self.location)
      }
  }
  ```  

  This binds the `Summary` behavior to `NewsArticle` ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=Implementing%20a%20trait%20on%20a,have%20for%20the%20particular%20type)). The “orphan rule” prevents implementing external traits on external types, ensuring coherence (you must own either the trait or the type to implement) ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=Here%2C%20we%20declare%20a%20trait,String)).  

## Default Method Implementations  
- Traits can provide **default method implementations**. A method with a body in the trait definition serves as a default that implementors can inherit or override ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=Sometimes%20it%E2%80%99s%20useful%20to%20have,override%20each%20method%E2%80%99s%20default%20behavior)). For example:  

  ```rust
  pub trait Summary {
      fn summarize(&self) -> String {
          String::from("(Read more...)")
      }
  }
  struct NewsArticle { /* fields */ }
  impl Summary for NewsArticle {} // uses default summarize()
  ```  

  Here, `NewsArticle` uses the default `summarize` unless it provides its own. This is useful for methods where a common behavior makes sense ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=Sometimes%20it%E2%80%99s%20useful%20to%20have,override%20each%20method%E2%80%99s%20default%20behavior)). An empty `impl Summary for NewsArticle {}` causes the default to be used ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=pub%20trait%20Summary%20,%7D)).  

## Trait Bounds and Generics  
- **Trait bounds** constrain generic type parameters. You can declare a generic function or type like `fn notify<T: Summary>(item: &T) { … }` to require that `T` implements `Summary` ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=The%20,bound%3B%20it%20looks%20like%20this)). Multiple bounds use `+`, e.g. `T: Summary + Display` (or `(impl Summary + Display)` for arguments) ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=We%20can%20also%20specify%20more,syntax)). Where-clauses (`where T: Debug + Clone`) improve readability when many bounds are needed ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=Using%20too%20many%20trait%20bounds,So%2C%20instead%20of%20writing%20this)). Trait bounds can also appear on `impl` blocks or associated types. This allows writing generic code over **any** type with the required behavior ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=A%20trait%20defines%20the%20functionality,type%20that%20has%20certain%20behavior)) ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=The%20,bound%3B%20it%20looks%20like%20this)).  
- For example:  

  ```rust
  use std::fmt::Display;
  fn print_summary<T: Summary + Display>(item: &T) {
      println!("{} says {}", item, item.summarize());
  }
  ```  

  combines two trait bounds. The Rust book notes that `impl Trait` syntax (e.g. `fn foo(item: &impl Summary)`) is syntactic sugar for these bounds ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=The%20,bound%3B%20it%20looks%20like%20this)).  

## Trait Objects and Dynamic Dispatch  
- **Trait objects** enable dynamic dispatch on trait methods. A trait object like `&dyn Draw` or `Box<dyn Draw>` is a pointer to some type implementing `Draw` plus a vtable for method lookup at runtime ([Using Trait Objects That Allow for Values of Different Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch18-02-trait-objects.html#:~:text=To%20implement%20the%20behavior%20we,the%20section%20%202%20%E2%80%9CDynamically)) ([Using Trait Objects That Allow for Values of Different Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch18-02-trait-objects.html#:~:text=When%20we%20use%20trait%20objects%2C,to%20know%20which%20method%20to)). Using a trait object, the compiler cannot monomorphize the call, so method calls go through a vtable pointer. For example:  

  ```rust
  trait Draw { fn draw(&self); }
  struct Button;
  impl Draw for Button { fn draw(&self) { /* ... */ } }
  let obj: Box<dyn Draw> = Box::new(Button);
  obj.draw(); // dynamic dispatch
  ```  

  As the Rust Book explains, a trait object is created with a pointer and the `dyn` keyword, and used wherever a concrete type would be ([Using Trait Objects That Allow for Values of Different Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch18-02-trait-objects.html#:~:text=To%20implement%20the%20behavior%20we,the%20section%20%202%20%E2%80%9CDynamically)). Calling methods on a trait object incurs one indirection: *“When we use trait objects, Rust must use dynamic dispatch”* ([Using Trait Objects That Allow for Values of Different Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch18-02-trait-objects.html#:~:text=When%20we%20use%20trait%20objects%2C,to%20know%20which%20method%20to)). Use `&dyn Trait`, `Box<dyn Trait>`, etc., to hold values of different types that share a trait. Compared to generic/`impl Trait` usage (which is static dispatch), trait objects allow heterogenous collections and runtime polymorphism at the cost of indirection.  

## Associated Types  
- **Associated types** are type placeholders in traits. A trait can declare `type Item;`, and each `impl` specifies the concrete type. For example, the `Iterator` trait has `type Item;` and `fn next(&mut self) -> Option<Self::Item>`. Associated types tie the return types to the implementing type, reducing the need for extra generic parameters. In an `impl` block you set the type, e.g. `type Item = u32;`. The Rust Reference notes that if the trait defines an associated type, implementations must supply the concrete type in `impl` (the trait itself only declares the name) ([Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#object-safety#:~:text=function%20defines%20a%20body%2C%20this,be%20specified%20in%20an%20implementation)). This simplifies generic signatures because the compiler can infer the concrete associated type for a given implementation.  

## Generic Associated Types (GATs)  
- **GATs** allow associated types to themselves have generic parameters. Stabilized in Rust 1.65 (2022) ([Generic associated types to be stable in Rust 1.65 | Rust Blog](https://blog.rust-lang.org/2022/10/28/gats-stabilization/#:~:text=As%20of%20Rust%201,to%20remove%20in%20the%20future)), they let you write e.g.:  

  ```rust
  trait LendingIterator {
      type Item<'a> where Self: 'a;
      fn next<'a>(&'a mut self) -> Self::Item<'a>;
  }
  ```  

  Here `Item<'a>` can borrow from `self` for lifetime `'a`. GATs round out the language’s generics syntax ([Generic associated types to be stable in Rust 1.65 | Rust Blog](https://blog.rust-lang.org/2022/10/28/gats-stabilization/#:~:text=At%20its%20core%2C%20generic%20associated,a%20GAT%20would%20look%20like)). They enable patterns like zero-copy iteration or streaming where the associated type depends on a lifetime or type parameter. The Rust blog notes GATs allow you to put generics on associated types (which was previously only possible on functions) ([Generic associated types to be stable in Rust 1.65 | Rust Blog](https://blog.rust-lang.org/2022/10/28/gats-stabilization/#:~:text=At%20its%20core%2C%20generic%20associated,a%20GAT%20would%20look%20like)). For example, a `LendingIterator` can return references tied to `self`. One limitation (as of stabilization) is that higher-ranked trait bounds like `for<'a> I::Item<'a>: Debug` may inadvertently imply a `'static` requirement, due to current borrow-checker limitations ([Generic associated types to be stable in Rust 1.65 | Rust Blog](https://blog.rust-lang.org/2022/10/28/gats-stabilization/#:~:text=Implied%20%60%27static%60%20requirement%20from%20higher,trait%20bounds)) ([Generic associated types to be stable in Rust 1.65 | Rust Blog](https://blog.rust-lang.org/2022/10/28/gats-stabilization/#:~:text=note%3A%20due%20to%20current%20limitations,)). Despite such quirks, GATs greatly expand trait flexibility.  

## Supertraits  
- A **supertrait** is a trait that requires another trait. You declare a supertrait with a colon: e.g. `trait Student: Person { ... }` means “Student is a supertrait of Person” and every `Student` type must also implement `Person` ([Supertraits - Rust By Example](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html#:~:text=Rust%20doesn%27t%20have%20,For%20example)). This is *not* inheritance of implementation, but an additional constraint on the impl. Multiple supertraits can be combined: `trait CompSciStudent: Programmer + Student { ... }` requires any `CompSciStudent` type to implement both `Programmer` and `Student` ([Supertraits - Rust By Example](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html#:~:text=%2F%2F%20CompSciStudent%20,CompSciStudent%3A%20Programmer%20%2B%20Student)). For example:  

  ```rust
  trait Person { fn name(&self) -> String; }
  trait Student: Person { fn university(&self) -> String; }
  struct CSStu(String, String);
  impl Person for CSStu { fn name(&self) -> String { self.0.clone() } }
  impl Student for CSStu { fn university(&self) -> String { self.1.clone() } }
  ```  

  Here `Student: Person` ensures we can call `name()` on any `Student`. Supertraits are like a trait “inheritance” of requirements ([Supertraits - Rust By Example](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html#:~:text=Rust%20doesn%27t%20have%20,For%20example)) (though Rust has no object-oriented inheritance).  

## Blanket Implementations  
- A **blanket implementation** applies a trait to all types satisfying certain bounds. In form: `impl<T> Trait for T where ... {}` ([What are "Blanket Implementations" in Rust? - Stack Overflow](https://stackoverflow.com/questions/73861891/what-are-blanket-implementations-in-rust#:~:text=A%20blanket%20implementation%20is%20an,trait%20on%20a%20generic%20parameter)). This is documented separately because it applies broadly. For example:  

  ```rust
  impl<T: Display> MyTrait for T {
      // methods...
  }
  ```  

  implements `MyTrait` for *any* `T` that implements `Display`. As one answer explains, a blanket impl is “an implementation of a trait on a generic parameter: `impl<T> Trait for T`” ([What are "Blanket Implementations" in Rust? - Stack Overflow](https://stackoverflow.com/questions/73861891/what-are-blanket-implementations-in-rust#:~:text=A%20blanket%20implementation%20is%20an,trait%20on%20a%20generic%20parameter)). Standard examples include `From<T> for T` and `Into<U> for T where T: From<U>` ([What are "Blanket Implementations" in Rust? - Stack Overflow](https://stackoverflow.com/questions/73861891/what-are-blanket-implementations-in-rust#:~:text=Some%20notable%20ones%3A)). Blanket impls let you write one generic impl that many types benefit from without extra code. Beware overlap and coherence: only one applicable impl may apply to a given type (the orphan rules still apply).  

## Auto Traits  
- **Auto traits** are special traits implemented by the compiler if no explicit impl or negative impl is given. Common auto traits include `Send`, `Sync`, `Unpin`, `UnwindSafe`, etc. ([Special types and traits - The Rust Reference](https://doc.rust-lang.org/reference/special-types-and-traits.html#:~:text=The%20Send%20%2C%20%2035%2C,Auto%20traits%20have%20special%20properties)). The Rust Reference notes: if no impl is written, the compiler auto-implements them according to well-defined rules (for pointers, composites, closures, etc.) ([Special types and traits - The Rust Reference](https://doc.rust-lang.org/reference/special-types-and-traits.html#:~:text=%5Blang)). For example, almost all types are `Send`/`Sync` unless they contain non-thread-safe pointers. You can opt out with a **negative impl** (nightly only): e.g. `impl !Send for MyType {}` ensures `MyType` is never `Send` and disables the implicit impl ([negative_impls - The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/language-features/negative-impls.html#:~:text=Declaring%20a%20negative%20impl%20,trait%20serves%20two%20purposes)). This is how Rust enforces thread-safety invariants (e.g. `Rc<T>` has `impl !Send`) without explicit code.  

## Negative Trait Bounds  
- Stable Rust does *not* support negative bounds like `T: !SomeTrait` in type signatures. The only way to express a negative guarantee is via the nightly `negative_impls` feature: 

  ```rust
  #![feature(negative_impls)]
  impl<T: ?Sized> !DerefMut for &T {}
  ```  

  This says “`&T` does not implement `DerefMut`” ([negative_impls - The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/language-features/negative-impls.html#:~:text=With%20the%20feature%20gate%20,as%20well%20as%20positive%20ones)). Negative impls give a semver guarantee and affect auto-traits: they disable the compiler’s auto-implementation for that type ([negative_impls - The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/language-features/negative-impls.html#:~:text=Declaring%20a%20negative%20impl%20,trait%20serves%20two%20purposes)). Outside of nightly, you must design APIs differently (often using marker traits or custom types) since there is no `T: !Trait` bound syntax in stable.  

## Object Safety  
- A trait is **object-safe** if it can be made into a trait object (`dyn Trait`). The rules (Rust Reference) are: every method must be callable on a trait object or be explicitly non-dispatchable ([Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#object-safety#:~:text=,mut%20self)) ([Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#object-safety#:~:text=,implies%20this)). Concretely, a dispatchable method cannot have generic parameters, cannot use `Self` except in its receiver, must have a receiver of type `&Self`, `&mut Self`, `Box<Self>`, `Rc<Self>`, `Arc<Self>`, or `Pin<P>` of those ([Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#object-safety#:~:text=,mut%20self)). Also it cannot return `impl Trait` or be `async fn` (since that hides a generic `Future`), and it cannot have a `where Self: Sized` bound ([Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#object-safety#:~:text=,implies%20this)). Methods that take `self` by value or are generic are *non-dispatchable* (callable only on concrete types). For example, a method `fn foo(self)` requires `Self: Sized` so it cannot be called on `dyn Trait`. If a trait has any incompatible methods, you simply cannot use `&dyn Trait` or `Box<dyn Trait>`. These object-safety rules ensure that all method calls can be resolved via the vtable at runtime ([Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#object-safety#:~:text=,mut%20self)) ([Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#object-safety#:~:text=,implies%20this)).  

## Async Traits (Limitations & Workarounds)  
- As of Rust 1.75 (Dec 2023), the language **permits writing `async fn` in trait definitions** (since async fn sugar is just `-> impl Future`) ([Announcing `async fn` and return-position `impl Trait` in traits | Rust Blog](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits/#:~:text=The%20Rust%20Async%20Working%20Group,in%20traits)). For example: `trait HttpService { async fn fetch(&self, url: Url) -> Body; }`. However, async trait methods (and any methods using `-> impl Trait`) are *not object-safe* and thus cannot be used through `dyn Trait` for now ([Announcing `async fn` and return-position `impl Trait` in traits | Rust Blog](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits/#:~:text=Traits%20that%20use%20%60,variant%60%20crate)). In practice, this means you can use async trait methods in generic contexts, but you cannot do `Box<dyn MyAsyncTrait>`. The common workaround on stable is to use the [`async-trait`](https://crates.io/crates/async-trait) crate (a procedural macro by David Tolnay) which rewrites async trait methods into returning `Pin<Box<dyn Future>>`. For example:  

  ```rust
  #[async_trait]
  trait MyTrait { async fn do_something(&self); }
  ```  

  Under the hood this generates a trait method returning a boxed future. The async working group blog notes that while async fn in traits are now allowed, *“they lack support for dynamic dispatch”* ([Announcing `async fn` and return-position `impl Trait` in traits | Rust Blog](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits/#:~:text=Traits%20that%20use%20%60,variant%60%20crate)). For fully async trait objects one must still use such crates or manually write methods returning `Box<dyn Future<Output=...>> + Send`, or use the nightly `trait-variant` utilities once available.  

## Trait Aliases (Nightly)  
- **Trait aliases** (currently a nightly feature) let you define shorthand for one or more traits. For example, on nightly you can write:  

  ```rust
  #![feature(trait_alias)]
  trait ReadWrite = std::io::Read + std::io::Write;
  ```  

  This alias means `ReadWrite` stands for anything that is both `Read` and `Write`. According to the Rust Unstable Book, aliases “allow aliases to be created for one or more traits… and used wherever traits would normally be used as either bounds or trait objects” ([Rust trait alias - Stack Overflow](https://stackoverflow.com/questions/62144807/rust-trait-alias#:~:text=The%20unstable%20book%20suggests%20that,emphasis%20added)). However, you **cannot** use an alias in an `impl`; it only works in bounds and object types. The StackOverflow answer sums it up: “trait alias is only meant to be used wherever traits normally used as either bounds or trait objects” ([Rust trait alias - Stack Overflow](https://stackoverflow.com/questions/62144807/rust-trait-alias#:~:text=trait%20alias%20is%20only%20mean,to%20be)). Thus `impl ReadWrite for MyType {}` is not allowed – aliases are purely syntactic sugar for use in signatures and `dyn` types on nightly.  

## Iterators and Real-World Uses  
- The standard library’s `Iterator` trait (with associated type `Item`) is a canonical example: iterators implement shared behavior (`next`) and come with many default adapter methods. Generic bounds on traits are used extensively in crates. For instance, `serde` defines `Serialize` and `Deserialize` traits so any type implementing them can be serialized; users typically write `#[derive(Serialize, Deserialize)]` on structs. The `tokio` crate uses traits like `Future`, `AsyncRead`, and `Stream` to abstract asynchronous tasks and I/O. The `async-trait` crate itself is a practical example of using traits and trait bounds to enable async methods on traits. In all these cases, understanding how to define traits, use bounds, and reason about objects vs generics is crucial.  

**References:** The above points draw on the Rust Book and Reference for trait syntax ([Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=Here%2C%20we%20declare%20a%20trait,String)) ([Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#object-safety#:~:text=,mut%20self)), the Rust language blog for GATs and async in traits ([Generic associated types to be stable in Rust 1.65 | Rust Blog](https://blog.rust-lang.org/2022/10/28/gats-stabilization/#:~:text=As%20of%20Rust%201,to%20remove%20in%20the%20future)) ([Announcing `async fn` and return-position `impl Trait` in traits | Rust Blog](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits/#:~:text=The%20Rust%20Async%20Working%20Group,in%20traits)), and community Q&A for blanket impls and trait aliases ([What are "Blanket Implementations" in Rust? - Stack Overflow](https://stackoverflow.com/questions/73861891/what-are-blanket-implementations-in-rust#:~:text=A%20blanket%20implementation%20is%20an,trait%20on%20a%20generic%20parameter)) ([Rust trait alias - Stack Overflow](https://stackoverflow.com/questions/62144807/rust-trait-alias#:~:text=The%20unstable%20book%20suggests%20that,emphasis%20added)), among others.