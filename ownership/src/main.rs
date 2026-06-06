fn main() {
    // instead of a string literal, which is immutable, do this
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{s}");

    let s1 = String::from("Anirudh");

    // s1 has gone out of scope
    // let s2 = s1;
    // println!("s1 = {s1} and s2 = {s2}");

    // s2 is a deep copy of s1, and s1 is still in scope
    let s2 = s1.clone();
    println!("s1 = {s1} and s2 = {s2}");

    let s3 = String::from("First");
    let s3 = String::from("Second");
    println!("Which? {s3}");

    // fixed size variables do not go on the heap, and can be copied
    let x = 5;
    let y = x;
    // This does not invalidate x
    println!("x = {x} and y = {y}");

    let a1 = String::from("Hi");
    takes_ownership(a1);
    // compile error, since the string has moved to the function
    // println!("{a1}");

    // such an error does not occur for integers and other fixed size data types
    let b1 = 10;
    makes_copy(b1);
    println!("b1 is {b1}")
}

fn takes_ownership(s: String) {
    println!("Inside function, s is {s}")
}

fn makes_copy(x: i32) {
    println!("Inside function, x is {x}")
}
