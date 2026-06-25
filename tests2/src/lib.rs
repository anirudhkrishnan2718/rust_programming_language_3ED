pub struct Guess {
    value: i32,
}

pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

// private function that can still be tested because it is brought into scope with
// use super::*
fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    // now has separate panic messages for each kind of error
    pub fn new2(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be  >= 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be <= 100, got {value}.");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic] // passes if the function panics
    fn guess_too_large() {
        // Guess::new(200);
        Guess::new(99); //does not panic, failing the test
    }

    #[test]
    #[should_panic(expected = "<= 100")] // passes if the function panics and the panic
    // #[ignore] // causes the test to be ignored
    //message contains the expected substring
    fn guess_too_large2() {
        // Guess::new2(101);
        Guess::new2(-1); // panics but does not get expected error substring, so test 
        // fails
    }

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
