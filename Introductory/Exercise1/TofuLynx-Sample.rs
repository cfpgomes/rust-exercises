// Author: TofuLynx
// This program prints "Hello World!".
// To compile: $ rustc TofuLynx-Sample.rs
fn main() {
    // First Part: Simple Print.
    println!("Hello World!");

    // Second Part: Formatted Print.

    println!("{} {}{}", "Hello", "World", "!");

    println!("The {} value is: {}", "Pi", 3.14);

    println!("{0} said {1} is beautiful, but {1} said that {0} is ugly.", "Bob", "Chloe");

    println!("{introduction}, {object}{exclamation}",
             object       = "the lazy dog",
             introduction = "Hello",
             exclamation  = "!");

    println!("The binary representation of {0} is {0:b}", 23);

    // This prints to io::stderr
    eprintln!("Error: Something Happened!");
}