use crate::traits::summary;
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl summary::Summary for Tweet {
    fn summarize(&self) ->  String {
        format!("{}:{}", self.username,self.content)
    }
}


