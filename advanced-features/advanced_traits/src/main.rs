#![allow(unused)]
use std::fmt;
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    trait Animal {
        fn baby_name() -> String;
    }

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Dog;

    struct Human;

    struct Point {
        x: i32,
        y: i32,
    }

    struct Wrapper(Vec<String>);

    impl Pilot for Human {
        fn fly(&self) {
            // キャプテンのお言葉
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            // 上がれ！
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            // *激しく腕を振る*
            println!("*waving arms furiosly*")
        }
    }

    impl Dog {
        fn baby_name() -> String {
            // スポット(Wikipediaによると、飼い主の事故死後もその人の帰りを待つ中堅の名前の模様)
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            // 子犬
            String::from("Puppy")
        }
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // 赤ちゃん犬は{}と呼ばれる
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let p = Point { x: 1, y: 2 };
    p.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}