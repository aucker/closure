use std::{thread, time::Duration, ptr::NonNull};

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> 
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: None }
    }   

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
// A implement of Cacher
// The Rust Programming Language

fn main() {
    // println!("Hello, world!");
    let simulated_user_value = 10;
    let simulated_random = 7;

    generate_workout(simulated_user_value, simulated_random);
}
fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups", expensive_closure.value(intensity));
        println!("Next, do {} situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Rememebr to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", expensive_closure.value(intensity));
        }
    }
}
// fn simulated_expensive_calculation(intensity: u32) -> u32 {

// }

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    println!("v1 = {}", v1);
    // assert_eq!(v2, 2);
}