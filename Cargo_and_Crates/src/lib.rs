//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convinient.
//! 
//! # 自分のクレート
//! 
//! `my_crate`は、ユーティリティの集まりであり、特定の計算をより便利に行うことができます。
//! 
//! Adds one to the number given. 
/// Adds one to the number given.
/// 与えられた数値に1を足す。
/// 
/// # Examples
/// 
/// ```
/// let five = 5
/// 
/// assert_eq!(6, my_crate::add_one)
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// # Art
/// 
/// A library for modeling artistic concepts.
/// #芸術
/// 
/// 芸術的な概念をモデル化するライブラリ。

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    /// RYBカラーモデルによる主色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    /// RYBカラーモデルによる副色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    use crate::kinds::PrimaryColor;

    /// Combines two colors in equal amounts.
    /// a secondary color.
    /// 二つの主色を同じ割合で混合し、副色にする。
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) => SecondaryColor::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Blue) => SecondaryColor::Green,
            (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            _ => SecondaryColor::Orange,
        }
    }
}