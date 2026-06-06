fn main() {
    // A String is a wrapper around a vector of bytes, with some extra safeguards

    // alternative initialization to String::from()
    let s = "initial contents".to_string();
    // UTF-8 encoded by definition
    let hello = String::from("नमस्ते");
    println!("{s} and {hello}");

    let mut s1 = String::from("foo");
    let s2 = "bar";

    // push_str only takes the immutable string slice as input
    // so s2 is still available
    s1.push_str(s2);
    println!("s2 is {s2} and s1 is {s1}");
    // push() is analogous function for appending a single char

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");

    // addition operator can only add a string slice (&str) to a String
    // s3 is destroyed here, but s4 remains valid
    // s4 is coerced by the compiler into a string slice
    let s5 = s3 + &s4;
    println!("s5 is {s5}");

    // format macro returns a String instead of printing
    let s6 = format!("{s4}---{s5}---{s4}");
    println!("s6 is {s6}");

    // cannot index bytes of a string by integer, since some unicode characters are
    // more than one byte long, leading to potential references to nonsense bytes
    // println!("The first byte of s5 is {}", &s5[0]);

    // can use string slices, but only if they obey unicode character boundaries

    // iterate char by char, respecting unicode byte size
    // can also iterate by bytes, and by grapheme using external crates
    for c in "शलगम".chars() {
        println!("{c}");
    }
}
