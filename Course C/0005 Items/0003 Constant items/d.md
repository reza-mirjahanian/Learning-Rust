


# Const Items in Rust

A **constant item** (`const`) in Rust is an immutable value bound to a name, evaluated at compile time and *inlined* wherever it’s used. It does **not** occupy a unique memory address at runtime. As the Rust Reference explains: “A constant item is an optionally named constant value which is not associated with a specific memory location in the program”. In practice, the compiler replaces each use of a `const` with its computed value, so using `&CONST` generally yields a reference to a temporary (not a single shared address). By contrast, a `static` item has a fixed address and lives in memory.

Below is a structured technical overview of `const` items, covering syntax, behavior, lifetimes, and advanced subtleties.

## Declaration and Syntax

A constant is declared with the `const` keyword, followed by a name, type, and a constant expression, ending in a semicolon. For example:

```rust
// Declare a constant with an explicit type. Naming is SCREAMING_SNAKE_CASE by convention.
const MAX_POINTS: u32 = 100_000;
```

* **Type annotation required**: Unlike `let`, a `const` must always have an explicit type. For example, `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;` is valid, whereas omitting `: u32` would be an error.
* **Immutable**: You cannot use `mut` with a `const`; it is *always* immutable.
* **Naming convention**: By Rust style, constant names use all-caps with underscores (e.g. `MY_CONST`).
* **Optional name (`_`)**: A *free* (module or block) constant may use the placeholder `_` as its name, creating an *unnamed constant*. This is useful in macros to emit constants without naming conflicts. For example:

  ```rust
  const _: i32 = 5;
  const _: i32 = 10; // allowed, both are distinct unnamed constants
  ```

  The compiler treats each `const _ = …;` as a separate constant; identical unnamed constants can even appear multiple times (e.g. via a macro) without conflict.
* **Constant in trait definitions**: In a `trait`, you can declare an associated `const` *without* an initializer (just like declaring a function signature). For example: `trait Foo { const ID: u32; }`. Such a declaration requires implementations (or defaults) to provide a value. Unlike free `const` items, *only* in a trait definition may you omit the expression.

## Namespace, Visibility, and Scope

A `const` defines a name in the **value namespace** of its enclosing module or block. Its scope and visibility follow the normal rules for Rust items:

* **Module-level const**: If declared in a module (or at top-level), it’s visible to that module. It may be marked `pub`, `pub(crate)`, etc., to control external access, just like `fn` or `static` items. By default, it is private to the current module.
* **Block/function-local const**: You can declare `const` inside functions or blocks. Such a `const` is local to that block. For example:

  ```rust
  fn compute() {
      const SCALE: f64 = 1.2345;
      let value = 10.0 * SCALE;
      // SCALE cannot be seen outside `compute`
  }
  ```
* **Shadowing**: Constants follow Rust’s shadowing rules like variables, but with a caveat. Because constants are inlined, a `let` binding cannot shadow a `const` of the same name. For example, if `const X: i32 = 10;` is in scope, writing `let X = X + 1;` is interpreted as a pattern match (`let 10 = 10 + 1;`) and errors, rather than binding a new `x`. In practice, to use a different value you must use a different variable name.

## Constant Expressions and Evaluation

The initializer of a `const` item must be a *constant expression*—fully computable at compile time. Rust’s compiler will evaluate it during compilation (CTFE: Compile-Time Function Evaluation). Key points:

* **Compile-time evaluation**: All free `const` items are evaluated at compile time, even if never used. If the expression contains a panic or assertion, the program fails to compile at that point.
* **Allowed operations**: The set of allowed operations in `const` expressions is limited but growing: integer and float arithmetic, boolean operations, comparisons, certain pointer and raw memory operations, indexing, array and tuple construction, control flow (`if`, `match`, loops with constant bounds), and calls to `const fn`. As of recent Rust versions, even heap-allocation types (e.g. `Vec`, `String`, `Box`) are allowed if they have `const fn` constructors (Nightly feature). For details see the \[Rust Reference on constant evaluation].
* **No runtime-only values**: You cannot use values only known at runtime. For example, `const N: i32 = std::env::var("X").unwrap().parse().unwrap();` is invalid. Only `const fn` calls or literals are allowed.
* **Constant in trait vs free**: A trait’s associated `const` default or declaration is *not* evaluated until used. Definitions of associated constants (including defaults) only run CTFE when the constant is referenced. In contrast, free `const` items are evaluated immediately at compile time.

## Type and Lifetime Requirements

