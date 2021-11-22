/// https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

/// Below is an example of a State Pattern
/// Add text to a post and send it in for review.
/// Once it is approved the content will be available.
/// 
/// Considering an alternative Matching Pattern, every
/// branch of logic would require a match expression.

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

  pub fn content(&self) -> &str { 
    &self.content
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content
    }
  }
}

pub struct PendingReviewPost {
  content: String,
}

impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content
    }
  }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;

  fn approve(self: Box<Self>) -> Box<dyn State>;

  // By default, do not show the content.
  // Notice the Draft and PendingReview states do not have
  // implementations for content, so they do not show the content.
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    ""
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    Box::new(Rejected {})
  }
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {
      approvals: 0
    })
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

struct PendingReview {
  approvals: u8,
}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    "Pending approvals"
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    if self.approvals + 1 == 2 {
      Box::new(Published {})
    } else {
      Box::new(PendingReview {
        approvals: 1
      })
    }
  }
}

struct Rejected {}

impl State for Rejected {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {
      approvals: 0
    })
  }
  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    // Stays rejected, it needs to go to review.
    self
  }
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    "Content removed"
  }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}

#[allow(dead_code)]
pub fn run() {
  println!("Create a post");
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  let post = post.request_review();
  let post = post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());

  // Below is old code from the early part of the example.
  // The new pattern transitions with transformations rather
  // that mutating the object and the state. 

  // // Require 2 approvals
  // post.approve();
  // assert_eq!("Pending approvals", post.content());
  // post.approve();
  // assert_eq!("I ate a salad for lunch today", post.content());

  // post.reject();
  // assert_eq!("Content removed", post.content());
}
