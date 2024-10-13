
fn main(){
    let guess: u32 = "42".parse().expect("Not a number!");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];


    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }

    // Statements are instructions that perform some action and do not return a value.
        // Expressions evaluate to a resultant value. Let’s look at some examples.
}