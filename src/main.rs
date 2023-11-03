use aggregator::*;
use std::fmt::Display;

fn main() {
    let mut tweets: Vec<Tweet> = Vec::new();

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    tweets.push(tweet);
    tweets.push(tweet2);

    notify(tweets);

    let pair = Pair::new(10, 15);

    pair.cmp_display();

    // returns_summarizable(true);
}

pub fn notify(items: Vec<Tweet>) {
    for item in items {
        println!("Breaking tweet ! {}", item.summarize());
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
