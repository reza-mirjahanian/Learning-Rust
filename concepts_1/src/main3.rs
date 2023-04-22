// References and Borrowing
// the opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *



// Mutable References
fn main() {
    let mut s = String::from("hello");
    change(&mut s);


    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; //cannot borrow `s` as mutable more than once at a time
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}