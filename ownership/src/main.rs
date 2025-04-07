fn main() {
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); //sの値が関数にムーブされ…
    // ...ここではもう有効ではない

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、i32はCopyはなので、この後にxを使っても大丈夫

    let s1 = gives_ownership(); //gives_ownershipは、戻り値をs1にムーブする

    let s2 = String::from("hello"); // s2がスコープに入る

    let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);

} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない
// ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので何も起こらない。s1もスコープを抜け、ドロップされる。

fn takes_ownership(some_string: String) { // some_stringがスコープに入る
    println!("{}", some_string);
} //ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾をしていたメモリが解放される

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} //ここでsome_integerがスコープを抜ける。何も特別なことは起こらない

fn gives_ownership() -> String { // gives_ownershipは、戻り値を呼び出した関数にムーブする

    let some_string = String::from("hello"); // somestringがスコープに入る

    some_string // some_stringが返され、呼び出し元関数にムーブされる

}

// takes_and_gives_backは、Stringをひとつ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る

    a_string // a_stringが返され、呼び出し元関数にムーブされる
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}
