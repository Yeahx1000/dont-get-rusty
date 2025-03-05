//! Control Flow in Rust
//! This module contains exercises to help you understand:
//! - if/else expressions
//! - loops (loop, while, for)
//! - match expressions
//! - Pattern matching

// Exercise 1: if/else Expressions
// TODO: Write a function that takes a number and returns:
// - "Too small" if the number is less than 5
// - "Too big" if the number is greater than 10
// - "Just right" if the number is between 5 and 10 (inclusive)

// Exercise 2: Multiple Conditions
// TODO: Write a function that takes a temperature in Celsius and returns:
// - "Freezing" if below 0
// - "Cold" if between 0 and 10
// - "Warm" if between 10 and 20
// - "Hot" if above 20

// Exercise 3: Loops
// TODO: Write a function that:
// 1. Uses a loop to count from 1 to 5
// 2. Uses a while loop to count down from 5 to 1
// 3. Uses a for loop to iterate over a range from 1 to 5

// Exercise 4: Match Expression
// TODO: Write a function that takes a number and returns:
// - "One" for 1
// - "Two" for 2
// - "Three" for 3
// - "Many" for any other number

// Exercise 5: Pattern Matching with Ranges
// TODO: Write a function that takes an age and returns:
// - "Child" for ages 0-12
// - "Teenager" for ages 13-19
// - "Adult" for ages 20-64
// - "Senior" for ages 65 and above

// Bonus Challenge:
// TODO: Write a function that uses nested match expressions to handle
// different combinations of two boolean values (true/false)

// Example solutions (commented out):
/*
pub fn run_exercises() {
    // Exercise 1
    fn check_number(n: i32) -> &'static str {
        if n < 5 {
            "Too small"
        } else if n > 10 {
            "Too big"
        } else {
            "Just right"
        }
    }
    println!("Number 3: {}", check_number(3));
    println!("Number 7: {}", check_number(7));
    println!("Number 12: {}", check_number(12));

    // Exercise 2
    fn check_temperature(temp: f32) -> &'static str {
        if temp < 0.0 {
            "Freezing"
        } else if temp <= 10.0 {
            "Cold"
        } else if temp <= 20.0 {
            "Warm"
        } else {
            "Hot"
        }
    }
    println!("Temperature -5°C: {}", check_temperature(-5.0));
    println!("Temperature 15°C: {}", check_temperature(15.0));

    // Exercise 3
    println!("Counting up:");
    for i in 1..=5 {
        println!("{}", i);
    }

    println!("Counting down:");
    let mut count = 5;
    while count > 0 {
        println!("{}", count);
        count -= 1;
    }

    // Exercise 4
    fn number_to_text(n: i32) -> &'static str {
        match n {
            1 => "One",
            2 => "Two",
            3 => "Three",
            _ => "Many",
        }
    }
    println!("Number 2: {}", number_to_text(2));
    println!("Number 4: {}", number_to_text(4));

    // Exercise 5
    fn age_category(age: u32) -> &'static str {
        match age {
            0..=12 => "Child",
            13..=19 => "Teenager",
            20..=64 => "Adult",
            _ => "Senior",
        }
    }
    println!("Age 8: {}", age_category(8));
    println!("Age 25: {}", age_category(25));
}
*/ 