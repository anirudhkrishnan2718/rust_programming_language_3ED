use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores = {scores:?}");

    // get value by key
    let team_name = String::from("Blue");
    // the get method returns an Option<&V>, which means a None if the key
    // does not exist
    // the copied method converts Option<&V> to Option<V>, and the unwrap_or
    // replaces None with the value in brackets.
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score for team {} is {}", team_name, score);

    // iteration over hashmap is arbitrary order
    for (key, val) in &scores {
        println!("{key}: {val}");
    }

    let mycolor = String::from("Red");
    scores.insert(mycolor, 25);
    // hashmap takes ownership of strings fed into it, unlike integers which have a
    // copy trait. This line will throw cause a compile error
    // println!("mycolor is {mycolor}");

    // normal reinsertion of the same key overrides existing values
    // insert key value pair only if key is previously absent
    scores.entry(String::from("Blue")).or_insert(1000);
    // or_insert method returns mutable ref to val if it exists, or inserts the
    // argument as the val for the new key
    scores.entry(String::from("Black")).or_insert(100);
    println!("scores = {scores:?}");

    // count occurrences of each word in phrase
    let text = "hello world wonderful world hello hello";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // assign the val corresponding to word to count if it exists
        // if not initialize val to 0
        let count = map.entry(word).or_insert(0);
        // dereference count to modify it
        *count += 1;
    }

    println!("{map:?}");
}
