# Patterns and Matching in Rust

## Fundamentals of Pattern Matching

Pattern matching is a powerful feature in Rust that allows you to destructure and match values against patterns. It's one of Rust's most distinctive features, combining aspects of both pattern matching from functional languages and traditional control flow.

### Basic Match Expressions

The `match` expression is the most fundamental pattern matching construct:

```rust
fn main() {
    let x = 1;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }
}
```

Key characteristics:
- **Exhaustive**: Must cover all possible values
- **Order matters**: Patterns are checked in sequence
- **Binding**: Can bind variables to matched values

### Pattern Types

Rust supports several pattern types:

1. **Literals**: Match exact values
2. **Variables**: Bind values to names
3. **Wildcards**: Match any value without binding
4. **Ranges**: Match values in a range
5. **OR patterns**: Match multiple patterns
6. **Destructuring**: Break down complex data types
7. **Guards**: Add conditional logic to patterns

## Advanced Pattern Matching

### Matching with Guards

Guards add conditional logic to pattern matching:

```rust
fn main() {
    let x = 4;
    
    match x {
        n if n % 2 == 0 => println!("{} is even", n),
        n => println!("{} is odd", n),
    }
}
```

### Destructuring Structs

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    
    match p {
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x: 0, y } => println!("On the y-axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

### Destructuring Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);
    
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
}
```

### Nested Patterns

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}
```

### Binding with @ Operator

The `@` operator lets you test a value and bind it simultaneously:

```rust
fn main() {
    let x = 5;
    
    match x {
        n @ 1..=5 => println!("Got a number in range 1-5: {}", n),
        n @ 6..=10 => println!("Got a number in range 6-10: {}", n),
        _ => println!("Number out of range"),
    }
}
```

## Pattern Matching Beyond `match`

### `if let` Expressions

`if let` provides concise pattern matching when you only care about one pattern:

```rust
fn main() {
    let some_value = Some(3);
    
    // Verbose match
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
    
    // Concise if let
    if let Some(3) = some_value {
        println!("three");
    }
}
```

### `while let` Loops

Similar to `if let`, but continues looping as long as the pattern matches:

```rust
fn main() {
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

### `for` Loops

The `for` loop also uses pattern matching:

```rust
fn main() {
    let v = vec!['a', 'b', 'c'];
    
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
```

### `let` Statements

Even simple `let` statements use pattern matching:

```rust
fn main() {
    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);
}
```

### Function Parameters

Function parameters can also use patterns:

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

## Refutability: Irrefutable vs. Refutable Patterns

- **Irrefutable patterns** always match (used in `let`, function parameters, `for` loops)
- **Refutable patterns** might not match (used in `if let`, `while let`, `match`)

```rust
fn main() {
    // This works - irrefutable pattern
    let x = 5;
    
    // This would fail to compile - refutable pattern in let
    // let Some(x) = Some(5);
    
    // This works - refutable pattern in if let
    if let Some(x) = Some(5) {
        println!("{}", x);
    }
}
```

## Pattern Syntax Reference

### Matching Literals

```rust
fn main() {
    let x = 1;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }
}
```

### Named Variables

```rust
fn main() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y), // This y shadows the outer y
        _ => println!("Default case, x = {:?}", x),
    }
    
    println!("at the end: x = {:?}, y = {}", x, y);
}
```

### Multiple Patterns

```rust
fn main() {
    let x = 1;
    
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

### Matching Ranges

```rust
fn main() {
    let x = 5;
    
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    
    let c = 'c';
    
    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
```

### Destructuring Structs with Shorthand

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    
    // Full syntax
    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b);
    
    // Shorthand when variable names match field names
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);
}
```

### Ignoring Values

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    
    // Ignore entire values with _
    match numbers {
        (first, _, third, _, _) => {
            println!("Some numbers: {}, {}", first, third)
        }
    }
    
    // Ignore parts of a value with ..
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    
    let origin = Point { x: 0, y: 0, z: 0 };
    
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    
    // Ignore multiple values
    let numbers = (2, 4, 8, 16, 32);
    
    match numbers {
        (first, .., last) => {
            println!("First: {}, Last: {}", first, last);
        }
    }
}
```

