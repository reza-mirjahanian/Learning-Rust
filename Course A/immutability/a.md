Mutable objects are the new spaghetti code. And by that, I mean that you, eventually, with mutable objects, create an intractable mess. And encapsulation does not get rid of that. Encapsulation just means: well, I am in charge of this mess. But the real mess comes from this network that you create of objects that can change, and your inability to look at the state of a system and understand how it got there, how to get it there to test it next time. So it is hard to understand a program where things can change out from underneath you. --- [Rick Hickey](https://github.com/matthiasn/talk-transcripts/blob/master/Hickey_Rich/ClojureConcurrency.md)

------------
A large fraction of the flaws in software development are due to programmers not fully understanding all the possible states their code may execute in.
--- [John Carmack](http://www.sevangelatos.com/john-carmack-on/)

--------

### [Move instead of `mut`]

Lean into Rust's ownership model to avoid mutable state.

It is safe to move variables into functions and structs, so use that to your advantage. This way you can avoid `mut` in many cases and avoid copies, which is especially important for large data structures.

[Don't Be Afraid Of Copying Data.]
----------------------------------------------------------------------------------------------------------

If you have the choice between a lot of `mut` and a few `.clone()` calls, copying data is not as expensive as you might think.

As computers get more cores and memory becomes cheaper, the benefits of immutability outweigh the costs: especially in distributed systems, synchronization and coordination of mutable data structures is hard and has a runtime cost. Immutability can help you avoid a lot of headaches.

[Don't worry about a few `.clone()` calls here and there.](http://xion.io/post/code/rust-borrowchk-tricks.html) Instead, write code that is easy to understand and maintain.

The alternative is often to use locks and these have a runtime cost, too. On top of that, they are a common source of deadlocks.

[Immutability Is A Great Default]
---------------------------------------------------------------------------------------------------------

Immutable code is easier to test, parallelize, and reason about. It's also easier to refactor, because you don't have to worry about side effects.

Rust pushes you towards immutability and offers `mut` as an opt-in escape hatch for hot paths and tight loops. Many (perhaps most) other languages do the exact opposite: they use mutability as the default and require you to consciously choose immutability.

[Limit Mutability To Tight Scopes]
-----------------------------------------------------------------------------------------------------------

Good code keeps mutable state short-lived, making it easier to reason about. The use of `mut` should be the exception, not the rule.