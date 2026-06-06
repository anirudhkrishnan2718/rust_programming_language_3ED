fn main() {
    let mut s = String::from("Hello World!");
    let word_size = first_word(&s);
    s.clear(); //empties the string
    println!("first word size is {word_size}");

    let mut s2 = String::from("Hello Anirudh");
    let first_word = first_word_sliced(&s2);

    // clearing s2 is caught by the compiler, unlike in the above example
    // s2.clear();
    println!("First word is {first_word}");

    let c = [1, 2, 3, 4, 5];
    let c_slice = &c[1..3];
    assert_eq!(c_slice, [2, 3], "Assertion for array slice");

    // if assert_eq! fails, it prints the message and details after crashing
    assert_eq!(c_slice, [2, 4], "Assertion for array slice");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("bytes is {:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    // fallback return value
    s.len()
}

fn first_word_sliced(s: &String) -> &str {
    let bytes = s.as_bytes();
    println!("bytes is {:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return string slice, which is a reference to a contiguous piece
            return &s[0..i];
        }
    }

    // omit start and end to mean the absoulte start and end of the slice
    &s[..]
}
