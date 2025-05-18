
### 1. What are Patterns?

At its core, a **pattern** is a way to describe the "shape" of data. When Rust encounters a value and a pattern, it compares the value against the shape described by the pattern. If the value fits the shape, the match is successful, and Rust might bind parts of the value to variables.

Patterns are composed of combinations of:
*   Literals
*   Destructured arrays, enums, structs, or tuples
*   Variables
*   Wildcards (`_`)
*   Placeholders (`..`)
*   References

### 2. Where are Patterns Used?

Patterns appear in several places in Rust:

*   **`match` expressions:** This is the most prominent place. Each arm of a `match` expression is a pattern.
*   **`let` statements:** Yes, `let x = 5;` uses a pattern! `x` is a pattern that matches anything and binds it to the variable `x`.
*   **Function and closure parameters:** Parameter lists are patterns.
*   **`if let` expressions:** A convenient way to match one pattern.
*   **`while let` conditional loops:** Loop as long as a pattern matches.
*   **`for` loops:** The variable used to iterate over a collection is a pattern.

---

### 3. Literal Patterns

You can match against fixed values directly. These are called **literal patterns**.

*   **Numbers:**
    ```rust
    fn main() {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }

        let y = 99.5;
        match y {
            0.0 => println!("zero"),
            99.5 => println!("almost a hundred!"),
            _ => println!("something else"),
        }
    }
    ```
*   **Characters (`char`):**
    ```rust
    fn main() {
        let c = 'k';
        match c {
            'a' => println!("starts with 'a'"),
            'k' => println!("found 'k'"),
            _ => println!("some other character"),
        }
    }
    ```
*   **String literals (`&str`):**
    ```rust
    fn main() {
        let s = "hello";
        match s {
            "hello" => println!("greeting received"),
            "goodbye" => println!("farewell"),
            _ => println!("unknown message"),
        }
    }
    ```
*   **Booleans (`bool`):**
    ```rust
    fn main() {
        let is_active = true;
        match is_active {
            true => println!("User is active"),
            false => println!("User is inactive"),
        }
    }
    ```

**Edge Case:** Floating point numbers can be tricky due to precision issues. Exact matching with floats is generally discouraged unless you are certain about the values.

---

### 4. Variable Patterns (Binding)

When a pattern consists of a variable name, it matches any value and **binds** that value to the variable. This is how we get values out of `match` arms or in `let` statements.

*   **Basic Binding:**
    ```rust
    fn main() {
        let some_value = Some(5);
        let some_string = Some("hello");

        match some_value {
            Some(number) => println!("Got a number: {}", number), // `number` is bound to 5
            None => println!("Got nothing"),
        }

        match some_string {
            Some(text) => println!("Got a string: \"{}\"", text), // `text` is bound to "hello"
            None => println!("Got nothing"),
        }

        let x = 10; // `x` is a pattern that binds to 10. This is an irrefutable pattern.
        println!("x = {}", x);
    }
    ```

*   **Shadowing:** If a variable name used in a pattern already exists in the current scope, the pattern will create a **new** variable with the same name, shadowing the outer one. This new variable is valid only within the scope of the match arm.

    ```rust
    fn main() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(y) => println!("Matched Some(y), y is: {}. Inner y shadows outer y.", y), // This `y` is 5
            None => println!("Matched None, outer y is: {}", y),
        }
        println!("After match, outer y is: {}", y); // This `y` is 10
    }
    ```

*   **Mutability:** You can make bindings mutable within a pattern using `mut`.

    ```rust
    fn main() {
        let mut z = Some(String::from("initial"));
        match z {
            Some(ref mut s) => { // We'll discuss `ref mut` more later, for now focus on `s` being mutable
                s.push_str(" and modified");
                println!("Modified string: {}", s);
            }
            None => (),
        }
        println!("Original (potentially modified) z: {:?}", z);

        let val = 5;
        match val {
            mut num => { // `num` is a new mutable binding
                num += 1;
                println!("num is {}", num); // num is 6
            }
        }
        // println!("{}", num); // Error: num is not in scope here
    }
    ```
    Note: `match mut num` is less common than `match num` and then `let mut num_copy = num;` inside the arm if mutation is needed on a copy. If you need to mutate the original value through the pattern, you'd typically match on a mutable reference.

---

### 5. Wildcard Pattern (`_`)

The wildcard pattern `_` matches any value but **does not bind** it to a variable. It's used to ignore values.

