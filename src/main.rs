#![allow(unused)]

use std::fmt::Display;

fn main() {
    /*
    // Every reference in Rust has a "lifetime"

    let r;                    // -----------+-- 'a
    {                         //            |
        let x = 5;            // -+-- 'b    |
        r = &x;               //  |         |
    }                         // -+         |
                              //            |
    println!("r: {}", r);     // -----------+   r refeers to a lifetime b (invalid)
    */

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("Largest: {}", result);


    // Wont compile as string2 lifetime is lost before println!
    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("Largest: {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Level: {}", i.level());
    println!("Announce and Return Part: {}", i.announce_and_return_part("Here is my announcement"));


    // Value is stored directly in the program binary, which is always available.
    let s: &'static str = "I have a static lifetime.";

    println!("Longest Plus: {}", longest_with_an_announcement("ABC", "DEFG", "Sending Two Strings"));
}

// Input Life Times, Output LifeTimes
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if (s1.len() > s2.len()) {
        s1
    } else {
        s2
    }
}


// Wont work as result is declared in the function scope and
// can not pass a ref as a return value, we need to return an Owned data type

// fn longest_test<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.clone().as_str()
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}


impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention Please: {}", announcement);
        self.part
    }
}

// LIFETIME ELISION RULES

// * The first rule is that each parameter that is a reference gets 
// its own lifetime parameter. In other words, a function with one 
// parameter gets one lifetime parameter: 

// fn foo<'a>(x: &'a i32); 

// a function with two parameters gets two separate lifetime 
// parameters: 

// fn foo<'a, 'b>(x: &'a i32, y: &'b i32); 

// and so on.


// * The second rule is if there is exactly one input lifetime parameter, 
// that lifetime is assigned to all output lifetime parameters: 

// fn foo<'a>(x: &'a i32) -> &'a i32.


// * The third rule is if there are multiple input lifetime parameters, 
// but one of them is &self or &mut self because this is a method, 
// the lifetime of self is assigned to all output lifetime parameters. 

// This third rule makes methods much nicer to read and write because 
// fewer symbols are necessary.

fn longest_with_an_announcement<'a, T>(
    x:&'a str,
    y:&'a str,
    ann: T,
) -> &'a str 
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}