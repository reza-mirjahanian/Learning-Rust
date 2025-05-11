https://doc.rust-lang.org/reference/attributes/diagnostics.html

[Lint check attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html#lint-check-attributes)
--------------------------------------------------------------------------------------------------------------

A lint check names a potentially undesirable coding pattern, such as unreachable code or omitted documentation.



The lint attributes `allow`, `expect`, `warn`, `deny`, and `forbid` use the [*MetaListPaths*](https://doc.rust-lang.org/reference/attributes.html#meta-item-attribute-syntax) syntax to specify a list of lint names to change the lint level for the entity to which the attribute applies.

For any lint check `C`:


-   `#[allow(C)]` overrides the check for `C` so that violations will go unreported.


-   `#[expect(C)]` indicates that lint `C` is expected to be emitted. The attribute will suppress the emission of `C` or issue a warning, if the expectation is unfulfilled.


-   `#[warn(C)]` warns about violations of `C` but continues compilation.


-   `#[deny(C)]` signals an error after encountering a violation of `C`,

-   `#[forbid(C)]` is the same as `deny(C)`, but also forbids changing the lint level afterwards,