*   **Ignoring Entire Value:**
    ```rust
    fn main() {
        let x = Some(5);
        let y: Result<i32, &str> = Ok(10);

        match x {
            Some(_) => println!("Got Some, but ignoring the value."),
            None => println!("Got None."),
        }

        match y {
            Ok(_) => println!("Operation successful, value ignored."),
            Err(_) => println!("Operation failed, error ignored."),
        }
    }
    ```

*   **Ignoring Parts of a Value:**
    ```rust
    fn main() {
        let tuple = (1, 2, 3);
        match tuple {
            (first, _, third) => println!("First: {}, Third: {}", first, third), // Ignores the middle element
        }

        struct Point { x: i32, y: i32, z: i32 }
        let p = Point { x: 0, y: 7, z: 12 };
        match p {
            Point { x, y: _, z } => println!("Point: x={}, z={}", x, z), // Ignores y
        }
    }
    ```

*   **Function Signatures (Ignoring Unused Parameters):**
    Prefixing a parameter name with `_` silences the "unused variable" warning. If you don't intend to use the parameter at all, you can just use `_`.
    ```rust
    fn main() {
        fn process_event(_event_id: i32, data: &str) { // _event_id is ignored, no warning
            println!("Processing data: {}", data);
        }
        process_event(123, "important stuff");

        fn handle_callback(_: String) { // Parameter completely ignored
            println!("Callback handled, data discarded.");
        }
        handle_callback("some data".to_string());
    }
    ```

*   **Ensuring Exhaustiveness in `match`:** The wildcard is crucial for making `match` expressions exhaustive when you don't want to handle every possible case explicitly.
    ```rust
    fn main() {
        let number = 7;
        match number {
            1 => println!("One!"),
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"), // `|` for OR patterns
            _ => println!("A non-prime or a prime I don't care about"),
        }
    }
    ```

---

### 6. Range Patterns (`..=`)

You can match against a continuous range of values using the `start..=end` syntax (inclusive range). This is primarily for numeric types and `char`.

*   **Numeric Ranges:**
    ```rust
    fn main() {
        let x = 5;
        match x {
            1 ..= 5 => println!("between one and five (inclusive)"),
            6 ..= 10 => println!("between six and ten (inclusive)"),
            _ => println!("something else"),
        }

        let temperature = -5;
        match temperature {
            -10 ..= 0 => println!("Freezing!"),
            1 ..= 15 => println!("Cool."),
            16 ..= 30 => println!("Warm."),
            _ => println!("Extreme temperature!"),
        }
    }
    ```

*   **Character Ranges:**
    ```rust
    fn main() {
        let c = 'g';
        match c {
            'a' ..= 'm' => println!("In the first half of the alphabet"),
            'n' ..= 'z' => println!("In the second half of the alphabet"),
            'A' ..= 'Z' => println!("Uppercase letter"),
            '0' ..= '9' => println!("Digit"),
            _ => println!("Something else"),
        }
    }
    ```

**Important:**
*   Range patterns are only allowed for `char` and integer types.
*   The compiler checks that the range is not empty (e.g., `5..=1` would be an error).

**Note on `..` (Exclusive Range Operator):** While `..` is used for creating `Range` and `RangeFrom` etc., for *patterns*, `start..end` (exclusive) is **not** directly supported for numbers in `match` arms in the same way `..=` is. `..` *is* used in slice/array patterns to mean "the rest of the elements", which we'll see next.

---

### 7. Destructuring Patterns

Destructuring allows you to break down structs, enums, tuples, and arrays/slices into their constituent parts.

#### a. Destructuring Structs

You can match against the fields of a struct.

*   **Matching Fields by Name:**
    ```rust
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x: 0, y: 0 } => println!("On the origin"),
            Point { x, y: 0 } => println!("On the x-axis at {}", x), // Binds p.x to x
            Point { x: 0, y } => println!("On the y-axis at {}", y), // Binds p.y to y
            Point { x, y } => println!("Somewhere else: ({}, {})", x, y), // Binds p.x to x, p.y to y
        }
    }
    ```

*   **Shorthand Field Patterns:** If the variable name you want to bind to is the same as the field name, you can use a shorthand:
    ```rust
    struct Config {
        port: u16,
        host: String,
        active: bool,
    }

    fn main() {
        let config = Config { port: 8080, host: "localhost".to_string(), active: true };

        match config {
            // `port` is shorthand for `port: port`
            // `host` is shorthand for `host: host`
            Config { port, host, active: true } => {
                println!("Active config on {}:{}", host, port);
            }
            Config { port, host, active: false } => {
                println!("Inactive config on {}:{}", host, port);
            }
        }
    }
    ```

