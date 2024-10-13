
fn main() {

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));



    // Quit has no data associated with it at all.
    //     Move has named fields, like a struct does.
    // Write includes a single String.
    //     ChangeColor includes three i32 values.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }


    // In Rust, you can use an underscore (_) in a match statement as a catch-all case for any value that wasn't matched by earlier arms. This is often called the "wildcard" arm. Here is an example:


    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything else"), // This arm matches any other value
    }
    let x = (1, "hello");

    match x {
        (1, _) => println!("matched 1 and anything"),
        (_, "world") => println!("matched anything and world"),
        _ => println!("matched anything else"),
    }

}
