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

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alaska => year > 1900,
            UsState::California => year > 1925,
            UsState::Florida => year > 1950,
            UsState::Texas => year > 1975,
        }
    }
}

fn main() {
    // let config_max = Some(3u8);
    // requires an annotation in the None case
    let config_max: Option<u8> = None;

    // if config_max is not None, run the code
    if let Some(max) = config_max {
        println!("The configured maximum is {max}");
    } else {
        // else is run for all other cases except the match above
        println!("No finite value for maximum");
    }

    let myQuarter = Coin::Quarter(UsState::Texas);
    println!("{:?}", describe_state_quarter(myQuarter));
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // bind the UsState of the Quarter to the variable if the coin is a Quarter
    // else exit the function
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1951) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new!"))
    }
}
