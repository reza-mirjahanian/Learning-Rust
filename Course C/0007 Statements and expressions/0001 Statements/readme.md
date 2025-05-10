https://doc.rust-lang.org/reference/statements.html


A *statement* is a component of a [block](https://doc.rust-lang.org/reference/expressions/block-expr.html), which is in turn a component of an outer [expression](https://doc.rust-lang.org/reference/expressions.html) or [function](https://doc.rust-lang.org/reference/items/functions.html).

Rust has two kinds of statement: [declaration statements](https://doc.rust-lang.org/reference/statements.html#declaration-statements) and [expression statements](https://doc.rust-lang.org/reference/statements.html#expression-statements).


[Declaration statements](https://doc.rust-lang.org/reference/statements.html#declaration-statements)
----------------------------------------------------------------------------------------------------

A *declaration statement* is one that introduces one or more *names* into the enclosing statement block. The declared names may denote new variables or new [items](https://doc.rust-lang.org/reference/items.html).

The two kinds of declaration statements are item declarations and `let` statements.