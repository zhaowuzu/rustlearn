// /// 我们把State当作对象来使用，最后返回。就必须要保证它的方法全部是对象安全的
// ///  1. 方法的返回类型不是self.
// ///  2. 方法中不包含任何泛型参数
// pub  trait State{
//     /// 方法只能被包裹着当前类型的Box实例调用，它会在调用过程中获取Box<Self>的所有权并使旧的状态失效，从而将Post的状态值转换为一个新的状态
//     /// 请求审批
//     fn request_review(self:Box<Self>) -> Box<dyn State>;
//     /// 批准
//     fn approve(self:Box<Self>)->Box<dyn State>;
//
//     ///做一个有默认值的content
//     fn content<'a>(&self,post:&'a Post) -> &'a str {
//         ""
//     }
// }
//
//
// pub struct Post{
//     state: Option<Box<dyn State>>,
//     content:String,
// }
//
// impl Post {
//     pub fn new() -> Post {
//         Post{
//             state: Some(Box::new(Draft{})), // 新创建的Post都会从草稿开始
//             content:String::new(),
//         }
//     }
//
//     pub fn add_text(&mut self,text:&str){
//         self.content.push_str(text);
//     }
//
//     pub fn request_review(&mut self){
//         // 为了消耗旧的状态，request_review需要获取状态值的所以权。Rust不允许结构体中出现未被填充的值。
//         // 我们使用Option的take方法取出state字段的Some值，并在原来位置留下一个None
//         // 这样做我们能够将state的值从post中移出来，而不单单借用它
//         // 最后我们又将这个方法结果赋值给了state字段
//         if let Some(s) = self.state.take(){
//             self.state = Some(s.request_review())
//         }
//         // // 不能够使用的代码,??还不是很明白
//         // self.state = self.state.request_review()
//     }
//
//     pub fn approve(&mut self){
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }
//
//     pub fn content(&self) -> &str {
//         // as_ref原因：我们只是需要Option中值的引用，而不是所有权。
//         // 不使用as_ref错误：不能将state从参数借用中移除
//         // unwrap可以直接使用的原因是，我们知道一定不是返回None.
//         self.state.as_ref().unwrap().content(&self)
//     }
// }
//
// /// 草稿
// pub struct  Draft{}
//
// impl State for Draft  {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview{})
//     }
//
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }
//
// /// 等待审批
// pub struct PendingReview{}
//
// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published{})
//     }
// }
//
// /// 发布
// pub struct Published{}
//
// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//
//     /// 复写content
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }


/*********************************错误示范加做对比******************************************/
pub struct Post {
    content:String,
}
pub struct DraftPost{
    content:String,
}
impl Post{
    pub fn new()-> DraftPost{
        DraftPost{
            content:String::new(),
        }
    }
    pub fn content(&self) ->&str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self,text:&str){
        self.content.push_str(text);
    }
    pub fn request_review(self)-> PendingReviewPost{
        PendingReviewPost{
            content:self.content
        }
    }
}
pub struct PendingReviewPost{
    content:String,
}

impl PendingReviewPost {
    pub  fn approve(self) -> Post{
        Post{
            content:self.content
        }
    }
}