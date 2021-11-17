#![allow(unused)]
fn main() {

    if_expressions();

    if_divisible();

    // If as an expression at right side of statement
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    loop_repetition();

    loop_labels();

    return_value_from_loop();

    while_loop();

    for_loop();

    for_loop_range();
}


fn if_expressions() {
    let number = 7;

    if (number < 5) {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
}

fn if_divisible() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4")
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }
}

fn loop_repetition() {
    let mut counter: usize = 0;

    loop {
        counter += 1;
        println!("Hello....{}", counter);

        if counter>=10 {
            break;
        }
    }
}

fn loop_labels() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);

        let mut remaining = 10;

        loop {
            println!("\tRemaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn return_value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter== 10 {
            break counter * 2;
        }
    };

    println!("The result form loop is {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for value in a {
        print!("{} ", value);
    }

    println!();
        
}

fn for_loop_range() {
    for i in (1..4).rev() {
        println!("{}!",i)
    }

    println!("LIFTOFF!!!")
}