By definition, all `const` values live for the entire program (“`'static`” lifetime). This imposes strict lifetime rules on `const` items:

* **Type must be `'static`**: Any reference type in a `const` must have a `'static` lifetime. In fact, “the only lifetime allowed in a constant is `'static`”. For example, `const GREETING: &str = "hello";` is effectively `&'static str` because string literals are `'static`. If you write `const T: &T = ...;` without a lifetime, it is assumed `'static` by elision.
* **References in initializer**: All references used inside the initializer must also be `'static`. This means you cannot, for example, take a reference to a stack-local value in a `const`.
* **Promotion of Rvalues**: Rust’s compiler may *promote* certain rvalues (temporaries) to have `'static` storage for the sake of constants. If a constant value is eligible for promotion, then taking a reference to it yields a `'static` reference. Otherwise, the compiler creates a temporary. As the Reference states: “A reference to a constant will have `'static` lifetime if the constant value is eligible for promotion; otherwise, a temporary will be created”.
* **No mutable references**: Critically, the final value of a `const` must not include *any* reference to mutable data. In other words, you cannot have a `const` that borrows a `&mut T` (or contains an `UnsafeCell`) in its final value. This ensures that the immutable const value cannot alias mutable memory.

## Inlining and Memory Representation

Because constants are inlined, there is no single memory location for a `const` item. The compiler essentially substitutes the constant’s value at each use. Concretely:

* **No unique address**: Two usages of the same `const` may end up with distinct copies or addresses. As the Reference notes, “references to the same constant are not necessarily guaranteed to refer to the same memory address”. This contrasts with `static`, where all references see the same address.
* **Compiler behavior**: In practice, small constants often compile to immediate literals or registers. Larger constants (e.g. big arrays) may be placed in read-only data. With optimization, compilers often share the data (e.g. in `.rodata`), but this is an optimization, not a language guarantee. Without optimizations, code may copy a large constant value onto the stack for each use. For very large data, the Rust Reference explicitly recommends using a `static` instead of a `const`, since statics ensure a single memory allocation and avoid potential stack bloat or repeated copies.
* **Drop and use**: When you use a `const` that is a type with a destructor (`Drop`), each use creates a fresh temporary value which will be dropped at the end of that scope. In fact, each reference to a `const` yields its own value: “A `const` item’s destructor will run at each point where the `const` item is used. If a `const` item is never used, its destructor will never run”. By contrast, a `static` with destructors would never drop (until program exit, and even then often not).

## Attributes and Modifiers

`const` items, like other items, may have attributes and visibility modifiers:

* **Visibility**: You can mark a constant `pub`, `pub(crate)`, etc. Just like functions or types, a `pub const` is accessible wherever its parent module is accessible.
* **No `mut` or `unsafe`**: You cannot declare a `const mut` or an `unsafe const`; `const` values are inherently immutable and don’t require `unsafe`.
* **Naming attributes**: You may attach attributes such as documentation comments (`///` or `#[doc]`), `#[deprecated]`, `#[allow]`, `#[cfg]`, etc. For example:

  ```rust
  #[deprecated(note = "Use NEW_CONST instead")]
  pub const OLD_CONST: i32 = 42;
  ```

  will generate a warning if `OLD_CONST` is used, just as with other items.
* **No linkage attributes**: `#[no_mangle]`, `#[link_name]`, or `extern` are pointless on a `const`, since it has no symbol to export. In fact, Rust lints against `#[no_mangle] const ...` and advises using a `static` instead.
* **Naming convention**: By convention, constant names use `SCREAMING_SNAKE_CASE`, matching Rust’s style guidelines.

## Associated Constants (in Traits and Impl)

Rust also supports **associated constants** on types and traits. These are declared with `const` inside an `impl` or a `trait`:

* **Trait declarations**: A trait can declare an associated constant (optionally with a default value). For example:

  ```rust
  trait Shape {
      const SIDES: u32;           // no default
      const MAX_SIDES: u32 = 10;  // with default
  }
  ```

  This does *not* provide a value by itself (unless default is given); each implementation must define it (or use the default). The declaration in the trait can omit the initializer, which is only allowed in trait definitions.
* **Impl definitions**: An `impl` for the trait provides the constant’s value, similar to a free `const` item. For example:

  ```rust
  struct Triangle;
  impl Shape for Triangle {
      const SIDES: u32 = 3;
      // MAX_SIDES uses the default 10
  }
  ```
