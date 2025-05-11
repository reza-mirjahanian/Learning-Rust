https://doc.rust-lang.org/reference/items/static-items.html


[
Using Statics or Consts](https://doc.rust-lang.org/reference/items/static-items.html#using-statics-or-consts)
---------------------------------------------------------------------------------------------------------------

It can be confusing whether or not you should use a constant item or a static item. Constants should, in general, be preferred over statics unless one of the following are true:

-   Large amounts of data are being stored.
-   The single-address property of statics is required.
-   Interior mutability is required.