*   **Ignoring Remaining Fields with `..`:**
    ```rust
    struct User {
        id: u32,
        username: String,
        email: String,
        is_admin: bool,
    }

    fn main() {
        let user1 = User {
            id: 1,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            is_admin: true,
        };

        match user1 {
            User { id, is_admin: true, .. } => { // `..` ignores username and email
                println!("Admin user with ID: {}", id);
            }
            User { id, .. } => { // `..` ignores username, email, and is_admin
                println!("Regular user with ID: {}", id);
            }
        }
    }
    ```

#### b. Destructuring Enums

Enums are often used with `match` to handle their different variants.

*   **Matching Variants and Extracting Values:**
    ```rust
    enum Message {
        Quit,
        Write(String),
        Move { x: i32, y: i32 },
        ChangeColor(i32, i32, i32),
    }

    fn main() {
        let msg1 = Message::Write("Hello, Rust!".to_string());
        let msg2 = Message::Move { x: 10, y: 20 };
        let msg3 = Message::Quit;
        let msg4 = Message::ChangeColor(255, 0, 128);

        fn process_message(msg: Message) {
            match msg {
                Message::Quit => println!("Quit instruction received."),
                Message::Write(text) => println!("Text message: {}", text),
                Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
                Message::ChangeColor(r, g, b) => println!("Change color to R:{}, G:{}, B:{}", r, g, b),
            }
        }

        process_message(msg1);
        process_message(msg2);
        process_message(msg3);
        process_message(msg4);
    }
    ```

#### c. Destructuring Tuples

Tuples can be destructured by their position.

```rust
fn main() {
    let pair = (0, -2);
    match pair {
        (0, y) => println!("First is 0, y = {}", y),
        (x, 0) => println!("x = {}, second is 0", x),
        _ => println!("It doesn't matter what they are"),
    }

    let triple = ("ID", 42, true);
    match triple {
        (label, number, flag) => {
            println!("Label: {}, Number: {}, Flag: {}", label, number, flag);
        }
    }
}
```

#### d. Destructuring Arrays and Slices

*   **Fixed-size Arrays:** You can match arrays with a known size.
    ```rust
    fn main() {
        let arr: [i32; 3] = [1, 2, 3];
        match arr {
            [1, second, third] => println!("Starts with 1, then {}, {}", second, third),
            [x, y, z] => println!("Array: [{}, {}, {}]", x, y, z),
            // _ => println!("Unexpected array content or size"), // Not needed if all variants match size
        }

        let arr2: [i32; 1] = [10];
        match arr2 {
            [10] => println!("Single element 10"),
            _ => println!("Something else"),
        }
    }
    ```

*   **Slices (`&[T]`):** Slice patterns are more flexible.
    *   `..` (the "rest" pattern) can be used to match an unspecified number of elements. It can only be used once per array/slice pattern.
    *   `ref` and `@` can be very useful here.

    ```rust
    fn main() {
        let numbers: &[i32] = &[1, 2, 3, 4, 5];

        match numbers {
            [] => println!("Empty slice"),
            [one] => println!("One element: {}", one),
            [first, second] => println!("Two elements: {} and {}", first, second),
            [1, 2, 3, ..] => println!("Starts with 1, 2, 3 and has more elements"),
            [first, .., last] => println!("First: {}, Last: {}", first, last), // `..` matches all elements in between
            [.., 4, 5] => println!("Ends with 4, 5"),
            rest => println!("All other slices: {:?}", rest), // `rest` binds to the whole slice
        }

        let short_slice: &[i32] = &[10];
        match short_slice {
            [first, .., last] => println!("Short slice - First: {}, Last: {}", first, last), // Works even for one element!
            _ => {}
        }

        let another_slice: &[i32] = &[1, 2, 3];
        match another_slice {
            // Using `ref` to get a slice reference for `middle_and_end`
            [first_val, ref middle_and_end @ ..] => {
                println!("First: {}", first_val);
                println!("Middle and End: {:?}", middle_and_end); // middle_and_end is &[2, 3]
            }
            _ => {}
        }
    }
    ```

    **Edge Cases for Slices:**
    *   `[.., only_one, ..]` is not allowed (more than one `..`).
    *   If `..` is used, it must be unambiguous.

