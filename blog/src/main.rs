extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    // 今日はお昼にサラダを食べた
    post.add_text("I ate a salad for lunch today");
    
    let post = post.request_review();

    let post = post.approve();
    
    assert_eq!("I ate a salad for lunch today", post.content());
}