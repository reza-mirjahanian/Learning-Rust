https://doc.rust-lang.org/reference/statements.html


A *statement* is a component of a [block](https://doc.rust-lang.org/reference/expressions/block-expr.html), which is in turn a component of an outer [expression](https://doc.rust-lang.org/reference/expressions.html) or [function](https://doc.rust-lang.org/reference/items/functions.html).

Rust has two kinds of statement: [declaration statements](https://doc.rust-lang.org/reference/statements.html#declaration-statements) and [expression statements](https://doc.rust-lang.org/reference/statements.html#expression-statements).


[Declaration statements](https://doc.rust-lang.org/reference/statements.html#declaration-statements)
----------------------------------------------------------------------------------------------------

A *declaration statement* is one that introduces one or more *names* into the enclosing statement block. The declared names may denote new variables or new [items](https://doc.rust-lang.org/reference/items.html).

The two kinds of declaration statements are item declarations and `let` statements.

### [Item declarations](https://doc.rust-lang.org/reference/statements.html#item-declarations)



An *item declaration statement* has a syntactic form identical to an [item declaration](https://doc.rust-lang.org/reference/items.html) within a [module](https://doc.rust-lang.org/reference/items/modules.html).



Declaring an item within a statement block restricts its [scope](https://doc.rust-lang.org/reference/names/scopes.html) to the block containing the statement. The item is not given a [canonical path](https://doc.rust-lang.org/reference/paths.html#canonical-paths) nor are any sub-items it may declare.



The exception to this is that associated items defined by [implementations](https://doc.rust-lang.org/reference/items/implementations.html) are still accessible in outer scopes as long as the item and, if applicable, trait are accessible. It is otherwise identical in meaning to declaring the item inside a module.



There is no implicit capture of the containing function's generic parameters, parameters, and local variables. For example, `inner` may not access `outer_var`.

```
fn outer() {
  let outer_var = true;

  fn inner() { /* outer_var is not in scope here */ }

  inner();
}

```