use two_posts::Post;

fn main() {
    let txt = String::from("I ate a salad for lunch today");

    let mut post = Post::new();

    post.add_text(&txt);

    let post = post.request_review();
    let post = post.approve();
    assert_eq!(&txt, post.content());
}
