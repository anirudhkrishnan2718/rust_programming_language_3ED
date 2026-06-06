fn main() {
    let number = 7;
    // condition must be of type bool
    if number < 5 {
        // if number {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let num = 7;

    // only the first matching arm is run
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 2, 3, or 4");
    }

    // if else is an expression
    let condition = true;
    let result = if condition { 100 } else { 200 };

    // error if both arms of the if result in different data types
    // let result = if condition { 100 } else { "two hundred" };
    println!("result is: {result}")
}
