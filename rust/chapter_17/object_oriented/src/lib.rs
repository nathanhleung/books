use either::{Either, Left, Right};

pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("Drawing Button...")
  }
}

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  pub fn add_text(&mut self, text: &str) {
    println!("Adding text...");
    if let Some(s) = &self.state {
      if s.editable() {
        self.content.push_str(text);
      } else {
        println!("Cannot add text!")
      }
    }
  }

  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self)
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
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn reject(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    ""
  }
  fn editable(&self) -> bool {
    false
  }
}
struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    println!("Requesting review...");
    Box::new(PendingReview { approvals: 0 })
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn editable(&self) -> bool {
    true
  }
}

struct PendingReview {
  approvals: i64,
}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    println!("Approving...");
    if self.approvals >= 1 {
      Box::new(Published {})
    } else {
      Box::new(PendingReview { approvals: 1 })
    }
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    println!("Rejecting...");
    Box::new(Draft {})
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

  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}

pub struct TypedPost {
  content: String,
}

impl TypedPost {
  pub fn new() -> DraftPost {
    println!("Creating draft...");
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

pub struct DraftPost {
  content: String,
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    println!("Adding text...");
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    println!("Requesting review...");
    PendingReviewPost {
      approvals: 0,
      content: self.content,
    }
  }
}

pub struct PendingReviewPost {
  content: String,
  approvals: i64,
}

impl PendingReviewPost {
  pub fn approve(self) -> PendingReviewPost {
    println!("Approving...");
    PendingReviewPost {
      content: self.content,
      approvals: self.approvals + 1,
    }
  }

  pub fn publish(self) -> Either<PendingReviewPost, TypedPost> {
    println!("Publishing...");

    if self.approvals >= 2 {
      Right(TypedPost {
        content: self.content,
      })
    } else {
      println!("Can't publish!");
      Left(self)
    }
  }

  pub fn reject(self) -> DraftPost {
    println!("Rejecting...");
    DraftPost {
      content: self.content,
    }
  }
}
