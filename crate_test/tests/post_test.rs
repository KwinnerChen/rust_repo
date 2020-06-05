#[test]
fn post_test() {
    use crate_test::Post;

    let mut post = Post::new();

    post.add_text("i ate a saled for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("i ate a saled for lunch today", post.content());
}
