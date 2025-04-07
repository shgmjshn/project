use std::io;

fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let _y = 2.0; // f64

    let _z: f32 = 3.0; // f32

    //  addition
    // 足し算
    let _sum = 5 + 10;

    // substruction
    // 引き算
    let _difference = 95.5 - 4.3;

    // multiplication
    // 掛け算
    let _product = 4 * 30;

    // division
    // 割り算
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0　けっかは０

    // remainder
    // 余り
    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false; // with explicit type annotetion  明示的型注釈付きで

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    let a: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = a.0;

    let _six_point_four = a.1;

    let _one = a.2;

    let _a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _a = [3; 5];

    let a =  [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index."); // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line"); // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number"); // 入力された数字ではありません
    
    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index, element
    );
        
    another_function();
}

fn another_function() {
    println!("Another function."); // 別の関数
    
}
