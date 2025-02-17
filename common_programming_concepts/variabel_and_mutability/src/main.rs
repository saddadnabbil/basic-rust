fn main() {
    // variabels
    println!("\n\n=== Variabels ===");

    let x = 5;
    let string: &str = "hello";

    println!("the value of x is: {x}");
    println!("the value of string is: {string}");

    // mutable variabels
    println!("\n\n=== Mutable Variabel ===");
    // x = 5; // error because the variabel x is not mutable
    let mut y = 6;
    println!("the value of mutable before y is {y}");
    y = 7;
    println!("the value of mutable after y is {y}");

    // constants
    println!("\n\n=== Constants ===");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("the value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    // shadowing
    println!("\n\n=== Shadowing ===");
    let x = 8;
    let x = x + 1;

    {
        let x = x * 2;
        println!("the value of x in inner scope is: {x}");
    }

    println!("the value of x is {x}");

    let space = "mantap";
    let space = space.len();
    // let mut s = "uhuy";
    // s = s.len(); // error expected `&str`, found `usize`
    println!("the length of space is {space}");
}
