pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            // 予想値は1から100の間でなければなりませんが、{}でした。
            panic!("Guess value must be greater than or equal 1, got {}", 
            value
        );
    } else if value > 100 {
        panic!(
            // 予想値は100以下でなければなりませんが、{}でした。
            "Guess value must be less than or equal to 100, got {}",
            value
        )
    }

        Guess { value }
    }
}

#[allow(dead_code)]
fn prints_and_returns_10(a: i32) -> i32 {
    // {}という値を得た
    println!("I got the value {}", a);
    10
}

pub fn add_three(a: i32) -> i32 {
    internal_adder(a, 3)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() -> Result<(), String>{
       if 2 + 2 ==4 {
        Ok(())
       } else {
        Err(String::from("two plus two does not equal four"))
       }
    }

    #[test]
    fn it_adds_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_handred() {
        assert_eq!(102, add_two(100))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            // 挨拶(greeting)は名前を含んでいません。その値は`{}`でした。
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // 予想値は100以下でなければなりません
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
        fn this_test_will_pass() {
            let value = prints_and_returns_10(4);
            assert_eq!(10, value);
        }

        #[test]
        #[ignore]
        fn this_test_will_fail() {
            let value = prints_and_returns_10(8);
            assert_eq!(5, value);
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(1, 3))
    }
}
