pub trait Summary {
    fn summarize(&self) -> String {
        return self.summarize_author();
    }
    fn summarize_author(&self) -> String {
        return "none".to_string();
    }
}

trait Test {
    fn test() -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{},{},{}", self.headline, self.author, self.location);
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{},{}", self.username, self.content);
    }
}

struct Wei {}

impl Summary for Wei {}

pub fn notify(item: &impl Summary) {
    println!("Breaking News!{}", item.summarize());
}

#[test]
#[ignore]
fn trait_ex() {
    let tweet = Tweet {
        username: "tweet".to_string(),
        content: "tweet content".to_string(),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());

    let w = Wei {};
    println!("{}", w.summarize());

    let n = NewsArticle {
        headline: "".to_string(),
        author: "news".to_string(),
        location: "shen zhen".to_string(),
        content: "".to_string(),
    };
    notify(&n);
}
