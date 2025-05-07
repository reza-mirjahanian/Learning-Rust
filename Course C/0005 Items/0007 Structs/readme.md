https://doc.rust-lang.org/reference/items/structs.html


A *unit-like struct* is a struct without any fields, defined by leaving off the list of fields entirely. Such a struct implicitly defines a [constant](https://doc.rust-lang.org/reference/items/constant-items.html) of its type with the same name. For example:

```
struct Cookie;
let c = [Cookie, Cookie {}, Cookie, Cookie {}];

```
is equivalent to

```

struct Cookie {}
const Cookie: Cookie = Cookie {};
let c = [Cookie, Cookie {}, Cookie, Cookie {}];


```

The precise memory layout of a struct is not specified. One can specify a particular layout using the [`repr` attribute](https://doc.rust-lang.org/reference/type-layout.html#representations).