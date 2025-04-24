#![allow(unused)]
fn main() {
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }
    
    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            if let Some(state) = self.state.as_ref() {
                if state.is_draft() {
                    self.content.push_str(text);
                }
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }

        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            pub fn content(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }

        pub struct PendingReviewPost {
            content: String,
        }

        impl PendingReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<State>;
        fn approve(self: Box<Self>) -> Box<State>;
        fn reject(self: Box<Self>) -> Box<State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
        fn is_draft(&self) -> bool {
            false
        }
    }
    
    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<State> {
            self
        }

        fn is_draft(&self) -> bool {
            true
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<State> {
            Box::new(PendingReview2 {})
        }

        fn reject(self: Box<Self>) -> Box<State> {
            Box::new(Draft {})
        }
    }

    struct PendingReview2 {}

    impl State for PendingReview2 {
        fn request_review(self: Box<Self>) -> Box<State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<State> {
            Box::new(Published {})
        }

        fn reject(self: Box<Self>) -> Box<State> {
            Box::new(Draft {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}
