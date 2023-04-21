fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // When a variable is immutable, once a value is bound to a name, you can’t change that value.
    // x = 6;
    // println!("The value of x is: {x}");


    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
    // The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    //Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    // Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    //     The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

    let spaces = "   ";
    let spaces = spaces.len();
    // The first spaces variable is a string type and the second spaces variable is a number type. Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name. However, if we try to use mut for this, as shown here, we’ll get a compile-time error:

    let mut spaces = "   ";
    // spaces = spaces.len();  error

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);



    //If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
