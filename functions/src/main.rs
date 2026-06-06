fn main() {
    println!("Hello, world!");
    another_function(7);
    labeled_measurement(5, 'h');

    // cannot assign a statement since it does not return a value
    // let x = (let y = 6);

    // using an expression makes this legal
    // the paired parentheses are a scope and an expression since the last line
    // does not have a semicolon
    let x = {
        let y = 10;
        // y + 1;
        y + 1
    };
    println!("The value of x is: {x:?}");

    let z = plus_one(15);
    println!("The value of z is: {z}")
}

fn another_function(x: i32) {
    println!("Another function! with x = {x}")
}

fn labeled_measurement(x: i32, unit: char) {
    println!("Number with units is {x}{unit}")
}

fn plus_one(x: i32) -> i32 {
    // having the semicolon means there is no ending expression (called tail)
    // that can be returned
    // x + 1;
    x + 1
}
