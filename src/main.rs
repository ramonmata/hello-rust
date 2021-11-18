#![allow(unused)]
fn main() {
    let config_max = Some(3u8);

    // Using Match is a bit verbose for a Some value
    match config_max {
        Some(max) => println!("Max config is {}", max),
        _ => ()
    }

    // Use if let
    if let Some(max) = config_max {
        println!("Max config is {}", max);
    }

    // if let also can use an else like match _
    if let Some(max) = config_max {
        println!("Max config is {}", max);
    } else {
        println!("No value in max config");
    }

}
