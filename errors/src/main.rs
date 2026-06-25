use ::std::fs::File;
use std::io::{self, Read};
fn main() {
    // Error occurring inside this function is propagated to main
    println!("The result was {:?}", read_username_from_file());

    // ? can only be used in functions that return a Result or Option type
    // this will throw an error
    // let greeting = File::open("hello4.txt")?;

    // The first line is Some(char) not the char itself, since the function returns an
    // Option<char>
    println!(
        "The result is {:?}",
        last_char_of_first_line("hello\nhow are you")
    );
    println!("The result is {:?}", last_char_of_first_line(""))
}

// The ? is a shortcut that returns the type of error defined in the function signature's
// output if failure
// or returns the file handle for the first ? and the file contents as a string for the
// second ?
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello4.txt")?.read_to_string(&mut username)?;
    // This expression is the return value in case both of the above ? succeed
    Ok(username)
}

// there is a shortcut called fs::read_to_string that avoids the username variable

fn last_char_of_first_line(text: &str) -> Option<char> {
    // next() runs the iterator once to provide the first line
    // ? returns None from the function if there is no first line
    text.lines().next()?.chars().last()
}

// custom type that implements validation as part of its constructor
pub struct Guess {
    value: i32,
}

impl Guess {
    // setter method with validation that panics for numbers out of range
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Guess { value }
    }

    // getter method because value field is private
    // This way there is no way to create an instance of Guess that bypasses the
    // validation in the constructor above
    pub fn value(&self) -> i32 {
        self.value
    }
}
