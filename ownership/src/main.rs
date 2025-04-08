fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); //sの値が関数にムーブされ…
    // ...ここではもう有効ではない

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、i32はCopyはなので、この後にxを使っても大丈夫

    let _s1 = gives_ownership(); //gives_ownershipは、戻り値をs1にムーブする

    let s2 = String::from("hello"); // s2がスコープに入る

    let _s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let mut s = String::from("hello");
    change(&mut s);
{
    let _r1= &mut s;
} // r1はここでスコープを抜けるため、問題なく新しい参照を作ることができる
   
    let _r2 = &mut s;

    let _word = first_word(&s); // word will get the value 5  wordの中身は、値5になる

    s.clear(); // this empties the String, making it equal to ""  Stringを空にする。つまり、""と等しくする

    // word still has the value 5 here, but there is no more string that we could meaningfully use the value 5 with. word is now totally invalid!
    // wordはまだ値5を保持しているが、もうこの値を正しい意味で使用できる文字列は存在しない。
    // wordは今や完全に無効なのだ！

    // first_word works on slices of `String`s
    // first_wordは`String`のスライスに対して機能する
    let my_string = String::from("hello world");
    let _word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slice of stiring literals
    // first_wordは文字列リテラルのスライスに対して機能する
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already, this works too, without the slice syntax!
    // 文字列リテラルは「それ自体すでに文字列スライスなので」、スライス記法なしでも機能するのだ！
    let _word = first_word(my_string_literal);

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

#[allow(dead_code)]
fn calculate_length_ref(s: &String) -> usize { // sはStringへの参照
    s.len()
} // ここで、sはスコープ外になる。しかし、参照しているものの所有権を持っているわけではないので何も起こらない

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}