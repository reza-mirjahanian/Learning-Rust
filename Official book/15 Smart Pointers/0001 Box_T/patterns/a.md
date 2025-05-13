# Patterns Are Not Expressions
Because They Are Duals

No day can pass on the Rust-Users forum without the ritual of someone asking why |&foo| { ... } or match bar { Some(&foo) => ... } moves the value following the &, whereas we have all been taught that & references (creates a pointer) and definitely does not dereference.

The background of the confusion is syntactic. Patterns and corresponding expressions have identical or similar syntax. Now you might cry **"design error!!!"** at this point. However, these constructs sharing the same syntax, albeit having the _opposite_ behavior, is **the** right choice. Let me explain why.

Expressions are language constructs for _building_ values. You can think of this a bit differently: some of them can be used for _wrapping_ values in other values. For example, given an "atomic" value like the variable foo or the literal 42:

*   Some(foo) and Some(42) produce the values wrapped inside an Option<T>.
*   &foo and &42 produce a pointer to the operand, i.e. it is "wrapped" in one level of indirection. (Containing a value and indirectly referring to one are conceptually very similar relations for the purpose of this discussion.)
*   (foo, 42) packs up the two individual values into a tuple of arity 2, i.e. a pair. Instead of two atomic values, we now wrapped them into a single but compound value containing both.
*   MyStruct { foo } constructs a struct named MyStruct with its field named foo taking ownership of the variable foo. Thus, the value the variable held is now wrapped in a struct.

If we now wish to extract the inner values from the wrappers, there are two possibilities. First off, the most obvious solution is to use different expressions that perform the **inverse** of the wrapping. Namely:

*   Some(foo).unwrap() — we can use a function on the Option type that gives us back the contained value (or panicks if there's nothing inside the optional)
*   \*&foo and \*&42 — we can _dereference_ a reference or pointer in order to get to its "contained" (pointed or referred) value. (This by itself only works with Copy types, because references always need to point to valid values, hence moving out of a reference is not allowed.)
*   (foo, 42).0 yields the value of foo, likewise (foo, 42).1 results in the value 42. This indexed field access syntax allows us to access any component of a tuple and disregard the rest.
*   MyStruct { foo }.foo similarly gives back the value of the field foo, in a sense "removing" the wrapper struct from around it.

However, this is not the only way to peel off layers of wrappers from values. The other, maybe less obvious, possibility is to use **pattern matching**.

Pattern matching means that given a (wrapped) value, we **describe the shape of the whole (wrapped) value**. We then "give names" (the technical term is to create **bindings**) to only the parts that we are interested in. Concretely, this could look like:

```
    match Some(42) {
        Some(value) => println!("the value is {}", value),
        None => println!("no value"),
    }
```


for checking an Option, or

```
    let MyStruct { foo } = my_struct_value;
```


for ripping apart a struct, after which you can use the binding foo just like any other variable, and it will contain the value of the field my\_struct\_value.foo. Or you could do something similar with tuples, too:

```
    let (_, forty_two) = (foo, 42);
```


which in turn discards the first tuple field and binds the value of the second field to the variable forty\_two. Finally, the same works for pointers/references as well:

```
    let &forty_two = &42;
```


which, again, copies the value 42 to the variable forty\_two.

#### The point is: when you are using pattern matching:

*   you describe the overall shape of the value _using a pattern_ on the left-hand-side of the equality or in the "case arms" of a match expression;
*   while the right-hand-side of the equality, or the so-called discriminator expression of the match will be the _value_ or _expression_ that you are matching against (or, as it's sometimes called, destructuring).

So what I'm trying to say here is that **patterns are not expressions.** Instead, they are a sort of **dual** to expressions (in the [loose but mathematical sense](https://math.stackexchange.com/questions/1518509/what-does-dual-mean-exactly-in-mathematics)): they look similar but perform the inverse operation. Or you can think of them as performing the same operation, but "from the inside out."

Patterns are everywhere in Rust, so it's important to understand them. In particular, they can occur:

*   in let statements
*   in if let expressions
*   in match expressions
*   and even in function and closure argument position.

The last one can perhaps be surprising. This means that you can write, for example:

```
    fn function_taking_my_struct(MyStruct { foo }: MyStruct) {
        println!("foo = {}", foo);
    }
```


then call it with a value of type MyStruct:

```
    function_taking_my_struct(MyStruct { foo: 1337 });
```


The same is true for closures; one could create and call them like

```
    let closure_1 = |MyStruct { foo }| {
        println!("foo = {}", foo);
    };
    closure_1(MyStruct { foo: 1337 });

    let closure_2 = |&forty_two| {
        println!("forty-two = {}", forty_two);
    };
    closure_2(&42);
```


(Note that while the |...| syntax may be similar to the capturing mode syntax of C++ lambda functions, which use \[square brackets\] to specify whether variables in the environment should be captured by value or by reference, the |...| part in Rust closures is **not** a capture list. It is merely the regular argument list of the closure.)

Furthermore, there are subtypes of patterns called **_refutable_ and _irrefutable_**; this relates to whether it is possible to bind a pattern against a value unconditionally, determining the successful pattern match completely at compile time (these are called irrefutable), or it's only conditionally possible to match the pattern, depending on the runtime state of the value being destructured (these patterns are called refutable). However, for understanding what patterns are, this distinction is not essential.