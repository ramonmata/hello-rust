#![allow(unused)]

const ZERO: usize = 0;

fn main() {
    println!("The value of constant ZERO is: {}", ZERO);

    const ONE: usize = 1;
    println!("The value of constant ONE is: {}", ONE);

    let mut x = 5;
    println!("The value of  is: {}", x);
    
    x += 1;

    { // Inner Scope
        let x:i64 = x * 2;
        println!("The value of shadow x at inner scope is: {}", x);
    }

    println!("The value of  is: {}", x);

}
