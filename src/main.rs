fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn run_something() {
    println!("print run_something \n");
}

#[test]
fn variable_test() {
    // variable in rust using snake_case
    let name_variable = "Rust";

    // and for print variable using {} in the string and in the end write , your_variable
    println!("Hello, {}", name_variable);

    // mutable variable
    let mut message_number = 1;
    let message1 = "hello";

    println!("\nmessage_number {}: {}", message_number, message1);

    // variable below can't be change we must using mut after let in variable message_number
    // output: error[E0384]: cannot assign twice to immutable variable
    // let message_number = 2;
    // let message2 = "world";

    message_number = 2;
    let message2 = "world";

    println!("message_number {}: {}", message_number, message2);

    message_number = 3; // write variable with method type inference
    let message3: i8 = 24; // write variable with method manifest typing

    // call {1} it's mean param of println! so if {1} it's get arguments 2, and if 0 it's get arguments 1
    // output: message_number 3: 24
    println!("message_number {1}: {0}", message3, message_number);

    // we can use like this also
    let new_variable: i32;
    new_variable = 1; // so it's predefined value

    println!("\nnew_variable {}", new_variable);

    // we can declare a lot of variable with one statement
    let (x, y) = (1, 2);
    println!("\nfirst variable: {}, second variable: {}", x, y);

    // also can use with mutable variable in one statement
    let (mut x, y) = (1, 2);
    println!("before: {}, {}", x, y);

    x = 3;
    println!("after: {}, {}", x, y);

    // declare variable with type data where in the last value
    let data1 = 24i8;
    println!("\n{}", data1);

    let data2 = 24_i8;
    println!("{}", data2);

    // shadowing variable
    let x = 1;
    let x = x + 1;
    println!("\nshadowing variable: {}", x);

    // variable _

    println!("\nvariable _");
    let _ = run_something();
}
