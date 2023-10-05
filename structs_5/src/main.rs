struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


//tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// Unit-Like Structs Without Any Fields
struct AlwaysEqual; //Trait



#[derive(Debug)] //Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct.
struct Rectangle {
    width: u32,
    height: u32,
}


// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object
// their first parameter is always self, which represents the instance of the struct the method is being called on.
// Each struct is allowed to have multiple impl blocks.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}





///All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.
/// We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.
/// We’ve already used one function like this: the String::from function that’s defined on the String type.
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {



    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //shorthand
        email,
        sign_in_count: 1,
    }
}