# Rust Enums: Complete Reference

## Basic Enum Definition

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let direction = Direction::North;
    
    match direction {
        Direction::North => println!("Going north!"),
        Direction::South => println!("Going south!"),
        Direction::East => println!("Going east!"),
        Direction::West => println!("Going west!"),
    }
}
```

## Enums with Data

```rust
enum WebEvent {
    PageLoad,                      // No data
    KeyPress(char),                // Single value
    Click { x: i64, y: i64 },      // Named fields
    ColorChange(i32, i32, i32),    // Multiple values
}

fn main() {
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('c'),
        WebEvent::Click { x: 10, y: 20 },
        WebEvent::ColorChange(255, 0, 0),
    ];
    
    for event in events {
        match event {
            WebEvent::PageLoad => println!("Page loaded"),
            WebEvent::KeyPress(c) => println!("Pressed '{}'", c),
            WebEvent::Click { x, y } => println!("Clicked at ({}, {})", x, y),
            WebEvent::ColorChange(r, g, b) => println!("Color changed to RGB({}, {}, {})", r, g, b),
        }
    }
}
```

## Type-like Enums (C-style Enums)

```rust
enum Color {
    Red = 0xFF0000,
    Green = 0x00FF00,
    Blue = 0x0000FF,
}

fn main() {
    println!("Red value: {}", Color::Red as i32);
    println!("Green value: {}", Color::Green as i32);
    println!("Blue value: {}", Color::Blue as i32);
}
```

## Option Enum

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    // Using map
    let result = divide(10.0, 2.0).map(|x| x * 2.0);
    println!("Result: {:?}", result);  // Some(10.0)
    
    // Using unwrap_or
    let safe_result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("Safe result: {}", safe_result);  // 0.0
    
    // Using if let
    if let Some(value) = divide(10.0, 2.0) {
        println!("Division result: {}", value);
    } else {
        println!("Cannot divide by zero");
    }
    
    // Using and_then (flatMap)
    let chained = divide(10.0, 2.0).and_then(|x| divide(x, 2.0));
    println!("Chained: {:?}", chained);  // Some(2.5)
}
```

## Result Enum

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn parse_number(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(String::from("Failed to parse number")),
    }
}

fn main() {
    // Using ? operator with Result
    fn process_input(input: &str) -> Result<i32, String> {
        let number = parse_number(input)?;
        Ok(number * 2)
    }
    
    // Using map_err
    let result = parse_number("10")
        .map_err(|e| format!("Error: {}", e));
    
    // Using match
    match process_input("42") {
        Ok(n) => println!("Processed: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    
    // Using unwrap_or_else
    let value = parse_number("abc").unwrap_or_else(|e| {
        println!("{}", e);
        -1
    });
}
```

## Methods on Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message received"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
    
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
    ];
    
    for message in messages {
        message.call();
        println!("Is quit? {}", message.is_quit());
    }
}
```

## Generic Enums

```rust
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn main() {
    let left: Either<i32, f64> = Either::Left(5);
    let right: Either<i32, f64> = Either::Right(3.14);
    
    match left {
        Either::Left(value) => println!("Left: {}", value),
        Either::Right(value) => println!("Right: {}", value),
    }
    
    match right {
        Either::Left(value) => println!("Left: {}", value),
        Either::Right(value) => println!("Right: {}", value),
    }
}
```

## Pattern Matching with Enums

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... other states
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    // Basic matching
    let cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value: {} cents", cents);
    
    // Multiple patterns
    let x = 5;
    match x {
        1 | 2 => println!("1 or 2"),
        3..=5 => println!("3 through 5"),
        _ => println!("anything else"),
    }
    
    // Guards in patterns
    let pair = (5, 10);
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 15 => println!("Sum is 15"),
        _ => println!("No match"),
    }
    
    // @ binding
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Move { x: a @ 0..=10, y } => println!("Move with limited x={}", a),
        Message::Move { .. } => println!("Some other move"),
        _ => println!("Something else"),
    }
}
```

## if let / while let

```rust
fn main() {
    let some_value = Some(3);
    
    // Instead of:
    match some_value {
        Some(3) => println!("three!"),
        _ => (),
    }
    
    // You can use:
    if let Some(3) = some_value {
        println!("three!");
    }
    
    // while let example
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
```

## Using #[derive] with Enums

```rust
#[derive(Debug, PartialEq, Clone)]
enum Status {
    Active,
    Inactive,
    Pending,
}

fn main() {
    let status1 = Status::Active;
    let status2 = status1.clone();
    
    println!("Status 1: {:?}", status1);
    println!("Are equal: {}", status1 == status2);
    
    // Custom Debug output
    #[derive(Debug)]
    enum Complex {
        Point { x: i32, y: i32 },
        Vector(i32, i32),
    }
    
    let point = Complex::Point { x: 10, y: 20 };
    println!("{:?}", point);  // Point { x: 10, y: 20 }
}
```

## Non-exhaustive Enums

```rust
// Library code
#[non_exhaustive]
pub enum DatabaseError {
    ConnectionError,
    QueryError,
    // More variants might be added in the future
}

// Client code
fn handle_error(error: DatabaseError) {
    match error {
        DatabaseError::ConnectionError => println!("Connection error"),
        DatabaseError::QueryError => println!("Query error"),
        // No need for _ => {} with #[non_exhaustive]
        // The compiler knows there might be variants we don't handle
        _ => println!("Unknown error"),
    }
}
```

## Recursive Enums

```rust
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    fn sum(list: &List<i32>) -> i32 {
        match list {
            Cons(value, next) => value + sum(next),
            Nil => 0,
        }
    }
    
    println!("Sum: {}", sum(&list));  // 6
}
```

## Memory Layout and Size

```rust
use std::mem::{size_of, align_of};