* **Default values**: If a trait provides a default (e.g. `const ID: i32 = 1;`), any type implementing the trait but not overriding that const gets the default value. In the example above, if `Shape::MAX_SIDES` defaults to 10 and an impl omits it, the constant `Type::MAX_SIDES` is 10.
* **Evaluation semantics**: Associated constants behave slightly differently from free ones. Importantly, their values are **not** evaluated at definition time; they are only evaluated when used. The Rust Reference notes: “Associated constant definitions undergo constant evaluation only when referenced”. Moreover, if the associated constant is generic, it is evaluated after monomorphization. For example, an impl of `const PANIC: () = panic!();` inside a type won’t cause a compile failure until `Type::PANIC` is actually accessed.
* **Usage**: Access them like `Type::CONST_NAME` or `<T as Trait>::CONST`. They obey the same `'static` lifetime and type rules as free `const`s.

## Destructors and Drop Behavior

As of Rust 1.46 (RFC 1440), types with destructors (`Drop`) are allowed in `const` items. Key points:

* **Destructors run on use**: If a `const` has a type that implements `Drop`, its destructor is called when each copy of the constant value goes out of scope. For example:

  ```rust
  struct Foo(u32);
  impl Drop for Foo {
      fn drop(&mut self) { println!("Dropping {}", self.0); }
  }
  const FOO_CONST: Foo = Foo(99);

  fn example() {
      let x = FOO_CONST;
      // At end of scope `x` is dropped, printing "Dropping 99".
  }
  ```
* **No double-drops**: Because each use of `const` yields a fresh value, there is no double-drop. However, if a `const` is never used, its destructor never runs. Also note that destructors in a `static` do *not* run in normal program termination (statically allocated globals are not dropped).
* **Future behavior**: A lint may warn if dropping a `const` implies a leak (since e.g. if you drop resources, multiple runs of program would not match intent).

## Limitations, Gotchas, and Tips

* **Interior mutability**: You cannot take a reference to a `static mut` or any mutable memory inside a `const`. The final value must be immutable. (This is the meaning of the rule “final value cannot contain references to anything mutable”.)
* **No lazy or runtime behavior**: A `const` is **not** lazily initialized; it is fully resolved at compile time. There is no “run this code once” at runtime for a `const`.
* **Inlining effects**: Since constants are inlined, large or complex constants can increase code size. For very large data, consider using a `static` to avoid repeated copies.
* **Cannot `extern` in const**: You cannot write `const FOO: i32 = extern { ... }`. Constants cannot come from external code, only from compile-time expressions.
* **Pattern usage**: A `const` of integer/char type can be used in patterns just like a literal. For example, if `const ZERO: u8 = 0;`, you can write `match x { ZERO => … }` and it behaves like `0`.
* **Shadowing pitfall**: As mentioned, a `let` cannot effectively shadow a `const` of the same name. The constant is replaced by its value, which can lead to confusing errors if you try this.
* **Const generics**: A `const` item can be used in const generics, e.g. `struct ArrayHolder; const N: usize = 5; let arr: [u8; N] = [0; N];`.
* **Promotion subtleties**: Not all expressions are promotable. For example, taking a reference to a struct or array constant with interior pointers may not yield a `'static` ref unless it meets the promotion rules.
* **Lint warnings**: Clippy issues lints like `large_const_arrays` if a constant array is very large, suggesting you might want a `static` instead. Also the compiler lints `no_mangle_const_items` if you mistakenly put `#[no_mangle]` on a const.
* **Doc and naming**: Always document public constants with `///` comments and follow naming conventions (SCREAMING\_SNAKE\_CASE).

## Summary of Key Points

* **Definition**: `const NAME: Type = expr;` is an immutable, compile-time constant value.
* **Type**: Must have `'static` lifetimes for all references in `Type` and in `expr`.
* **Evaluation**: Computed at compile time (Rust CTFE). Free constants are *always* evaluated; associated constants evaluate on use.
* **Inlining**: Constants are inlined. No fixed address; each use may duplicate the value.
* **Immutability**: Cannot mutate; no interior mutability allowed in final value.
* **Visibility**: Respects normal item visibility (`pub`/private) within modules.
* **Destructors**: Allowed. A `Drop` will run for each use of the `const`.
* **Const vs Static**: Prefer `const` for small data and generic contexts; use `static` for large data, single-address requirements, or interior mutability.
* **Official docs**: For full details see the \[Rust Reference on constants] and \[Rust Book].

By understanding these rules and subtleties, you can use `const` items effectively in Rust code, avoiding common pitfalls and leveraging compile-time computation.