---

### 8. Reference Patterns (`&`, `ref`, `ref mut`) and Dereferencing

Handling references in patterns can be subtle, but Rust's "match ergonomics" often make it intuitive.

*   **Matching References (`&`):** If the value you are matching is a reference, you can use `&` in the pattern to destructure the referenced value.
    ```rust
    fn main() {
        let x = &Some(5); // x is &Option<i32>

        match x {
            &Some(value) => println!("Got value from reference: {}", value), // `value` is i32
            &None => println!("Got None from reference"),
        }

        // Match ergonomics often make the `&` optional in simpler cases:
        match x {
            Some(value) => println!("Ergonomics: Got value: {}", value), // `value` is i32
            None => println!("Ergonomics: Got None"),
        }

        let val = &42;
        match val {
            &n => println!("Value is {}", n), // n is i32
        }
    }
    ```

*   **Creating References with `ref` and `ref mut`:**
    Sometimes, you want to get a reference to a part of the value being matched, rather than moving or copying it.
    *   `ref`: Creates an immutable reference.
    *   `ref mut`: Creates a mutable reference.

    ```rust
    fn main() {
        let robot_name = Some("Bender".to_string());

        match robot_name {
            Some(ref name) => { // `name` is &String, not String. `robot_name` is not moved.
                println!("Robot's name is: {}", name);
            }
            None => println!("No name specified"),
        }
        println!("Original robot_name is still available: {:?}", robot_name);


        let mut count = Some(0);
        match count {
            Some(ref mut number) => { // `number` is &mut i32.
                *number += 1;
                println!("Count incremented to: {}", number);
            }
            None => println!("Count is None"),
        }
        println!("Original count: {:?}", count); // count is Some(1)
    }
    ```
    **When is `ref` needed?**
    When you're matching a value directly (not a reference to it), but you want to bind a variable to a *reference* to one of its fields/parts, rather than moving or copying that part. This is especially important for types that don't implement `Copy`.

*   **Dereferencing with `*`:** You can use `*` in patterns to dereference pointers, but this is less common in idiomatic Rust due to match ergonomics and `ref`.

    ```rust
    fn main() {
        let x = Box::new(5);
        match *x { // Dereference the Box<i32> to match the inner i32
            val if val > 0 => println!("Positive value: {}", val),
            _ => println!("Other value"),
        }
    }
    ```
    Match ergonomics significantly simplify how references are handled. The compiler will often try to insert `&`, `&mut`, or `*` implicitly to make the pattern match the type of the value being matched.

    **Table: `ref` vs `&` in patterns**

    | Scenario                                     | Pattern without `ref`/`&`       | With `&` in pattern | With `ref` in pattern                 | Variable Type | Behavior                                                              |
    | -------------------------------------------- | ------------------------------- | ------------------- | ------------------------------------- | ------------- | --------------------------------------------------------------------- |
    | `let v = Value; match v { ... }`             | `Variant(x)`                    | N/A                 | `Variant(ref x)`                      | `&FieldType`  | Binds `x` as a reference to the field inside `v`. `v` is not moved. |
    | `let v = Value; match v { ... }`             | `Variant(x)`                    | N/A                 | `Variant(x)` (if `FieldType` is `Copy`) | `FieldType`   | Binds `x` as a copy of the field. `v` might be partially moved.     |
    | `let v = Value; match v { ... }`             | `Variant(x)`                    | N/A                 | `Variant(x)` (if `FieldType` not `Copy`)| `FieldType`   | Binds `x` by moving the field out of `v`. `v` is (partially) moved. |
    | `let r = &Value; match r { ... }`            | `Variant(x)` (ergonomics)       | `&Variant(x)`       | Not idiomatic here, `Variant(ref x)` better | `FieldType` or `&FieldType` (depends) | `x` gets the value (or ref) from behind `r`. |
    | `let r = &Value; match r { ... }`            | `Variant(ref x)` (explicit ref) | `&Variant(ref x)`   | `Variant(ref x)` inside `&Variant(...)` | `&FieldType`  | `x` is explicitly a reference to the field.                         |

    The key takeaway for `ref` is: use it when you are matching an owned value, but you want to borrow a part of it instead of moving it.

---

### 9. Match Guards

