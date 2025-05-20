
---

## 🔹 Core Concept: Iteration in Rust

Rust provides three primary methods for iterating over a collection:

| Method        | Consumes the collection? | Mutable access? | Ownership?       | Typical Use Case                         |
| ------------- | ------------------------ | --------------- | ---------------- | ---------------------------------------- |
| `iter()`      | ❌ (No)                   | ❌ (No)          | Shared reference | Read-only access                         |
| `iter_mut()`  | ❌ (No)                   | ✅ (Yes)         | Mutable ref      | Modify items in place                    |
| `into_iter()` | ✅ (Yes)                  | ❌ (No)          | Takes ownership  | Consume collection, move items elsewhere |

---

## 🧱 Level 1: **Fundamentals of `iter()`, `iter_mut()`, and `into_iter()`**

### ✅ `iter()`

* Creates an iterator **by reference**.
* Items are **borrowed (`&T`)**.

```rust
fn main() {
    let nums = vec![1, 2, 3];
    for n in nums.iter() {
        println!("{}", n);  // type: &i32
    }
}
```

---

### ✅ `iter_mut()`

* Creates an iterator **by mutable reference**.
* Items are **mutably borrowed (`&mut T`)**.

```rust
fn main() {
    let mut nums = vec![1, 2, 3];
    for n in nums.iter_mut() {
        *n *= 2;
    }
    println!("{:?}", nums); // [2, 4, 6]
}
```

---

### ✅ `into_iter()`

* **Consumes the collection**, taking ownership.
* Items are moved (`T`).

```rust
fn main() {
    let nums = vec![1, 2, 3];
    for n in nums.into_iter() {
        println!("{}", n);  // type: i32
    }
    // nums is no longer accessible here
}
```

---

## 🧠 Level 2: **Compare Behavior with `Vec<T>`**

```rust
let v = vec![10, 20, 30];

let i1 = v.iter();        // type: Iter<'_, i32>
let i2 = v.iter_mut();    // type: IterMut<'_, i32>
let i3 = v.into_iter();   // type: IntoIter<i32>
```

---

## 🧰 Level 3: **Using in `for`, `map`, `collect`, etc.**

### `iter()` – Read-only access

```rust
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

### `iter_mut()` – Modify in-place

```rust
let mut v = vec![1, 2, 3];
for x in v.iter_mut() {
    *x += 10;
}
```

### `into_iter()` – Ownership taken

```rust
let v = vec![1, 2, 3];
let squared: Vec<i32> = v.into_iter().map(|x| x * x).collect();
// v cannot be used here
```

---

## 🧪 Level 4: **Working with `structs`**

```rust
#[derive(Debug)]
struct Point(i32, i32);

let points = vec![Point(1, 2), Point(3, 4)];

// Read-only
for p in points.iter() {
    println!("{:?}", p); // &Point
}

// Mutable
let mut points = vec![Point(1, 2), Point(3, 4)];
for p in points.iter_mut() {
    p.0 += 10;
}

// Ownership
let points = vec![Point(1, 2), Point(3, 4)];
for p in points.into_iter() {
    println!("{:?}", p); // Point
}
```

---

## 🔍 Level 5: **Edge Cases and Gotchas**

### ⚠️ `for x in v` is not always `into_iter()`

#### Case 1: `Vec<T>`

```rust
let v = vec![1, 2, 3];
for x in v {
    // This uses `into_iter()` on Vec<T>
}
```

#### Case 2: `[T; N]` array

```rust
let arr = [1, 2, 3];
for x in arr {
    // Rust 2021+: This also uses `into_iter()` via Copy
}
```

### ⚠️ `for x in &v` is `iter()`

```rust
let v = vec![1, 2, 3];
for x in &v {
    // Equivalent to: for x in v.iter()
}
```

### ⚠️ `for x in &mut v` is `iter_mut()`

```rust
let mut v = vec![1, 2, 3];
for x in &mut v {
    *x += 1;
}
```

---

## ⚙️ Level 6: **Trait Implementation Behind the Scenes**

| Type          | Implements                  |
| ------------- | --------------------------- |
| `&Vec<T>`     | `IntoIterator<Item=&T>`     |
| `&mut Vec<T>` | `IntoIterator<Item=&mut T>` |
| `Vec<T>`      | `IntoIterator<Item=T>`      |

```rust
fn print_all<I: IntoIterator<Item = i32>>(items: I) {
    for item in items {
        println!("{}", item);
    }
}
```

You can pass a `Vec<i32>` into `print_all`, because `Vec<i32>: IntoIterator<Item=i32>`.

But if you try:

```rust
print_all(&vec![1, 2, 3]);
```

It will fail unless you adjust the trait bound to `Item = &i32`.

---

## 🧬 Level 7: **Custom Collections: Implementing `IntoIterator`**

```rust
struct MyCollection(Vec<i32>);

impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
```

### Usage:

```rust
let mc = MyCollection(vec![1, 2, 3]);
for x in mc {
    println!("{}", x);
}
```

---

## 🆚 Summary Table: Quick Comparison

| Feature             | `iter()`       | `iter_mut()`    | `into_iter()`                   |
| ------------------- | -------------- | --------------- | ------------------------------- |
| Ownership           | No             | No              | Yes (moves)                     |
| Mutability          | No             | Yes             | No (unless owned mutably)       |
| Use after iteration | Yes            | Yes             | No                              |
| Item type           | `&T`           | `&mut T`        | `T`                             |
| Performance         | Fast           | Fast            | Fast (no borrowing)             |
| Use Case            | Read-only loop | Modify in-place | Move items / transfer ownership |

---

## 🎯 Best Practices

* Use `iter()` when **reading**.
* Use `iter_mut()` when **modifying**.
* Use `into_iter()` when **owning/moving** or **collecting/transformation**.

---

