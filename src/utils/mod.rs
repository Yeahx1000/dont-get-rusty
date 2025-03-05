//! Utility functions shared across the project

#[allow(dead_code)]
pub fn print_section_header(title: &str) {
    println!("\n=== {} ===\n", title);
}

#[allow(dead_code)]
pub fn print_test_result(test_name: &str, passed: bool) {
    println!("{}: {}", test_name, if passed { "✅ PASSED" } else { "❌ FAILED" });
}

#[allow(dead_code)]
pub fn create_test_vector() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

#[allow(dead_code)]
pub fn create_test_string() -> String {
    String::from("Hello, Rust!")
}

#[allow(dead_code)]
pub fn create_test_hashmap() -> std::collections::HashMap<String, i32> {
    let mut map = std::collections::HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);
    map
}

#[allow(dead_code)]
pub fn create_test_hashset() -> std::collections::HashSet<i32> {
    vec![1, 2, 3, 4, 5].into_iter().collect()
}

#[allow(dead_code)]
pub fn create_test_strings() -> Vec<String> {
    vec![
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
        "programming".to_string(),
    ]
}

#[allow(dead_code)]
pub fn create_test_numbers() -> Vec<i32> {
    vec![1, 3, 5, 2, 4, 6, 8, 10]
}

#[allow(dead_code)]
pub fn create_test_points() -> Vec<(f64, f64)> {
    vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]
}

#[allow(dead_code)]
pub fn create_test_errors() -> Vec<Result<i32, String>> {
    vec![
        Ok(1),
        Err("Invalid input".to_string()),
        Ok(2),
        Err("Operation failed".to_string()),
        Ok(3),
    ]
}

#[allow(dead_code)]
pub fn create_test_options() -> Vec<Option<i32>> {
    vec![Some(1), None, Some(2), None, Some(3)]
}

#[allow(dead_code)]
pub fn create_test_async_tasks() -> Vec<tokio::time::Duration> {
    vec![
        tokio::time::Duration::from_millis(100),
        tokio::time::Duration::from_millis(200),
        tokio::time::Duration::from_millis(300),
    ]
} 