enum SmallEnum {
    A,
    B,
    C,
}

enum DataEnum {
    A(i32),
    B(f64),
    C { x: i32, y: i32 },
}

fn main() {
    println!("Size of SmallEnum: {} bytes", size_of::<SmallEnum>());
    println!("Alignment of SmallEnum: {} bytes", align_of::<SmallEnum>());
    
    println!("Size of DataEnum: {} bytes", size_of::<DataEnum>());
    println!("Alignment of DataEnum: {} bytes", align_of::<DataEnum>());
    
    // Showing tag storage
    // DataEnum is large enough to store any variant plus a tag (discriminant)
}
```

## Typed Enums (Phantom Types)

```rust
enum Opened {}
enum Closed {}

struct Door<State> {
    room_number: u32,
    _state: std::marker::PhantomData<State>,
}

impl Door<Closed> {
    fn new(room_number: u32) -> Self {
        Door {
            room_number,
            _state: std::marker::PhantomData,
        }
    }
    
    fn open(self) -> Door<Opened> {
        println!("Opening door {}", self.room_number);
        Door {
            room_number: self.room_number,
            _state: std::marker::PhantomData,
        }
    }
}

impl Door<Opened> {
    fn close(self) -> Door<Closed> {
        println!("Closing door {}", self.room_number);
        Door {
            room_number: self.room_number,
            _state: std::marker::PhantomData,
        }
    }
}

fn main() {
    let closed_door = Door::<Closed>::new(42);
    let opened_door = closed_door.open();
    let closed_again = opened_door.close();
    // This would cause a compile error:
    // closed_door.close();
}
```

## Enums vs Structs Comparison

| Feature | Enums | Structs |
|---------|-------|---------|
| Purpose | Represent values that can be one of several variants | Represent a single data structure with named fields |
| Data | Can contain different types of data in each variant | All instances have the same structure |
| Memory | Size is determined by largest variant + tag | Fixed memory layout |
| Use Case | When data can be in different states/forms | When data always has the same shape |
| Pattern Matching | Native support with `match` | Less natural pattern matching |

## Performance Considerations

### Memory Layout and Space Efficiency
- Enums use a tag (discriminant) to indicate the variant
- Size is at least the size of the largest variant + tag size
- Small optimization: unit-only enums can be more compact
- Can lead to inefficient memory usage if variants have very different sizes

### Time Complexity

| Operation | Time Complexity | Notes |
|-----------|-----------------|-------|
| Creating an enum | O(1) | Same as struct creation |
| Variant check | O(1) | Tag comparison is fast |
| Pattern matching | O(1) | Compiled to efficient branching |
| Converting from integer | O(1) | Direct mapping for repr(C)-like enums |

## Advanced Enum Patterns

### Type-State Pattern

```rust
enum Pending {}
enum InProgress {}
enum Completed {}

struct Task<S> {
    id: u32,
    description: String,
    _state: std::marker::PhantomData<S>,
}

impl Task<Pending> {
    fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            _state: std::marker::PhantomData,
        }
    }
    
    fn start(self) -> Task<InProgress> {
        println!("Starting task: {}", self.description);
        Task {
            id: self.id,
            description: self.description,
            _state: std::marker::PhantomData,
        }
    }
}

impl Task<InProgress> {
    fn complete(self) -> Task<Completed> {
        println!("Completing task: {}", self.description);
        Task {
            id: self.id,
            description: self.description,
            _state: std::marker::PhantomData,
        }
    }
}

fn main() {
    let task = Task::<Pending>::new(1, "Learn Rust".to_string());
    let task = task.start();
    let task = task.complete();
    
    // This would fail to compile:
    // task.start();
}
```

### Custom Display Implementation

```rust
use std::fmt;

enum Status {
    Active,
    Inactive,
    Pending,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Active => write!(f, "ACTIVE"),
            Status::Inactive => write!(f, "INACTIVE"),
            Status::Pending => write!(f, "PENDING"),
        }
    }
}

