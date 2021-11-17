#![allow(unused)]
fn main() {
    let str = "hello";
    let mut string = String::from(str);


    string.push_str(" world!");
    let mut m = string; // moved string to m, string var is not longer valid

    // Does not compile
    // println!("{}", s); // s has been moved to m

    println!("{}", str);
    println!("{}", m);


    let s1 = String::from("Hello Clone");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    takes_ownership(s1);
    // s1 has been moved from previous call to function


    let x = 5;
    makes_copy(x);
    println!("x is still = {}", x);

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = take_and_back(s1);
    println!("{}", s2);

    let (s2, len) = calculate_length(s2);
    println!("s2 len is: {}", len);

}

fn takes_ownership(s: String) {
    println!("Took ownership of string argument {}", s);
}


fn  makes_copy(value: i32) {
    println!("Copy of value is {}", value)
}

fn gives_ownership() -> String {
    String::from("Back to you")
}

fn take_and_back(mut s: String) -> String {
    s.push_str("... moved and back to you!");
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}