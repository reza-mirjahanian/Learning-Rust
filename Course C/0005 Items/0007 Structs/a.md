

### Structs in Rust: A Comprehensive Guide

#### Overview of Structs in Rust
Structs are custom data types that allow grouping multiple values of potentially different types. These values are referred to as *fields*. Structs are a fundamental part of Rust's type system and are used to model more complex data.

### Types of Structs

1. **Classic Struct**: This is the most basic form of a struct, where each field has a name and a type.
2. **Tuple Struct**: A simpler struct, similar to a tuple, where each field is unnamed, but the types are still specified.
3. **Unit-Like Struct**: A struct without any fields. Often used for types that don't store data but implement traits for behavior.

---

### Classic Struct Example

```rust
struct Person {
    name: String,
    age: u32,
}
```

#### Instantiating and Accessing Fields

```rust
let person = Person {
    name: String::from("Alice"),
    age: 30,
};

// Accessing fields
println!("Name: {}, Age: {}", person.name, person.age);
```

---

### Tuple Struct Example

```rust
struct Point(i32, i32);

let point = Point(3, 4);

// Accessing fields (indexing)
println!("Point: ({}, {})", point.0, point.1);
```

---

### Unit-Like Struct Example

```rust
struct Empty;

let empty = Empty;
```

These structs don't hold data but can be useful when implementing traits or types that don't need fields.

---

### Struct Methods

You can define methods on structs, typically by associating functions with a struct type. The methods can take `&self`, `&mut self`, or `self` as the first parameter, which determines how they can interact with the struct.

#### Example: Methods with `self`, `&self`, and `&mut self`

```rust
struct Circle {
    radius: f64,
}

impl Circle {
    // Method with &self
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // Method with &mut self
    fn set_radius(&mut self, new_radius: f64) {
        self.radius = new_radius;
    }

    // Method with self (ownership transfer)
    fn take_ownership(self) -> f64 {
        self.radius
    }
}
```

#### Creating and Using Methods

```rust
let mut circle = Circle { radius: 5.0 };

// Calling a method with &self
println!("Area: {}", circle.area());

// Calling a method with &mut self
circle.set_radius(10.0);
println!("Updated radius: {}", circle.radius);

// Taking ownership
let radius = circle.take_ownership();
println!("Taken radius: {}", radius);
```

---

### Structs with Default Values

Rust provides a `Default` trait, which can be implemented to provide default values for struct fields. If you implement `Default`, you can initialize a struct using `..Default::default()` syntax.

#### Example: Implementing `Default` for a Struct

```rust
#[derive(Default)]
struct Settings {
    volume: u32,
    brightness: u32,
}

let default_settings: Settings = Default::default();
println!("Default Volume: {}, Brightness: {}", default_settings.volume, default_settings.brightness);
```

---

### Structs with Associated Constants

You can define constants within structs, which are associated with the type and not instances.

#### Example: Associated Constants

```rust
struct MyStruct;

impl MyStruct {
    const CONSTANT_VALUE: u32 = 100;
}

println!("Constant value: {}", MyStruct::CONSTANT_VALUE);
```

---

### Structs and Ownership

When passing a struct to a function, Rust checks ownership rules. By default, structs are passed by value (taking ownership), but you can pass by reference using `&`.

#### Example: Passing by Ownership vs Reference

```rust
struct Book {
    title: String,
    author: String,
}

fn print_book(book: Book) {
    println!("Book: {} by {}", book.title, book.author);
}

let my_book = Book {
    title: String::from("Rust Programming"),
    author: String::from("John Doe"),
};

print_book(my_book);  // Ownership is moved to the function
// println!("{}", my_book.title); // Error: use of moved value
```

To borrow instead of moving:

```rust
fn print_book_ref(book: &Book) {
    println!("Book: {} by {}", book.title, book.author);
}

print_book_ref(&my_book);  // Borrowing
```

---

### Structs and Pattern Matching

