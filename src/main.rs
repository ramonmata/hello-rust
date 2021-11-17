#![allow(unused)]
fn main() {
    // Scalar

    // Integer Types
    // Length  --  Signed  --  Unsigned
    // 8 bit         i8           u8
    // 16 bit        i16          u16
    // 32 bit        i32          u32
    // 64 bit        i64          u64
    // 128 bit       i128         u128
    // arch          isize        usize  <- Depends of platform (32/64 bits)

    // Rust defaults integers to i32
    // isize / usize primary use when indexing some sort of colleciton
    
    // Integer Literals
    // Decimal 98_222    _ <-- visual separator
    // Hex     0xff
    // Octal   0o77
    // Binary  0b1111_0000
    // Byte    b'A'


    let mut x: u8 = 10;

    // To handle wrapping of values 
    // we can use wrapping_* checked_* overflowing_* saturating_* methods
    x = match x.checked_add(250u8) {
        Some(r) => r,
        None => x
    };

    println!("x = {}", x);


    // Floating-Point types
    // Length  --  Signed
    // 32 bit        f32
    // 64 bit        f64  <- Default type in rust

    let x = 2.0;       // f64
    let y: f32 = 3.0;  // f32

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    
    // Integer division rounds down to nearest integer
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;


    // Boolean Type
    let t = true;

    let f: bool = false; // with explicit type annotation


    // Character type (4 bites in size)

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    
    // Compound types can group multiple values into one type

    // Tuple type
    let mut tup: (usize, f64, i32) = (1, 2.0, 3);

    tup.2 = 100;  // access single element

    let (x, y, z) = tup; // destructure tuple

    tup.0 = 10;
    println!("{}", tup.0);


    // Array type
    let months = [
        "January", "February", "March",
        "April", "May", "June", "July",
        "August", "September", "October",
        "November", "December"];

    let a = [0,1,2,3,4,5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3;5];
    
    println!("{}", a[3]);
}
