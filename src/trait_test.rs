use std::fmt::Debug;
use std::fmt::Display;
use std::convert::TryInto;

pub trait Summary {
    fn summarize_author(&self) -> String;

    // 默认方法，可以覆盖
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

// 类型实现trait
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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
}


pub trait Count {
    fn count(&self) -> u32;
}

// 对任何实现了特定 trait 的类型有条件地实现 trait
// 只要是实现了Summary的类型，都为它实现ToString trait
impl<T: Summary> Count for T {
    fn count(&self) -> u32 {
        let c: u32 = self.summarize().len().try_into().unwrap();
        c
    }
}




// trait作为参数
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
// impl语法糖
pub fn notify1(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


// 两个参数的类型必须相同，且实现了Summary
pub fn notify2<T: Summary>(item1: T, item2: T) {}
// 两个参数的类型可以不同
pub fn notify3<T: Summary, E: Summary>(item1: T, item2: E) {}
// 两个参数的类型可以不同
pub fn notify4(item1: impl Summary, item2: impl Summary) {}


// 实现多个trait的类型
pub fn notify5<T: Summary + Display>(item1: T) {}
pub fn notify6(item1: impl Summary + Display) {}


fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}
// 使用where简化
fn some_function1<T, U>(t: T, u: U)
where T: Display + Clone, U: Clone + Debug {}







// 返回实现了trait的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 有条件的实现方法
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// Pair类有范型T，只有T实现了Display和PartialOrd，才可以使用cmp_display方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}







#[cfg(test)]
mod tests {
    use crate::trait_test::{NewsArticle, Summary, Count};

    #[test]
    fn summary_test() {
        let a = NewsArticle{
            headline: String::from("123"),
            location: String::from("123"),
            author: String::from("123"),
            content: String::from("123"),
        };
        println!("{}", a.summarize());
        println!("{}", a.count());
    }
}