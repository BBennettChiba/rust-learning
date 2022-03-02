use state_design_patterns::Post;
fn main() {
    let mut post = Post::new("");

    post.add_text(&"hello");
    // assert_eq!(post.content(), "");

    // post.request_review();
    // assert_eq!(post.content(), "");
    let post = post.request_review();
    // post.approve();
    let post = post.approve();
    assert_eq!(post.content(), "hello");
}
