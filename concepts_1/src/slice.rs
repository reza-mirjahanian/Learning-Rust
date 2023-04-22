// With Rust’s .. range syntax, if you want to start at index 0, you can drop the value before the two periods. In other words, these are equal:

fn main(){

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
// By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
// You can also drop both values to take a slice of the entire string. So these are equal:

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];



    ///
    let s = String::from("hello world");

    let hello = &s[0..6];
    let world = &s[6..11];


    println!("The value of hello is: {hello}|");
    println!("The value of world is: {world}");
}