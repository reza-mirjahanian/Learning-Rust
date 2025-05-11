### **Structs in Rust**

---

#### **Types of Structs**
1. **Unit Struct**  
   ```rust
   struct Point; // No fields
   ```
   - Used as a marker or placeholder.

2. **Tuple Struct**  
   ```rust
   struct Color(i32, i32, i32); // Unnamed fields
   ```
   - Similar to tuples but can have methods and derive traits.

3. **Regular Struct**  
   ```rust
   struct User {
       name: String,
       age: u8,
   }
   ```
   - Named fields for clarity.

---

#### **Creating Instances**
```rust
// Regular
let user = User {
    name: String::from("Alice"),
    age: 30,
};

// Tuple
let color = Color(255, 0, 0);

// Unit
let point = Point;
```

---

#### **Destructuring Structs**
```rust
// Regular
let User { name, age } = user;

// Tuple
let Color(r, g, b) = color;

// Ignore fields
let User { name, .. } = user;
```

---

#### **Deriving Traits**
```rust
#[derive(Debug, PartialEq, Clone)]
struct User {
    name: String,
    age: u8,
}

let user1 = User { .. };
let user2 = user1.clone();
assert_eq!(user1, user2);
println!("{:?}", user1);
```

---

#### **Memory Layout & Alignment**
```rust
#[repr(C)]
struct Padded {
    a: u8,   // 1 byte
    b: u64,  // 8 bytes + 7 bytes padding after `a`
}

assert_eq!(std::mem::size_of::<Padded>(), 16);
```

---

#### **Methods & Associated Functions**
```rust
impl User {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }

    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

let user = User::new(String::from("Bob"), 25);
println!("{}", user.describe());
```

---

#### **Ownership & Borrowing**
```rust
struct RefUser<'a> {
    name: &'a str, // Requires lifetime annotation
}

// Error: dangling reference
// let user = RefUser { name: &String::from("Alice") }; 
let s = String::from("Alice");
let user = RefUser { name: &s }; // Valid
```

---

#### **Pattern Matching**
```rust
match user {
    User { name: "Admin", .. } => println!("Admin detected"),
    _ => println!("Regular user"),
}
```

---

#### **Advanced Topics**
1. **PhantomData**  
   ```rust
   use std::marker::PhantomData;
   struct MyVec<T> {
       data: Vec<T>,
       _marker: PhantomData<T>, // For type parameters without data
   }
   ```

2. **Recursive Structs**  
   ```rust
   struct Node {
       left: Option<Box<Node>>,
       right: Option<Box<Node>>,
   }
   ```

3. **#[repr] for Memory Control**  
   ```rust
   #[repr(transparent)]
   struct Wrapper(u32); // Same layout as u32
   ```

---

#### **Pros/Cons Table**

| **Type**        | **Pros**                          | **Cons**                          |
|------------------|-----------------------------------|-----------------------------------|
| **Unit Struct**  | Simple, zero-sized                | No data storage                   |
| **Tuple Struct** | Concise syntax                    | No field names                    |
| **Regular Struct** | Clear field names, methods     | Slightly verbose                  |

---

#### **Trade-Offs (O())**
- **Memory**: Padding increases size but may improve access speed.
- **Alignment**: `#[repr(C)]` ensures FFI compatibility but may waste space.
- **Copy vs Clone**: `Copy` is O(1), but only for `Sized` types. `Clone` is O(n) for data size.

---

#### **Edge Cases**
1. **Empty Struct**  
   ```rust
   struct Empty; // Valid, size is 0
   ```

2. **Recursive Structs**  
   Require boxing to avoid infinite size.

3. **Generic Structs**  
   ```rust
   struct Point<T> {
       x: T,
       y: T,
   }
   ```

---

#### **Comparison with Similar Concepts**
- **Enums**: Can represent multiple variants; structs are single data types.
- **Tuples**: No named fields; cannot derive traits.
- **Unions**: Allow overlapping memory but are unsafe.

---

#### **Code Snippets for Edge Cases**
1. **Lifetime Annotations**  
   ```rust
   struct RefData<'a> {
       data: &'a i32,
   }
   ```

2. **Default Derive**  
   ```rust
   #[derive(Default)]
   struct Config {
       timeout: u32,
       retries: u8,
   }

   let config = Config::default();
   ```

3. **PartialEq with Generics**  
   ```rust
   #[derive(PartialEq)]
   struct Pair<T> {
       a: T,
       b: T,
   }
   ```

---

#### **Tricky Parts**
- **Padding and Size**: Use `std::mem::size_of` to debug.
- **Lifetimes in Structs**: Must match all referenced data.
- **FFI Compatibility**: Use `#[repr(C)]` for C interoperability.

---

#### **Visibility & Modules**
```rust
mod user {
    pub struct User {
        pub name: String,
        age: u8, // Private
    }
}
```