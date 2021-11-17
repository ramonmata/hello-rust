#![allow(unused)]
fn main() {
    // Slices lets us reference contiguous sequence
    // of elements in a collection rather than the whole
    // collection.

    let s1 = String::from("Hello World");
    let first_word = find_first_word(&s1);

    println!("Word: {}", s1);
    println!("Index at first word {}", first_word);

    // String slice
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];

    println!("{} / {}", hello, world);

    println!("First word = {}", find_first_word_plus_plus(&s));


    // Other Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

fn find_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }

    s.len()
}

fn find_first_word_plus(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}

// Having a &str as argument we can accept:
// string literals, String references and slices
fn find_first_word_plus_plus(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}