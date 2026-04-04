pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess should be between 1 and 100");
        }

        Guess { value }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height > other.height
    }
}

fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10
        };

        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
            "Greeting didn't contain Carol, it was `{result}`"
        );
    }

    #[test]
    fn smaller_wont_hold_larger() {
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        let larger = Rectangle {
            width: 10,
            height: 10,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn add_eq_four() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic]
    fn out_of_range_guess_panics() {
        Guess::new(0);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
