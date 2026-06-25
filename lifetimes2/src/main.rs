// structs holding references need a lifetime annotation
struct ImportantExcerpt<'a> {
    // struct instance cannot outlive the reference it holds in its part field
    part: &'a str,
}

fn main() {
    let string1 = String::from("long string is long");
    // let result;

    {
        let string2 = String::from("xyz");

        // will not compile since string2 does not live long enough
        // result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is: {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// since the same 'a is used for both parameters and the return value, string2 needs
// to be valid as long as result is
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// string literals have a static lifetime, even though the variable they are bound to
// does not
