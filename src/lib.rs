// Declares a Trait
pub trait Summary {

    // Describes behavior
    // fn summarize(&self) -> String;

    // .. more behaviors as needed
    fn summarize_author(&self) -> String;


    // We can algo define a default trait
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implements Summary trait for News Article
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})",self.headline, self.author, self.location)
//     }
// }

// Empty Implements Summary trait for News Article, uses default
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implements Summary trait for Tweet
impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("\"{}\", by {}", self.content, self.username)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// impl Trait "Syntax"
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound "Syntax" -- longer form of "impl trait"
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}