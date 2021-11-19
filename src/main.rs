#![allow(unused)]
fn main() {
    
    let str_slice = "Hello, I am a string slice";  // Stored in program's binary

    let str_data = String::from("Hello World"); // Stored in Heap


    // Creates a new empty String
    let mut s = String::new();

    // to_string method is available to types that implements Display trait
    let s_literal = "Hello";
    let s = s_literal.to_string();

    let s = "Hello".to_string();

    // We can also use String::from
    let s = String::from("Hello");

    // Strings are UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // UPDATING a String

    // Append string slice to String
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    // By using + operator

    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 has been moved here, can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "" +&s2 + "" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);


    // INDEXING into Strings

    // let s1 = String::from("Hello");
    // let h = &s1[0]; // Rust does not allow indexing of strings

    // IF needed you should use ranges to create slices with CAUTION
    let hello = "Здравствуйте";
    let s = &hello[0..2];
    println!("{}\n",s);

    // ITERATING over Strings
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }

    println!();

    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }

    println!();


}
