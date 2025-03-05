# Beginner Level Answers

This guide contains solutions for all beginner-level exercises. Try to solve the exercises yourself first, and only look at these answers when you're stuck or want to verify your solution.

## Variables and Basic Types

### Exercise 1: Variable Declaration and Mutability
```rust
fn exercise1() {
    let mut age = 25;
    println!("Age: {}", age);
    age = 26;
    println!("New age: {}", age);
}
```

### Exercise 2: Type Inference
```rust
fn exercise2() {
    let integer = 42;        // i32
    let float = 3.14;       // f64
    let boolean = true;     // bool
    let character = 'A';    // char

    println!("Integer: {} (type: {})", integer, std::mem::size_of_val(&integer));
    println!("Float: {} (type: {})", float, std::mem::size_of_val(&float));
    println!("Boolean: {} (type: {})", boolean, std::mem::size_of_val(&boolean));
    println!("Character: {} (type: {})", character, std::mem::size_of_val(&character));
}
```

### Exercise 3: Constants
```rust
fn exercise3() {
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);
}
```

### Exercise 4: Shadowing
```rust
fn exercise4() {
    let x = 5;
    println!("First x: {}", x);
    let x = x + 1;
    println!("Second x: {}", x);
    let x = x * 2;
    println!("Third x: {}", x);
}
```

### Exercise 5: Type Annotations
```rust
fn exercise5() {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    println!("Annotated integer: {}", integer);
    println!("Annotated float: {}", float);
    println!("Annotated boolean: {}", boolean);
}
```

## Functions and Closures

### Exercise 1: Basic Functions
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Exercise 2: Multiple Return Values
```rust
fn stats(numbers: &[i32]) -> (i32, f64, i32) {
    let sum: i32 = numbers.iter().sum();
    let avg = sum as f64 / numbers.len() as f64;
    let max = *numbers.iter().max().unwrap_or(&0);
    (sum, avg, max)
}
```

### Exercise 3: Closures
```rust
fn apply_closure<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// Usage:
let add_one = |x| x + 1;
let multiply_by_two = |x| x * 2;
apply_closure(add_one, 5);      // Returns 6
apply_closure(multiply_by_two, 5); // Returns 10
```

### Exercise 4: Function Pointers
```rust
fn apply_function(f: fn(i32) -> i32, start: i32, end: i32) -> Vec<i32> {
    (start..=end).map(f).collect()
}

fn square(x: i32) -> i32 {
    x * x
}

// Usage:
let squares = apply_function(square, 1, 5);
// Returns [1, 4, 9, 16, 25]
```

### Exercise 5: Methods
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
```

## Control Flow

### Exercise 1: if/else Expressions
```rust
fn check_number(n: i32) -> &'static str {
    if n < 5 {
        "Too small"
    } else if n > 10 {
        "Too big"
    } else {
        "Just right"
    }
}
```

### Exercise 2: Multiple Conditions
```rust
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
```

### Exercise 3: Loops
```rust
fn exercise3() {
    // For loop
    println!("Counting up:");
    for i in 1..=5 {
        println!("{}", i);
    }

    // While loop
    println!("Counting down:");
    let mut count = 5;
    while count > 0 {
        println!("{}", count);
        count -= 1;
    }
}
```

### Exercise 4: Match Expression
```rust
fn number_to_text(n: i32) -> &'static str {
    match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Many",
    }
}
```

### Exercise 5: Pattern Matching with Ranges
```rust
fn age_category(age: u32) -> &'static str {
    match age {
        0..=12 => "Child",
        13..=19 => "Teenager",
        20..=64 => "Adult",
        _ => "Senior",
    }
}
```

### Bonus Challenge: Nested Match
```rust
fn match_booleans(a: bool, b: bool) -> &'static str {
    match (a, b) {
        (true, true) => "Both true",
        (true, false) => "First true, second false",
        (false, true) => "First false, second true",
        (false, false) => "Both false",
    }
}
``` 