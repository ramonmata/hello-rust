#![allow(unused)]

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest_value = list[0];

    for &e in list {
        if (e > largest_value) {
            largest_value = e;
        }
    }

    largest_value
}

fn largest_char(list: &[char]) -> char {
    let mut largest_value = list[0];

    for &e in list {
        if (e > largest_value) {
            largest_value = e;
        }
    }

    largest_value
}

// Largest using Generics

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest_value = list[0];

    for &item in list {
        if item > largest_value {
            largest_value = item;
        }
    }

    largest_value
}


// Generic in Struct Definitions

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V,W>) -> Point<T,W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f64,f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Option<T> {
    Some(T),
    None,
}

use std::fmt::{Display, Debug};
use hello_rust::{Tweet, NewsArticle, Summary};

fn main() {

    let numbers = vec![10,20,30,40,50,60,70,80,90,100];
    println!("Largest is: {}", largest_i32(&numbers));

    let chars = vec!['a','b','c','d','e','f'];
    println!("Largest is: {}", largest_char(&chars));


    println!("(Generic) Largest is: {}", largest(&numbers));
    println!("(Generic) Largest is: {}", largest(&chars));

    let pos_integer = Point { x: 10, y: 10};
    // pos_integer.distance_from_origin();  // Method does not exist for i32,i32

    let pos_float = Point { x: 10.3, y: 10.1};
    pos_float.distance_from_origin();

    let pos_integer_float = Point { x: 10, y: 10.1};

    let pos_mixed = pos_integer.mixup(pos_float);


    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false
    };

    println!("{}", tweet.summarize());

    let article = NewsArticle {
        author: String::from("Iceburgh"),
        location: String::from("Pittsburgh, PA, USA"),
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        content: String::from("The Pittsburgh Penguins once again are the best \
        hockey team in the NHL"),
    };

    println!("New article available! {}", article.summarize());

    let summarizable = returns_summarizable();
    println!("{}", summarizable.summarize_author());
}


// Specifying multiple trait bounds with +
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}

// Using where to make it clearer

fn some_function_clearer<T, U>(t: &T, u: &U) -> i32 
    where   T:Display + Clone,
            U: Clone + Debug
{
    1
}


// Using impl Trait syntax in the return position
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false
    }
}


// Fails it can return between 2 types NewsArticle or Tweet
// fn returns_summarizable_fails(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }