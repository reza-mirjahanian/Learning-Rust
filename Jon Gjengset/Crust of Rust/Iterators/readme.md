https://gist.github.com/jonhoo/dd63b720fa4a220ea845a77e2d75831e


```rust
(0..) // It represents 0 to infinity. You can iterate over it as long as you want (or until overflow).

let evens = (0..).step_by(2).take(5);  // 0, 2, 4, 6, 8

//////////////////


let mut iter = (0..).map(|i| 0..i);

let r0 = iter.next().unwrap();  // 0..0
let r1 = iter.next().unwrap();  // 0..1
let r2 = iter.next().unwrap();  // 0..2
``` 


