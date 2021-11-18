#![allow(unused)]

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_world(s: Option<String>) -> Option<String> {
    match s {
        None => None,
        Some(text) => {
            let mut new_text = String::from("");
            new_text.push_str(&text);
            new_text.push_str(" world!");
            Some(new_text)
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        None => None,
    }
}

fn main() {
    let c1 = Coin::Dime;
    let c2 = Coin::Penny;
    let c3 = Coin::Quarter(UsState::Alabama);

    println!("{}", value_in_cents(&c1));

    println!("{}", value_in_cents(&c2));

    println!("{}", value_in_cents(&c3));

    let n = Some(1); // Integer implements Copy trait
    let n_plus_one = plus_one(n);
    let nope = plus_one(None);

    println!("{:?}",n);
    println!("{:?}",n_plus_one);
    println!("{:?}",nope);

    nope.unwrap(); // Works because i32 implements Copy trait

    let greet = Some(String::from("Hello"));
    println!("{:?}", plus_world(greet));

    // Wont wokr as String does not implement Copy trait
    //greet.unwrap();


    // Match all

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)
    };


    // if we do not want to use the match value,
    // use _ pattern to avoid unsed variable
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll()
    };


    // if we do not want to use the match value,
    // use _ pattern to avoid unsed variable
    // also use the unit value () to express "nothing else happens"
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()
    };
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(i: i32) {}
fn reroll() {}
