fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is {}", x);

    let x = plus_one(5);

    println!("The value of x is {}", x);
}

fn plus_one(x: i32) -> i32{
    x + 1
}

fn another_function(value: i32, unit_lavel: char) {
    println!("The meansurement is: {}{}", value, unit_lavel); 
}
