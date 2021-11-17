#![allow(unused)]


use std::time::{Duration, Instant};

// Structs
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32);

// Unit-like Structs
struct AlwaysEqual;


fn main() {
    let start = Instant::now();

    let mut user1 = User {
        email: String::from("some@mail.com"),
        user_name: String::from("nickname"),
        active: true,
        sign_in_count: 0
    };

    user1.email = String::from("other@mail.com");

    let user2 = make_user(
        "nick".to_string(),
        "new@mail.com".to_string()
    );

    let user3 = User {
        email: String::from("third@mail.com"),
        active: user2.active,
        user_name: user2.user_name,
        sign_in_count: user2.sign_in_count
    };

    // struct update syntax (..struct instance)
    let user4 = User {
        email: String::from("fourth@mail.com"),
        ..user1 // remaining fields will copy from user1
    };

    // User 1 String fields "moved" to user4 can't be used  
    //println!("user1: {}", user1.user_name);


    // Tuple Structs

    let black_color = Color(0, 0, 0);
    let origin = Point(0, 0);

    let subject = AlwaysEqual;

    println!("Elapsed time: {:?}", start.elapsed());
}


fn make_user(user_name: String, email: String) -> User {
    User {
        user_name,  // argument is named as the struct fields
        email,      // argument is named as the struct fields
        active: true,
        sign_in_count: 0,
    }
}