#![allow(unused)]

fn main() {


    // CREATING A VECTOR

    // Empty vector of i32
    let v: Vec<i32> = Vec::new();

    // Using macro Rust infers the type
    let mut v = vec![1, 2, 3]; // Vec<i32> because default integer literal is i32

    v.push(4);
    v.push(5);

    // Rust infeers this is Vec<f64> as it is the default float type
    let mut v = Vec::new();
    v.push(1.0);
    v.push(2.0);
    v.push(3.0);

    {
        let v = vec![1,2,3,4,5];
    } // v is dropped from memory after is out of scope


    // READING ELEMENTS OF VECTOR

    let mut v = vec![1,2,3,4,5];

    // This will panic as such element does not exists
    // let third = &v[10];

    let third = &v[2];
    
    // Panics here as reference is borrowed in third
    //v.push(1);
    
    println!("The third element is: {}", third);


    match v.get(3) {
        Some(data) => println!("The fourth element is: {}", data),
        None => println!("There is no fourth element at vector")
    };

    if let Some(data) = v.get(4) {
        println!("The fifth element is: {}", data);
    } else {
        println!("There is no fifth element at vector");
    }


    // ITERATING ELEMENTS OF VECTOR


    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  // Dereference operator * to get value i before we can use +=
        println!("{}", i);
    }


    // Use ENUMS to Store MULTIPLE TYPES

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for element in &row {
        match element {
            SpreadsheetCell::Int(e) => println!("{}", e),
            SpreadsheetCell::Text(e) => println!("{}", e),
            SpreadsheetCell::Float(e) => println!("{}", e),
        }
    }

    
}
