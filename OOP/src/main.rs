mod lib;
use lib::gui::{self, Screen, Draw, Button};

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 実際にセレクトボックスを描画するコード
    }
}

impl Draw for String {
    fn draw(&self) {
        // 実際に文字列を描画するコード
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    // はい
                    String::from("Yes"),
                    // 多分
                    String::from("Maybe"),
                    // いいえ
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                // 了解
                label: String::from("OK"),
            }),
            Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}
