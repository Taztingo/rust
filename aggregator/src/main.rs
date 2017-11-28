extern crate aggregator;

fn main() {
    let temp = aggregator::NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content")
    };
    aggregator::notify(temp);
}