// imports and assignments
use ferris_says::say;
use std::io::{stdout, BufWriter};

// main function of the program 
fn main() {
    // setting a variable to the standard out package?
    let stdout = stdout();
    // setting a variable of type string and filling it.
    let message = String::from("Hello fellow Rustaceans!");
    // setting another variable that is the length of the characters of the previous variable
    let width = message.chars().count();

    // setting a mutex on the standard out buff writer to be used by the message?
    let mut writer = BufWriter::new(stdout.lock());

    // pass the variables and a pointer to the standard out lock to the say package that will print out the message.
    say(message.as_bytes(), width, &mut writer).unwrap();
}