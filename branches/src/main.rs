fn main() {
    let number = 6;

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is devisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is devisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is devisible by 2");
    } else {
        // 数値は４，３，２，で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // numberの値は、｛｝です
    println!("The value of number is: {}", number);
}
