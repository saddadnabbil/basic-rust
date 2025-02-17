mod chatgpt;

fn main() {
    chatgpt::learn::run();
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

#[test]
fn data_type_test() {
    // signed interger
    let numeric1 = 1; // default 32 if not defined
                      // let numeric2: i8 = 128 // output: error[E0107]: literal out of range for `i8`
    let numeric2: i8 = 127;
    let numeric3: i64 = 255;

    println!(
        "\nnumeric1: {}, numeric2: {}, numeric3: {}",
        numeric1, numeric2, numeric3
    );

    // unsigned interger
    let numeric4 = 1; // default 32 if not defined
                      // let numeric5: u8 = 256 // output: error[E0107]: literal out of range for `u8`
    let numeric5: u8 = 255;

    println!("\nnumeric4: {}, numeric5: {}", numeric4, numeric5);

    // float
    let numeric6 = 1.30; // default 32 if not defined
    let numeric7: f32 = 1.430; // default 32 if not defined
    let numeric8: f64 = 1.92920; // default 64 if not defined

    println!(
        "\nnumeric6: {}, numeric7: {}, numeric8: {:.2}",
        numeric6, numeric7, numeric8
    );
}