A **match guard** is an additional `if` condition that can be added to a `match` arm. The pattern must match *and* the guard condition must be true for the arm to be chosen.

*   **Basic Guard:**
    ```rust
    fn main() {
        let num = Some(4);
        match num {
            Some(x) if x < 0 => println!("Negative number: {}", x),
            Some(x) if x % 2 == 0 => println!("Even number: {}", x),
            Some(x) => println!("Odd positive number: {}", x),
            None => println!("No number"),
        }

        let pair = (2, -2);
        match pair {
            (x, y) if x == y => println!("Equal: {} == {}", x, y),
            (x, y) if x + y == 0 => println!("Sum to zero: {} + {} = 0", x, y),
            (x, y) => println("No specific property: ({}, {})", x, y),
        }
    }
    ```

*   **Guards and Ownership:** Variables bound in the pattern are available in the guard expression. The guard does not take ownership of these variables.
    ```rust
    fn main() {
        let name: Option<String> = Some("Rustacean".to_string());

        match name {
            Some(s) if s.len() > 5 => println!("Long name: {}", s), // `s` is borrowed by the guard
            Some(s) => println!("Short name: {}", s),              // `s` is moved here (or copied if String were Copy)
            None => println!("No name"),
        }
        // If the first arm was chosen, `name` would still be Some(String) because the guard only borrows `s`.
        // If the second arm was chosen (and String were not Copy), `name` would be moved.
        // However, with String, it's moved in either Some arm.
    }
    ```
    To avoid moving the `String` if you need it later, use `ref` in the pattern:
    ```rust
    fn main() {
        let name: Option<String> = Some("Rustacean".to_string());

        match name {
            Some(ref s) if s.len() > 5 => println!("Long name: {}", s), // s is &String
            Some(ref s) => println!("Short name: {}", s),              // s is &String
            None => println!("No name"),
        }
        println!("Name after match: {:?}", name); // `name` is still valid
    }
    ```

*   **Order Matters:** Match arms are checked in order. A later arm with a guard might not be reached if an earlier pattern (even without a guard) matches.

    ```rust
    fn main() {
        let number = Some(4);
        match number {
            Some(x) => println!("Matched Some(x): {}", x), // This arm will always match if number is Some
            // Some(x) if x % 2 == 0 => println!("This will never be reached"), // Dead code
            None => (),
        }

        // Corrected order:
        match number {
            Some(x) if x % 2 == 0 => println!("Even number: {}", x),
            Some(x) => println!("Odd number: {}", x),
            None => (),
        }
    }
    ```

---

### 10. `@` Bindings (Subpattern Binding)

The `@` symbol (pronounced "at") lets you bind a value to a variable while also testing that value against a further pattern. This is useful when you want the whole value but also need to inspect its parts or ensure it fits a more complex structure.

*   **Basic `@` Binding:**
    ```rust
    struct IdRange {
        min: u32,
        max: u32,
    }

    fn main() {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::ChangeColor(r, g, b) if r == 0 && g > 128 && b > 128 => {
                println!("Low red, high green and blue");
            }
            // Using @ to bind the value while testing it
            Message::Move { x: x_val @ 0..=100, y: y_val @ 50..=100 } => {
                println!("Moving to valid range: x={}, y={}", x_val, y_val);
            }
            Message::Move { x, y } => {
                println!("Moving out of preferred range: x={}, y={}", x, y);
            }
            _ => println!("Other message"),
        }


        let id = 5;
        match id {
            // Bind `id_val` to the value `id` if it's in the range 1..=7
            id_val @ 1 ..= 7 => println!("ID in range 1-7: {}", id_val),
            id_val @ 8 ..= 10 => println!("ID in range 8-10: {}", id_val),
            _ => println!("ID out of range"),
        }
    }
    ```

*   **`@` with Destructuring:**
    ```rust
    enum MyEnum {
        Variant { id: u32, name: String },
        Other,
    }

    fn main() {
        let data = MyEnum::Variant { id: 42, name: "Answer".to_string() };

        match data {
            // `v` gets bound to the whole `MyEnum::Variant { id: 42, name: "Answer" }`
            // while `id_val` gets bound to `42`
            v @ MyEnum::Variant { id: id_val @ 0..=50, .. } => {
                println!("Variant with ID {} found: {:?}", id_val, v);
            }
            MyEnum::Variant { id, .. } => {
                println!("Variant with ID {} (not in special range)", id);
            }
            MyEnum::Other => println!("Other variant"),
        }
    }
    ```

