#![allow(unused)]

// // Noob example

// fn main() {
//     let width = 30;
//     let height = 50;
//     println!(
//         "The area of the rectangle is {} sq pixels",
//         area(width, height)
//     );
// }

// fn area(width: i32, height: i32) -> i32 {
//     width * height
// }

// // Refactoring as Tuple
// fn main() {
//     let rec = (30, 50);
//     println!(
//         "The area of the rectangle is {} sq pixels",
//         area(rec)
//     );
// }

// fn area(d: (i32, i32)) -> i32 {
//     d.0 * d.1
// }


// Refactoring as Struct

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

// adds "associate functions"
impl Rectangle {

    // Have access to self (methods)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }

    // (!methods) no access to self
    // Ofthen used as constructors / builders, etc
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rec = Rectangle{
        // width: dbg!(30*2),
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 20,
        height: 30
    };

    println!("rec is {:#?}", rec);

    println!(
        "The area of the rectangle is {}/{} sq pixels",
        rec.area(),
        area(&rec)
    );

    println!("rec can hold rec2 = {}", rec.can_hold(&rec2));
    println!("rec2 can hold rec = {}", rec2.can_hold(&rec));
    
    // dbg!(&rec);


    // Calling a constructor / builder

    let sq = Rectangle::square(20);
    println!("Square = {:?}", sq);
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
