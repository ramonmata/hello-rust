#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // Store data in the Heap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Creating a hash map from a list of teams and scores
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Accesing values in a HashMap

    let team_blue = String::from("Blue");
    let score = scores.get(&team_blue);
    if let Some(s) = score {
        println!("Blue score is {}", s);
    } else {
        println!("Blue score is Missing");
    }

    // Iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Ownership
    // If types implement the Copy trait (e.g. 132) values are copied to hash map
    // Owned values like String, values are moved and hash map will own them.

    let field_name = String::from("Hello");
    let field_value = String::from("World");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value can't be used at this point


    // UPDATING the hash map

    // OVERWRITING
    scores.insert(String::from("Blue"), 25); // Replaces the value to 25

    // Insert if key has no value
    scores.entry(String::from("Blue")).or_insert(26);

    // UPDATING based on current value

    let text = "hello world wonderful world of rust world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