fn main() {
    let status = Status::Active;
    println!("Current status: {}", status);  // ACTIVE
}
```

### Transparent Enums

```rust
#[repr(transparent)]
enum Wrapper(i32);

fn main() {
    let wrapped = Wrapper(42);
    
    // This is useful for FFI where the enum must have the
    // same memory layout as the inner type
}
```

### Compile-Time Enum Size Check

```rust
use std::mem::size_of;

enum Large {
    A([u8; 1024]),
    B(String),
}

const _: () = assert!(size_of::<Large>() <= 2048);

fn main() {
    println!("Size of Large: {} bytes", size_of::<Large>());
}
```

## Tricky Parts and Gotchas

### Missing Exhaustiveness Checks

```rust
enum Animal {
    Dog,
    Cat,
    Bird,
}

fn make_sound(animal: Animal) -> &'static str {
    match animal {
        Animal::Dog => "Woof!",
        Animal::Cat => "Meow!",
        // Missing Bird case - compiler error!
    }
}

// Fix with:
fn make_sound_fixed(animal: Animal) -> &'static str {
    match animal {
        Animal::Dog => "Woof!",
        Animal::Cat => "Meow!",
        Animal::Bird => "Tweet!",
        // Or use _ => "Unknown sound" as a catch-all
    }
}
```

### Type Coercion with Option and Result

```rust
fn main() {
    let nums = vec![Some(1), None, Some(3)];
    
    // This works due to type coercion
    let sum: i32 = nums.iter()
        .filter_map(|&x| x)
        .sum();
    println!("Sum: {}", sum);  // 4
    
    // Common mistake with string types
    let strings: Vec<Option<String>> = vec![Some("hello".to_string()), None];
    
    // This won't compile - &str isn't String
    // let filtered: Vec<String> = strings.iter().filter_map(|x| x).collect();
    
    // Correct way:
    let filtered: Vec<String> = strings.into_iter().filter_map(|x| x).collect();
}
```

### Enum Variant Name Conflicts

```rust
enum Direction {
    Left,
    Right,
}

enum Movement {
    Up,
    Down,
    Left,  // Name collision with Direction::Left
    Right, // Name collision with Direction::Right
}

fn main() {
    // Need to use the full path to disambiguate
    let dir = Direction::Left;
    let mov = Movement::Left;
    
    // Or bring specific variants into scope
    use Direction::Right as DirRight;
    let right = DirRight;
}
```

### Memory Layout Control

```rust
// Default layout (Rust decides)
enum Default {
    A(u8),
    B(u32),
}

// C-like layout (compatible with C enums)
#[repr(C)]
enum CEnum {
    A(u8),
    B(u32),
}

// Integer representations
#[repr(u8)]
enum Byte {
    One = 1,
    Two = 2,
}

fn main() {
    println!("Default size: {}", std::mem::size_of::<Default>());
    println!("C-like size: {}", std::mem::size_of::<CEnum>());
    println!("Byte size: {}", std::mem::size_of::<Byte>());
}
```

### Impl Blocks Limitations

```rust
enum Value {
    Number(i32),
    Text(String),
}

impl Value {
    // General methods for any variant
    fn describe(&self) -> String {
        match self {
            Value::Number(n) => format!("Number: {}", n),
            Value::Text(s) => format!("Text: {}", s),
        }
    }
    
    // NOTE: You cannot have separate impl blocks for each variant
    // This doesn't exist in Rust:
    // impl Value::Number { ... }
}

fn main() {
    let num = Value::Number(42);
    let text = Value::Text("Hello".to_string());
    
    println!("{}", num.describe());
    println!("{}", text.describe());
}
```

## Comparison with Similar Concepts

### Enum vs Union

| Feature | Rust Enum | C Union |
|---------|-----------|---------|
| Type Safety | Safe, tagged union | Unsafe, untagged |
| Memory | Size of largest variant + tag | Size of largest member |
| Variant Tracking | Compiler knows current variant | Programmer must track |
| Pattern Matching | Native support | Not available |
| Memory Safety | Guaranteed | Not guaranteed |

### Enum vs Trait Objects

| Feature | Enum | Trait Objects |
|---------|------|--------------|
| Known Variants | Fixed at compile time | Open for extension |
| Dispatch | Static dispatch | Dynamic dispatch |
| Performance | Faster, no vtable | Slower, uses vtable |
| Memory Layout | Inline, compact | Pointer + vtable |
| Use Case | Closed set of variants | Open/extensible types |

### Rust Enums vs Algebraic Data Types in Other Languages

| Language | Feature | Comparison with Rust Enums |
|----------|---------|----------------------------|
| Haskell | ADTs | Very similar, both support sum types |
| Swift | Enums | Similar, both support associated values |
| TypeScript | Discriminated Unions | Similar but with type annotations |
| C++ | std::variant | Similar but less ergonomic |
| Java | Enum | Rust enums are more powerful |
| Python | None | Closest is inheritance or match-case |