*   **Using `@` with Slice Patterns:**
    ```rust
    fn main() {
        let numbers: &[i32] = &[1, 2, 3, 4, 5];

        match numbers {
            // `all @` binds the entire slice that matches `[first, ..]` to `all`
            all @ [first, ..] if first == 1 => {
                println!("Slice starts with 1: {:?}, entire matched part: {:?}", first, all);
            }
            // `sub_slice @` binds the part of the slice `[a,b]` to `sub_slice`
            [_, sub_slice @ ..] if sub_slice.len() == 2 => { // This is a bit contrived.
                 // Actually, the pattern `[_, a, b]` would be simpler here if you only want two elements.
                 // This demonstrates binding a sub-slice matched by `..`.
                 // For `[1,2,3,4,5]`, `_` is 1, `sub_slice` is `&[2,3,4,5]`.
                 // For this arm to match, `sub_slice` must have length 2.
                 // Example: numbers = &[0, 10, 20]; then _ is 0, sub_slice is &[10, 20]
                 println!("A sub-slice of length 2: {:?}", sub_slice);
            }
             // More practical example:
            [first, elements @ ..] => { // elements will be &[2,3,4,5] for numbers
                println!("First: {}, Rest: {:?}", first, elements);
            }
            _ => println!("Other slice"),
        }

        let data = [1,2,3];
        match data {
            // `all_three` will be `[1,2,3]`, `one` is 1, `two` is 2, `three` is 3
            all_three @ [one, two, three] => {
                println!("All: {:?}, one:{}, two:{}, three:{}", all_three, one, two, three);
            }
            _ => {}
        }
    }
    ```

---

### 11. Irrefutable vs. Refutable Patterns

Patterns come in two flavors:

*   **Irrefutable Patterns:** These are patterns that will match for any possible value of the given type. They cannot fail.
    *   Examples: `x` (a variable), `(a, b)`, `Point { x, y }` (when destructuring a known struct type).
    *   Used in:
        *   `let` statements: `let x = 5;` ( `x` is irrefutable)
        *   Function parameters: `fn foo(x: i32)` (`x` is irrefutable)
        *   `for` loops: `for i in 0..10` (`i` is irrefutable)

    ```rust
    fn main() {
        let x = 5; // `x` is an irrefutable pattern
        let (a, b) = (1, 2); // `(a,b)` is irrefutable if the RHS is a tuple
        struct MyStruct { val: i32 }
        let MyStruct { val } = MyStruct { val: 10 }; // `MyStruct { val }` is irrefutable

        // fn print_coordinates((x, y): (i32, i32)) { // (x,y) is irrefutable
        //     println!("Coordinates: ({}, {})", x, y);
        // }
    }
    ```

*   **Refutable Patterns:** These are patterns that can fail to match for some possible value.
    *   Examples: `Some(x)` (when matching an `Option<T>`), `Ok(v)` (when matching a `Result<T, E>`), `1`, `User { name: "Alice", .. }`.
    *   Used in:
        *   `match` arms (except the `_` arm or a variable arm, which are irrefutable within the context of being a catch-all).
        *   `if let` expressions.
        *   `while let` loops.

    ```rust
    fn main() {
        let an_option: Option<i32> = Some(10);

        // `Some(x)` is a refutable pattern because `an_option` could be `None`.
        if let Some(x) = an_option {
            println!("Got value: {}", x);
        }

        // This would be a compile-time error because `Some(y)` is refutable
        // and `another_option` could be `None`, but `let` requires an irrefutable pattern.
        // let Some(y) = an_option; // Error: refutable pattern in local binding

        // However, if you are *certain* it will match, you can do this,
        // but it will panic if it doesn't match:
        // let Some(z) = Some(5) else { panic!("This won't happen"); };
        // More commonly for structs/tuples:
        struct Point { x: i32, y: i32 }
        let p = Point { x: 1, y: 2};
        let Point {x, y} = p; // This is irrefutable because p is known to be a Point
                              // and Point {x,y} matches any Point.

        let val: Result<i32, &str> = Ok(5);
        match val {
            Ok(num) => println!("Ok: {}", num), // Ok(num) is refutable
            Err(e)  => println!("Err: {}", e),  // Err(e) is refutable
        }
    }
    ```

