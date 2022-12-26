use  xvii_blog::Post;

fn main() {
    // 延时blog包预期行为的代码示例
    // 正常演示
    // let mut post = Post::new();
    //
    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("",post.content());
    //
    // post.request_review();
    // assert_eq!("",post.content());
    //
    // post.approve();
    // assert_eq!("I ate a salad for lunch today",post.content());

    // 故意出错,明天再搞
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today",post.content());
}
