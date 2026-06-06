fn main() {
    let s1 = String::from("Anirudh");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}");
    // This throws an error since s1 is out of scope
    // println!("The length of '{s1}' is {len}");

    // borrowing instead of moving by passing a reference as argument
    let s3 = String::from("Krishnan");
    let len2 = calculate_length_referenced(&s3);
    println!("The length of '{s3}' is {len2}");

    // immutable object
    // let s4 = String::from("Hello");
    // change(&s4);

    // mutable object
    let mut s5 = String::from("How are you?");
    change_mutable(&mut s5);
    println!("s5 is {s5}");

    let r1 = &mut s5;

    // complains about more than one mutable reference
    // let r2 = &mut s5;
    // println!("r1 is {r1} and r2 is {r2}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_referenced(s: &String) -> usize {
    s.len()
}

// fn change(s: &String) {
//     s.push_str(", world!");
// }

fn change_mutable(s: &mut String) {
    s.push_str(" I am fine.");
}
