https://doc.rust-lang.org/reference/items/constant-items.html

The constant expression may **only be omitted** in a trait definition.


### Unnamed constant

Unlike an [associated constant](https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants), a [free](https://doc.rust-lang.org/reference/glossary.html#free-item) constant may be unnamed by using an underscore instead of the name. For example:

```ruat
const _: () =  { struct _SameNameTwice; };

// OK although it is the same name as above:
const _: () =  { struct _SameNameTwice; };

```