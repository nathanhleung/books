mod rectangle {
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

mod guess {
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!("Guess value greater than or equal to 1, got {}", value);
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}",
                    value
                );
            }
            Guess { value }
        }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("Got value: {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::guess::*;
    use super::rectangle::*;
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("This test will fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_and_three() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn it_adds_two_and_four() {
        assert_eq!(6, add_two(4));
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Nate");
        assert!(
            result.contains("Nate"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn test_prints_and_returns_10() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        println!("ignore me")
    }
}
