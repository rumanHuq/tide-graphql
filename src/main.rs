trait Messenger {
    fn send(&self, msg: &str);
}
struct LimitTracker<'lifetime, T: Messenger> {
    messenger: &'lifetime T,
    value: usize,
    max: usize,
}

impl<'lifetime, T: Messenger> LimitTracker<'lifetime, T> {
    fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let max_percentage = self.value as f64 / self.max as f64;

        if max_percentage >= 1.0 {
            self.messenger.send("Error, quota exceeded");
        } else if max_percentage >= 0.9 {
            self.messenger.send("90% limit reached");
        } else if max_percentage >= 0.75 {
            self.messenger.send("75% limit reached");
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, value: &str) {
            &self.sent_messages.borrow_mut().push(value.into());
        }
    }
    #[test]
    fn sends_over_90_percent_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(90);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
