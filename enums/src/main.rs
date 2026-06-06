enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    // each variant will have a String associated with it
    V4(u8, u8, u8, u8),
    // different variants can have different value types
    V6(String),
}

// instead of four different types of struct, use an enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // identify the kind of enum using double colon
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // the variant behaves like a function that takes a string as input and returns
    // an instance of IpAddr
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let toRed = Message::ChangeColor(1, 0, 0);

    // Option<T> is an enum that has two variants, None and Some<T>
    let some_number = Some(5);
    // the annotation tells the Option what it would be if not null
    let absent_number: Option<i32> = None;

    // Option<T> cannot be used in place of <T>
    let x: i8 = 5;
    let y: Option<i8> = Some(10);

    // this throws an error because y is not guaranteed to be an i8, it can also be
    // None
    // println!("x + y = {}", x + y);
}
