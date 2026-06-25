use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// This method is implemented for all Pair instances
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This method is implemented only for Pair instances where the type T has both the
// Display and PartialOrd traits.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is {}", self.x);
        } else {
            println!("The largest member is {}", self.y)
        }
    }
}

fn main() {
    // can declare a variable without a value
    let r: i32;
    // error if it is used without assigning a value to it, meaning that it is not null
    // by default
    // println!("r = {r}");
    r = 10;
    println!("r = {r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// the lifetime of the returned string slice is equal to both parameters, which are
// equal to each other

// Without lifetime annotations, the compiler will cause an error
// fn longest<'a>(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// the lifetime of the result must be equal to or smaller than the smallest lifetime
// among the arguments
