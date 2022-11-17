pub trait Summary{
    fn summarize(&self) -> String;
}

pub struct NewsArticle{
    pub headline:string,
    pub location:string,
    pub author:string,
    pub content:string,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{},by{} ({})",self.headline,self.author,self.location)
    }
}

pub struct Tweet{
    pub username:string,
    pub content:string,
    pub reply:bool,
    pub retweet:bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{}:{}",self.username,self.content)
    }
}