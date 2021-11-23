#![allow(unused)]

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest_value = list[0];

    for &e in list {
        if (e > largest_value) {
            largest_value = e;
        }
    }

    largest_value
}

fn largest_char(list: &[char]) -> char {
    let mut largest_value = list[0];

    for &e in list {
        if (e > largest_value) {
            largest_value = e;
        }
    }

    largest_value
}

// Largest using Generics

fn largest<T>(list: &[T]) -> T {
    let mut largest_value = list[0];

    for &item in list {
        if item > largest_value {
            largest_value = item;
        }
    }

    largest_value
}


// Generic in Struct Definitions

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V,W>) -> Point<T,W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f64,f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Option<T> {
    Some(T),
    None,
}


fn main() {

    let numbers = vec![10,20,30,40,50,60,70,80,90,100];
    println!("Largest is: {}", largest_i32(&numbers));

    let chars = vec!['a','b','c','d','e','f'];
    println!("Largest is: {}", largest_char(&chars));


    println!("(Generic) Largest is: {}", largest(&numbers));
    println!("(Generic) Largest is: {}", largest(&chars));

    let pos_integer = Point { x: 10, y: 10};
    // pos_integer.distance_from_origin();  // Method does not exist for i32,i32

    let pos_float = Point { x: 10.3, y: 10.1};
    pos_float.distance_from_origin();

    let pos_integer_float = Point { x: 10, y: 10.1};

    let pos_mixed = pos_integer.mixup(pos_float);

}
