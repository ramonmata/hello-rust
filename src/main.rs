#![allow(unused)]

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Catcher<T> 
where
    T: Fn(u32) -> u32, // trait bound for Fn (Fn, FnMut FnOnce)
{
    calculation: T,
    // value: Option<u32>,
    values_map: HashMap<u32, u32>,
}

impl<T> Catcher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Catcher<T> {
        Catcher {
            calculation,
            // value: None,
            values_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values_map.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values_map.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let x = "1".to_string();
    let mut expensive_closure = Catcher::new(|num| {
        let y = &x;
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    println!("{}", x);

    // let generic_closure = |x| x;

    // generic_closure(1);
    // generic_closure("Hello");

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break todal! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_result = simulated_expensive_calculation(intensity);
//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_result);
//         println!("Next, do {} situps!", expensive_result);
//     } else {
//         if random_number == 3 {
//             println!("Take a break todal! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_result);
//         }
//     }
// }

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("Calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }