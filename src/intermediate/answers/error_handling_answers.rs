// Example solutions (commented out):
/*
pub fn run_exercises() {
    use std::error::Error;
    use std::fmt;
    use serde_json::Value;
    use std::fs;

    print_section_header("Error Handling Exercises");

    // Exercise 1
    fn process_string(s: String) -> Result<String, String> {
        if s.len() > 5 {
            Ok(s.to_uppercase())
        } else {
            Err("String too short".to_string())
        }
    }
    let s = create_test_string();
    match process_string(s) {
        Ok(s) => println!("Processed: {}", s),
        Err(e) => println!("Error: {}", e),
    }
    print_test_result("String processing", true);

    // Exercise 2
    fn find_first_even(v: &[i32]) -> Option<i32> {
        v.iter().find(|&&x| x % 2 == 0).copied()
    }
    let v = create_test_numbers();
    match find_first_even(&v) {
        Some(n) => println!("First even: {}", n),
        None => println!("No even numbers found"),
    }
    print_test_result("Find first even", true);

    // Exercise 3
    #[derive(Debug)]
    enum CalculatorError {
        DivisionByZero,
        InvalidOperation(String),
    }
    impl fmt::Display for CalculatorError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                CalculatorError::DivisionByZero => write!(f, "Division by zero"),
                CalculatorError::InvalidOperation(op) => write!(f, "Invalid operation: {}", op),
            }
        }
    }
    impl Error for CalculatorError {}
    print_test_result("Custom error type", true);

    // Exercise 4
    fn read_json_file(path: &str) -> Result<Value, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let value: Value = serde_json::from_str(&contents)?;
        Ok(value)
    }
    print_test_result("Error propagation", true);

    // Exercise 5
    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|e| format!("Failed to parse number: {}", e))
    }
    match parse_number("42") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    print_test_result("Number parsing", true);

    // Bonus: Process test errors
    let errors = create_test_errors();
    for result in errors {
        match result {
            Ok(n) => println!("Success: {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    // Process test options
    let options = create_test_options();
    for opt in options {
        match opt {
            Some(n) => println!("Some: {}", n),
            None => println!("None"),
        }
    }
}
*/