**Key Rule:**
*   `let`, function parameters, and `for` loops can **only** accept irrefutable patterns because the program logic assumes the match will always succeed.
*   `if let`, `while let`, and `match` arms (except for catch-alls like `_` or a plain variable) are designed for refutable patterns, allowing for conditional execution based on whether the pattern matches.

---

### 12. Advanced Pattern Matching Scenarios

#### a. Nested Patterns

Patterns can be nested to any depth, allowing you to match complex data structures.

```rust
fn main() {
    enum Wrapper {
        Content(Option<Result<i32, String>>),
        Empty,
    }

    let data1 = Wrapper::Content(Some(Ok(42)));
    let data2 = Wrapper::Content(Some(Err("Failed".to_string())));
    let data3 = Wrapper::Content(None);
    let data4 = Wrapper::Empty;

    fn process_wrapper(w: Wrapper) {
        match w {
            Wrapper::Content(Some(Ok(number))) => {
                println!("Success with number: {}", number);
            }
            Wrapper::Content(Some(Err(message))) => {
                println!("Failure with message: {}", message);
            }
            Wrapper::Content(None) => {
                println!("Content was None");
            }
            Wrapper::Empty => {
                println!("Wrapper is Empty");
            }
        }
    }

    process_wrapper(data1);
    process_wrapper(data2);
    process_wrapper(data3);
    process_wrapper(data4);

    // Example with nested structs and tuples
    struct Inner { value: i32 }
    struct Outer { data: (String, Option<Inner>) }

    let o = Outer { data: ("info".to_string(), Some(Inner { value: 100 })) };
    match o {
        Outer { data: (label, Some(Inner { value: v @ 50..=150 })) } => {
            println!("Label: '{}', valid inner value: {}", label, v);
        }
        Outer { data: (label, Some(Inner { value })) } => {
            println!("Label: '{}', other inner value: {}", label, value);
        }
        Outer { data: (label, None) } => {
            println!("Label: '{}', no inner value.", label);
        }
    }
}
```

#### b. Patterns in `for` Loops

The variable in a `for` loop is actually a pattern. This is most commonly a simple variable, but it can be a destructuring pattern.

```rust
fn main() {
    let v = vec![(1, 'a'), (2, 'b'), (3, 'c')];

    // Destructuring a tuple in a for loop
    for (number, character) in v {
        println!("Number: {}, Character: {}", number, character);
    }

    let points = vec![
        (0, 0),
        (1, 5),
        (10, -3),
    ];
    for (x,y) in points.iter() { // Iterating over references to tuples
        println!("Point: ({}, {})", x, y);
    }
}
```

#### c. Patterns in Function Parameters (Destructuring)

Function parameters are patterns. You can destructure tuples, structs, etc., directly in the parameter list.

```rust
fn main() {
    // Destructuring a tuple parameter
    fn print_coordinates((x, y): (i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    print_coordinates((10, 20));

    struct Point { x: i32, y: i32 }
    // Destructuring a struct parameter
    fn print_point_x(Point { x, y: _ }: Point) { // `y: _` to ignore y if not used
        println!("Point's x coordinate: {}", x);
    }
    let p = Point { x: 5, y: 15 };
    print_point_x(p); // p is moved here

    // To avoid moving, pass a reference and destructure the reference
    fn print_point_y(Point { x: _, y }: &Point) {
        println!("Point's y coordinate: {}", y); // y is &i32, dereferenced by println!
    }
    let p2 = Point { x: 7, y: 27 };
    print_point_y(&p2);
}
```

#### d. Multiple Patterns (`|` OR operator)

You can match multiple patterns in a single `match` arm using the `|` (OR) operator. All patterns in an OR must bind variables of the same name and type.

```rust
fn main() {
    let x = 3;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        y @ (4 | 5 | 6) => println!("four, five, or six (bound to y: {})", y), // All patterns must bind `y`
        _ => println!("something else"),
    }

    enum Data {
        Small(i32),
        Medium(i32),
        Large(String),
    }

    let d = Data::Small(5);
    match d {
        Data::Small(n) | Data::Medium(n) => { // `n` must be bound in both
            println!("Small or Medium with value: {}", n);
        }
        Data::Large(s) => {
            println!("Large with string: {}", s);
        }
    }

    // Binding constraints:
    // The following would be an error because `a` and `b` are different names/types for the binding.
    // match some_option {
    //     Some(a) | None(b) => {} // ERROR
    // }
    // This is also an error if `a` from the first pattern isn't compatible with `a` from the second
    // match (1, 2) {
    //    (a, 1) | (1, a) => {} // If `a` implies different types or if one doesn't bind `a`
    // }
    // Correct usage if types are consistent:
    let val = Some(1);
    match val {
        Some(x @ 1) | Some(x @ 2) => println!("x is 1 or 2: {}", x),
        _ => {}
    }
}
```

---

### 13. Comparison with Similar Concepts

#### `match` vs. `if/else if/else`

| Feature            | `match`                                                                  | `if/else if/else`                                            |
| ------------------ | ------------------------------------------------------------------------ | ------------------------------------------------------------ |
| **Primary Use**    | Branching on the structure/value of a single expression. Destructuring.    | Branching on arbitrary boolean conditions.                   |
| **Exhaustiveness** | Compiler checks for exhaustiveness (all cases handled).                   | No built-in exhaustiveness check.                            |
| **Destructuring**  | Excellent support for destructuring enums, structs, tuples.              | No direct destructuring; requires manual access.             |
| **Value Return**   | `match` is an expression, can return a value from each arm.              | `if/else` is an expression, can return a value.              |
| **Readability**    | Often more readable for complex branching based on data shape.           | Can become nested and harder to read for many conditions.    |
| **Pattern Types**  | Literals, ranges, variables, wildcards, structs, enums, tuples, `@`, guards. | Boolean expressions.                                         |

**When to use `match`:**
*   When you need to make decisions based on the different variants of an enum.
*   When you want to destructure a value and make decisions based on its parts.
*   When you want the compiler to ensure you've handled all possible cases (exhaustiveness).
*   For complex conditional logic that fits the pattern-matching paradigm well.

**When to use `if/else if/else`:**
*   For simple boolean conditions.
*   When conditions are unrelated or involve multiple variables that don't fit a single "matchable" expression.
*   When pattern matching would be overly verbose for the conditions at hand.

**`if let` as a bridge:**
`if let` combines the conciseness of `if` with one arm of a `match`. It's useful when you only care about one specific pattern and want to ignore others (optionally with an `else` block).

```rust
fn main() {
    let an_option = Some(7);

    // Using match
    match an_option {
        Some(x) => println!("Match: Got {}", x),
        None => println!("Match: Got None"),
    }

    // Using if let
    if let Some(x) = an_option {
        println!("if let: Got {}", x);
    } else {
        println!("if let: Got None");
    }

    // When you only care about one case and not the else:
    if let Some(10) = an_option {
        println!("It's ten!");
    }
    // This is more concise than:
    // match an_option {
    //     Some(10) => println!("It's ten!"),
    //     _ => {} // ignore other cases
    // }
}
```

---

### 14. Best Practices and Common Pitfalls

*   **Strive for Exhaustiveness:** Always ensure your `match` statements are exhaustive. The compiler helps, but think carefully about all cases. Use `_` judiciously for truly unhandled or "don't care" cases.
*   **Readability:**
    *   Keep patterns clear and concise.
    *   For very complex patterns, consider breaking them down or using helper functions.
    *   Order match arms logically, often from most specific to most general.
*   **Avoid Overly Complex Patterns:** If a single pattern becomes too convoluted with many nested parts and guards, it might be a sign that the data structure itself is too complex, or that the logic could be refactored.
*   **`ref` for Borrowing:** Remember to use `ref` (or `ref mut`) when you want to bind to a reference within a pattern from an owned value, especially for non-`Copy` types, to avoid moving parts of the value.
*   **Guards vs. More Patterns:**
    *   If a condition can be expressed by refining the pattern itself (e.g., `Some(0)` vs. `Some(x) if x == 0`), prefer refining the pattern.
    *   Use guards for conditions that are hard or impossible to express directly in the pattern syntax (e.g., `Some(x) if x > y`, where `y` is another variable).
*   **Shadowing Awareness:** Be mindful of variable shadowing within match arms. While powerful, it can sometimes lead to confusion if not carefully managed.
*   **`@` Bindings for Clarity:** Use `@` when you need both the whole value and parts of it, as it can make the intent clearer than re-binding or re-accessing parts.
*   **Understanding `|` (OR) Binding Rules:** When using `|`, ensure all patterns in the OR can bind variables with the same names and compatible types.
*   **`if let` for Simplicity:** Don't hesitate to use `if let` (and `while let`) for simpler cases where a full `match` is overkill. It improves conciseness.