### Match Guards

```rust
fn main() {
    let num = Some(4);
    
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}
```

### Binding with @ Operator

```rust
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}
```

## Comparison with Similar Concepts

| Feature | Rust | Swift | Haskell | Scala |
|---------|------|-------|---------|-------|
| Basic Syntax | `match x { ... }` | `switch x { ... }` | `case x of ...` | `x match { ... }` |
| Exhaustiveness | Required | Required | Required | Required |
| Guards | `if` condition | `where` clause | Guards | Guards |
| Binding | `@` operator | Binding names | `@` pattern | `@` pattern |
| Pattern Types | Literals, ranges, structs, enums, tuples | Similar to Rust | Similar to Rust | Similar to Rust |
| Special Forms | `if let`, `while let` | `if case`, `while case` | N/A | `for` comprehensions |

## Advanced Use Cases

### Pattern Matching with References

```rust
fn main() {
    let reference = &4;
    
    match reference {
        &val => println!("Got a value via destructuring: {}", val),
    }
    
    // Alternative approach
    match *reference {
        val => println!("Got a value via dereferencing: {}", val),
    }
}
```

### Creating References in Patterns

```rust
fn main() {
    let value = 5;
    
    match value {
        ref r => println!("Got a reference to {}", r),
    }
    
    let mut mut_value = 6;
    
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {}", m);
        }
    }
}
```

### Matching Custom Types with Complex Patterns

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to RGB({}, {}, {})", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to HSV({}, {}, {})", h, s, v)
        }
        _ => (),
    }
}
```

### Combining Multiple Patterns

```rust
fn main() {
    let x = 4;
    let y = false;
    
    match (x, y) {
        (4, true) => println!("x is 4 and y is true"),
        (4, false) => println!("x is 4 and y is false"),
        (_, true) => println!("x is anything and y is true"),
        (_, false) => println!("x is anything and y is false"),
    }
}
```

## Common Pitfalls and Best Practices

### Exhaustiveness and the Compiler

Rust's compiler ensures all patterns are exhaustive:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Removing this arm would cause a compiler error
        Coin::Quarter => 25,
    }
}
```

### Non-exhaustive Patterns with `_`

```rust
fn main() {
    let dice_roll = 9;
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // Do nothing for other values
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

### Pattern Binding Precedence

Be careful with nested patterns and binding precedence:

```rust
fn main() {
    let x = Some(5);
    
    if let Some(n @ 1..=10) = x {
        println!("Match! n = {}", n);
    } else {
        println!("No match");
    }
}
```

### Performance Considerations

Pattern matching in Rust is generally compiled to efficient code, but complex patterns with many guards can lead to less optimal code. The compiler is good at optimizing simple pattern matching.

## Real-world Applications

### State Machines

Pattern matching is excellent for implementing state machines:

```rust
enum State {
    Start,
    Processing { steps_completed: u32 },
    Success(String),
    Failed(String),
}

fn process_state(state: State) {
    match state {
        State::Start => println!("Starting process"),
        State::Processing { steps_completed } if steps_completed < 3 => {
            println!("In progress: {} steps done", steps_completed)
        }
        State::Processing { steps_completed } => {
            println!("Almost done: {} steps completed", steps_completed)
        }
        State::Success(result) => println!("Success: {}", result),
        State::Failed(error) => println!("Failed: {}", error),
    }
}
```

### Error Handling

Pattern matching works well with Rust's `Result` and `Option` types:

```rust
fn process_file(path: &str) -> Result<String, std::io::Error> {
    let file_result = std::fs::read_to_string(path);
    
    match file_result {
        Ok(contents) if contents.len() > 0 => Ok(contents),
        Ok(_) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "File is empty")),
        Err(e) => Err(e),
    }
}
```

### Command Parsing

```rust
enum Command {
    Quit,
    Move(i32, i32),
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process_command(command: Command) {
    match command {
        Command::Quit => println!("Quitting..."),
        Command::Move(x, y) => println!("Moving to position ({}, {})", x, y),
        Command::Write(s) => println!("Writing: {}", s),
        Command::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}
```

