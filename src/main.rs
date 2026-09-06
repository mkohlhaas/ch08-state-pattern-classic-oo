// See first the Typestate pattern example! (Code will be easier to understand.)
//
// Implementation of a blogging workflow:
// Draft -> Pending Review -> Published
// a Post starts as a `Draft`, moves to `Pending Review`, and finally becomes `Published`.
//
// Draft, PendingReview, Published will be the States.
// Post will be the Context. Post will have a State.

// ========================= //
// 1. Define the State trait //
// ========================= //

trait State {
    // =========================== //
    // A. State changing functions //
    // =========================== //

    // They are not applicable for each State.
    // These are the accumulated functions of all States.

    // Consumes the old state and returns the new state
    fn request_review(self: Box<Self>) -> Box<dyn State>; // for Drafts (will turn into PendingReview)
    fn approve(self: Box<Self>) -> Box<dyn State>; // for PendingReview (will turn into Published)

    // ======================== //
    // B. Overridable Functions //
    // ======================== //

    // Default implementation: most states should not show content (only Published should)
    // applicable for Published (State won't be changed; Published stays Published)
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        println!("Default trait content");
        ""
    }
}

// ============================ //
// 2. Define the Context struct //
// ============================ //

// Post has a State
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})), // Post starts as Draft
            content: String::new(),
        }
    }

    // maybe this should be in the State trait
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // delegate behavior to the current state
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // temporarily take ownership out of the Option
        if let Some(s) = self.state.take() {
            // delegates request_review
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            // delegates approve
            self.state = Some(s.approve());
        }
    }
}

// ============================ //
// 3. Implement concrete states //
// ============================ //

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("State Draft: request_review");
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("State Draft: approve");
        self // cannot approve a draft directly; return self unchanged
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("State PendingReview: request_review");
        self // already pending review
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("State PendingReview: approve");
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("State Published: request_review");
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("State Published: approve");
        self
    }
    // only the Published state overrides this to reveal content
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        println!("State Published: content");
        &post.content
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let mut post = Post::new();
    post.add_text("Rust is awesome.");

    assert_eq!(post.content(), ""); // Default trait content

    post.request_review(); // State Draft: request_review
    assert_eq!(post.content(), ""); // Default trait content (in `Pending Review` state)
    post.add_text("I can add more text!"); // we can't do this in the typestate design pattern!

    post.approve(); // State PendingReview: approve
    assert_eq!(post.content(), "Rust is awesome.I can add more text!"); // State Published: content
}
