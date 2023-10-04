fn main() {
    let mut x = 4;
    println!("x is : {x}");
    x = 5;
    println!("x is : {x}");


    // you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.
    // You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.
    const TIME_IS: i32 = 3* 4;


    // Shadowing

    let y = 5;
    let y = 6;
    println!("y is : {y}");
    { //block
        println!("a) y is: {y}");
        let y = 7;
        println!("b) y is {y}")
    }
    println!("c) y is {y}")
}