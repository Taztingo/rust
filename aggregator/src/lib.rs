pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summarizable for NewsArticle {
    
    //This is now using the default implementation
    //fn summary(&self) -> String {
    //    format!("{}, by {} ({})", self.headline, self.author, self.location)
    //}

    fn author_summary(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}