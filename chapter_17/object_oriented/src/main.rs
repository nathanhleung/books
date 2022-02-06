use either::{Left, Right};
use object_oriented::{Button, Draw, Post, Screen, TypedPost};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing SelectBox...")
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("OK")],
            }),
            Box::new(Button {
                width: 50,
                height: 50,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    println!();
    println!("=== State Pattern ===");
    let mut post = Post::new();

    post.add_text("Hello");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("Hello");
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // Two approvals required
    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Hello", post.content());

    println!("Post content: {}", post.content());
    println!();

    println!("=== Typed Post ===");
    let mut typed_post = TypedPost::new();
    typed_post.add_text("Typed Post");
    let typed_post = typed_post.request_review();
    let typed_post = typed_post.reject();
    let typed_post = typed_post.request_review();
    let typed_post = typed_post.approve();

    if let Left(draft_post) = typed_post.publish() {
        let typed_post = draft_post.approve();
        if let Right(published_post) = typed_post.publish() {
            assert_eq!("Typed Post", published_post.content());
            println!("Post content: {}", published_post.content());
            println!();
        }
    }
}
