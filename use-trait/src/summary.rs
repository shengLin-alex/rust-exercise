use std::fmt::Display;

/// A traic looks like interface in other language, but a little different.
pub trait Summarizable {

    /// 產生 summary.
    /// 
    /// 使用默認實現
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

/// 帶有泛型條件的實做
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x, 
            y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {

    /// T 有實做 Display 和 PartialOrd 的話, 就可以呼叫 cmp_display
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}