enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    California,
    Florida,
    Texas,
}

fn main() {
    let mycoin = Coin::Penny;
    let texcoin = Coin::Quarter(UsState::Texas);
    println!("The value of my coin is {} cents", value_in_cents(mycoin));
    println!("The value of my coin is {} cents", value_in_cents(texcoin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {six:?}, but none is {none:?}");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // curly brackets only needed for multiline arm
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // state binds to the UsState variable of the Coin::Quarter instance
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

// function operates on Option with a match arm for None and Some
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // commenting out the next line causes a compiler error since one case is not
        // covered
        None => None,
        Some(i) => Some(i + 1),
    }
}

// catch-all arms come last, and catch-all variables that will not be used are
// denoted by an underscore.
