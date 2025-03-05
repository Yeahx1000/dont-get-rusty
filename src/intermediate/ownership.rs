//! Ownership, Borrowing, and Lifetimes in Rust
//! This module contains exercises to help you understand:
//! - Ownership rules
//! - Borrowing (mutable and immutable)
//! - Lifetimes
//! - References and borrowing

// Exercise 1: Basic Ownership
// TODO: Write a function that:
// - Takes ownership of a String
// - Prints it
// - Returns it
// Try calling it twice with the same string

// Exercise 2: Mutable References
// TODO: Write a function that:
// - Takes a mutable reference to a String
// - Appends " world" to it
// - Returns nothing (unit type)
// Try calling it with the same string multiple times

// Exercise 3: Multiple References
// TODO: Write a function that:
// - Takes a vector of integers
// - Returns both the first and last elements
// - Uses references to avoid copying
// Handle the case where the vector is empty

// Exercise 4: Lifetimes
// TODO: Write a struct named 'TextWindow' that:
// - Has a reference to a string slice
// - Has a start and end position
// - Implements a method to get the text between positions
// - Uses lifetime annotations

// Exercise 5: Ownership and Move Semantics
// TODO: Write a function that:
// - Takes ownership of a Vec<i32>
// - Removes all even numbers
// - Returns the modified vector
// Try to use the original vector after calling the function

// Bonus Challenge:
// TODO: Write a function that:
// - Takes a reference to a string
// - Returns a new string containing only the vowels
// - Uses iterators and collect

// Example solutions (commented out):
/*
pub fn run_exercises() {
    use crate::utils::print_section_header;
    use crate::utils::create_test_string;
    use crate::utils::create_test_vector;

    // Exercise 1
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }
    let s = create_test_string();
    let s = take_ownership(s);
    let s = take_ownership(s);
    println!("Final string: {}", s);

    // Exercise 2
    fn append_world(s: &mut String) {
        s.push_str(" world");
    }
    let mut s = create_test_string();
    append_world(&mut s);
    append_world(&mut s);
    println!("Modified string: {}", s);

    // Exercise 3
    fn get_first_last(v: &[i32]) -> Option<(&i32, &i32)> {
        if v.is_empty() {
            None
        } else {
            Some((&v[0], &v[v.len() - 1]))
        }
    }
    let v = create_test_vector();
    if let Some((first, last)) = get_first_last(&v) {
        println!("First: {}, Last: {}", first, last);
    }

    // Exercise 4
    struct TextWindow<'a> {
        text: &'a str,
        start: usize,
        end: usize,
    }
    impl<'a> TextWindow<'a> {
        fn new(text: &'a str, start: usize, end: usize) -> Self {
            Self { text, start, end }
        }
        fn get_text(&self) -> &'a str {
            &self.text[self.start..self.end]
        }
    }
    let text = "Hello, Rust!";
    let window = TextWindow::new(text, 0, 5);
    println!("Window text: {}", window.get_text());

    // Exercise 5
    fn remove_evens(mut v: Vec<i32>) -> Vec<i32> {
        v.retain(|&x| x % 2 != 0);
        v
    }
    let v = create_test_vector();
    let v = remove_evens(v);
    println!("Odd numbers: {:?}", v);
}
*/ 