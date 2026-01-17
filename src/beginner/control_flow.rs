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

#[allow(dead_code)]
pub fn exercise1(n: i32) -> () {
    if n < 5 {
        println!("Too small");
    } else if n > 10 {
        println!("Too big");
    } else {
        println!("Just right");
    }
}

// Exercise 2: Multiple Conditions
// TODO: Write a function that takes a temperature in Celsius and returns:
// - "Freezing" if below 0
// - "Cold" if between 0 and 10
// - "Warm" if between 10 and 20
// - "Hot" if above 20

#[allow(dead_code)]
pub fn excercise2(temp: i32) -> () {
    if temp < 0 {
        println!("Freezing")
    } else if temp <= 10 && temp >= 0 {
        println!("Cold")
    } else if temp <= 20 && temp >= 10 {
        println!("Warm")
    } else {
        println!("Hot!")
    }
}

// Exercise 3: Loops
// TODO: Write a function that:
// 1. Uses a loop to count from 1 to 5
// 2. Uses a while loop to count down from 5 to 1
// 3. Uses a for loop to iterate over a range from 1 to 5

#[allow(dead_code)]
pub fn excercise3() {
    let mut i: i32 = 5;

    while i >= 1 {
        println!("i: {}", i);
        i -= 1;
    }

    for i in 1..=5 {
        println!("i: {}", i);
    }
}

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
