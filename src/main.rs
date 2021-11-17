#![allow(unused)]
fn main() {
    println!("Hello World");

    another_function();

    another_function_x(5);

    print_labeled_measurement(5, 'h');

    let x = 5;
    let y = {
        let x = 3;
        x + 1 // <- if you add ; it is converted to statement
    };

    println!("The value of y is: {}", y);

    let mut x = five();
    println!("The value of x is: {}", x);

    x = plus_one(10);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another Function.")
}

fn another_function_x(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(value: i32) -> i32 {
    value + 1
}
