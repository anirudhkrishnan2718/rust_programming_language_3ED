use std::fmt::{Debug, Display};
fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("unfortunately, as you already know, people"),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());
}

pub trait Summary {
    // each type that implements this trait has to provide its own function body for
    // summarize
    fn summarize(&self) -> String {
        // default implementation in the absence of a type's own implementation
        format!("(Read more from {}...)", self.summarize_author())
    }

    // this method does not have a default impl, and thus requires types to specify one
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// complains unless all traits are implemented
impl Summary for NewsArticle {
    // method signature must match trait definition
    fn summarize(&self) -> String {
        format!(
            "{}, by {} (filed from {})",
            self.headline, self.author, self.location
        )
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    // commenting this out uses the default impl of summarize
    // fn summarize(&self) -> String {
    //     format!("[{}] -> {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// generic is bound to have the Summary Trait be the same type for both parameters
pub fn notify_bound<T: Summary>(item1: &T, item2: &T) {}
// both parameters can be different types, as long as they implement the Summary trait
pub fn notify_bound_2(item1: &impl Summary, item2: &impl Summary) {}

// use plus sign to specify multiple trait bounds that are all enforced
pub fn notify_bound3(item: &(impl Summary + Display)) {}
pub fn notify_bound4<T: Summary + Display>(item: &T) {}

// less cluttered way of writing trait bounds using where
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

fn some_function2(switch: bool) -> impl Summary {
    // returns any type that implements the Summary trait
    // can only return one type, not one of many possible types using an if condition
    NewsArticle {
        headline: String::from("A"),
        location: String::from("bcd"),
        author: String::from("efg"),
        content: String::from("hij"),
    }
}