Rust allows pattern matching on structs, enabling concise code for extracting data.

#### Example: Pattern Matching with Structs

```rust
struct Person {
    name: String,
    age: u32,
}

fn match_person(person: Person) {
    match person {
        Person { name, age } if age > 18 => println!("{} is an adult.", name),
        Person { name, .. } => println!("{} is a minor.", name),
    }
}

let person = Person { name: String::from("Alice"), age: 20 };
match_person(person);
```

---

### Struct Update Syntax

Rust allows for convenient struct updates, copying most of the fields while modifying specific ones.

#### Example: Struct Update Syntax

```rust
let person1 = Person {
    name: String::from("Alice"),
    age: 30,
};

let person2 = Person {
    age: 25,  // Update age
    ..person1  // Copy other fields
};

println!("Person2: {} ({})", person2.name, person2.age);
```

---

### Struct Visibility and Encapsulation

Rust allows you to control visibility using the `pub` keyword. By default, struct fields are private to the module, but `pub` allows them to be accessed outside the module.

#### Example: Public Struct Fields

```rust
mod my_module {
    pub struct Person {
        pub name: String,
        age: u32,
    }

    impl Person {
        pub fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
    }
}

let person = my_module::Person::new(String::from("Alice"), 30);
println!("Name: {}", person.name);  // Public field is accessible
```

---

### Structs and `derive` Attribute

Rust provides the `#[derive(...)]` attribute to automatically implement certain traits for structs, such as `Clone`, `Debug`, `PartialEq`, etc.

#### Example: Deriving Traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 3, y: 4 };
let p2 = p1.clone();

println!("{:?}", p2); // Debug output
println!("Are points equal? {}", p1 == p2);  // PartialEq
```

#### Commonly Derived Traits:
- `Debug`: Allows printing the struct using `{:?}`.
- `Clone`: Enables creating a deep copy of the struct.
- `PartialEq`: Enables comparison using `==`.
- `Eq`: Requires `PartialEq` and guarantees reflexivity, symmetry, and transitivity.

---

### Pros and Cons of Using Structs

| **Pros**                                 | **Cons**                               |
|------------------------------------------|----------------------------------------|
| **Memory Efficient**: Structs are laid out contiguously in memory. | **No Inheritance**: Rust doesn’t have inheritance like OOP languages. |
| **Customizability**: Structs allow creating complex, tailored types. | **Manual Implementation of Traits**: Sometimes you need to manually implement functionality for structs. |
| **Clear Data Representation**: Fields provide a clear way to represent related data. | **No Default Destructor**: You have to manually implement cleanup when struct is dropped, though this is usually handled by `Drop` trait. |
| **Enforces Data Integrity**: Structs enforce type safety by ensuring data types match. | **More Verbose**: Compared to simpler types like tuples, structs can feel more verbose for small data. |

---

### Performance Considerations

- **Memory Layout**: Structs are packed into contiguous blocks of memory, and Rust ensures that they don't have padding unless necessary. This results in efficient memory usage, especially when fields are of different types.
- **Ownership**: Since structs are passed by value by default, moving a struct to a function can have performance implications, especially for large structs. You may need to pass references (`&T` or `&mut T`) to avoid ownership transfers.
- **Copy vs Move**: If a struct implements the `Copy` trait, it will be copied rather than moved. For types like `i32` and `f64`, this is efficient.

#### Example: Performance Considerations with Ownership

```rust
#[derive(Copy, Clone)]
struct SmallStruct {
    value: i32,
}

let a = SmallStruct { value: 5 };
let b = a;  // Copy, not move
println!("{}", a.value);  // No error
```

---

### Summary

- Structs are a powerful feature in Rust, enabling developers to group related data into custom types.
- You can define methods, implement traits, and control field visibility to suit various needs.
- Be mindful of ownership and borrowing when passing structs around.
- Use `derive` to automatically implement common traits and increase code simplicity.

