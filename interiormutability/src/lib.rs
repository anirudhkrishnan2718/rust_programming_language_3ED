pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // this function does not return anything, making it impossible to test directly.
    // need a mock object that keeps track of the outputs sent by set_value
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max > 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max > 0.9 {
            self.messenger
                .send("Urgent Warning: You have used up over 90% of your quota!");
        } else if percentage_of_max > 0.75 {
            self.messenger
                .send("Urgent Warning: You have used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // interior mutability
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger {
            //     sent_messages: vec![],
            // }
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // this will not work
            // self.sent_messages.push(String::from(message));

            // this works
            // self.sent_messages.borrow_mut().push(String::from(message));

            // this only panics at runtime, not at compile time, even though there are
            // clearly two mutable borrows
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
