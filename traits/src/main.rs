use core::fmt::Display;
use core::fmt::Debug;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} \n********************\n{}", self.headline, self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary){
    println!("Breaking news!!! {}", item.summarize());
}

pub fn notify2(item1: &(impl Summary + Display), item2: &impl Summary) {
}

// pub fn some_function<T: Display + Clone, U: Clone + Debug>(t:&T, u:&U) {
//     println!("{:?}", t);
//     println!("{:?}", u);
// }

pub fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    println!("{}", t);
    println!("{:?}", u);
    1
}

fn return_summarizeable() -> impl Summary{
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair <T> {
    x: T, 
    y: T
} 

impl <T>Pair<T> {
    fn new(x: T, y: T) -> Self{
        Self {
            x,
            y
        }
    }
}

impl <T: Display + PartialOrd> Pair <T> {
    fn can_display(&self){
        if self.x > self.y {
            println!("{} is greater than {}", self.x, self.y);
        }
        else {
            println!("{} is less than {}", self.x, self.y);
        }
    }
}

fn main() {
    let article = NewsArticle { 
        author: "Bryson Bennett".to_string(),
        headline: "Rust is awesome!".to_string(),
        content: "Rust is pretty awesome!".to_string()
    };
    let tweet  = Tweet { 
        username: "Hurinfan".to_string(),
        content: "I love Rust!".to_string(),
        reply: false,
        retweet: true
    };
    // println!("{}", article.summarize());
    // println!("{}", tweet.summarize());
    // notify(&article);
    println!("{}",return_summarizeable().summarize());
    let pair = Pair::new(article,article);
    pair.can_display();
}
