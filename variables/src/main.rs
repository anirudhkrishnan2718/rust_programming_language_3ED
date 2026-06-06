fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // let mut spaces = "    ";
    // spaces = spaces.len();

    // let guess = "42".parse().expect("Not a Number");
    let guess: u32 = "42".parse().expect("Not a Number");
    println!("The value of guess is: {guess}");

    // num and denom have to be same type.
    // Integer division automatic if they are ints
    let a = -5 / 3;
    // Float division if they are both floats
    let b = -5.0 / 3.0;
    println!("The values are {a}, {b}");

    let heart_eyed_cat = "☝♠😻";
    println!("Unicode char example {heart_eyed_cat}");

    let tup = (500, 6.4, "Hello");
    let (x1, y1, z1) = tup;
    println!("The value of x1 is: {x1}, y1 is: {y1}, and z1 is: {z1}");

    // Index tuples. Cannot do this inside the println macro
    let x2 = tup.0;
    let y2 = tup.1;
    let z2 = tup.2;
    println!("The value of x2 is: {x2}, y2 is: {y2}, and z2 is: {z2}");

    // Array
    // implicit type, all elements must be of same type
    let a = [1, 2, 3, 4, 5];
    println!("a is {a:?}");

    // accessing array elements using square bracket indexing
    println!(
        "The first three elements of a are {}, {}, and {}",
        a[0], a[1], a[2]
    );

    // invalid indexing compile time error
    // panic if user inputs an invalid index at runtime instead
    // println!("Outside bounds index is {}", a[10]);
}
