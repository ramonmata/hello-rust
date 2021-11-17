#![allow(unused)]
fn main() {
    let mut s1 = String::from("Hello Rust");
    let len = calculate_length(&s1);
    println!("Length of '{}' = {}", s1, len);

    change_reference(&mut s1);
    println!("Pointer of s1 = {}", s1);



    // Non mutable references, no data race, OK
    let r1 = &s1;
    let r2 = &s1;

    println!("{} == {}", r1, r2);


    // This fails at compile time to avoid "data race"
    // let r1 = &s1;
    // let r2 = &mut s1;

    // println!("{} == {}", r1, r2);


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    
    let r3 = &mut s; // no problem
    println!("{}", r3);

    // At compiler time Rust detects dangling references
    // making safe code
    // let reference_to_nothing = dangle();
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_reference(s: &mut String) {
    s.push_str(" World");
}

// fn dangle() -> &String {
//     let s = String::from("Hello Dangle");
//     &s
// }