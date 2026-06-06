fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // struct instances have to be initialized as mutable if needed
    // can't make just a few fields mutable, only entire instances
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("user1.email before is {}", user1.email);
    user1.email = String::from("someone@google.com");
    println!("user1.email after is {}", user1.email);

    // move over all remaining fields from user1
    // user1 cannot be used any longer since the Strings cannot be copied, only moved
    let user2 = User {
        email: String::from("anirudh@yahoo.com"),
        ..user1
    };

    println!(
        "user2.name is {} and user2.email is {}",
        user2.username, user2.email
    );

    // This string was moved in creating user2, and thus cannot be used
    // println!("user1.name is {}", user1.username);
    // This string was not moved, so it can be used
    println!("user1.email is {}", user1.email);

    // tuple structs are shorthand structs with no field names
    struct Point(i32, i32, i32);
    let point1 = Point(1, 2, 3);

    // destructuring a tuple struct requires the struct name before the variables
    let Point(x, y, z) = point1;
    println!("x = {x}, y = {y}, and z = {z}");
}

// shorthand syntax when the function parameters are the exact same as the struct
// fields
// fn build_user(username: String, email: String) -> User {
//     User {
//         active: false,
//         username,
//         email,
//         sign_in_count: 0,
//     }
// }
