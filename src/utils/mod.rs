//! Utility functions shared across the project

#[allow(dead_code)]
pub fn print_section_header(title: &str) {
    println!("\n=== {} ===\n", title);
}

#[allow(dead_code)]
pub fn print_test_result(test_name: &str, passed: bool) {
    println!(
        "{}: {}",
        test_name,
        if passed { "✅ PASSED" } else { "❌ FAILED" }
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_create_test_vector() {
        let vec = create_test_vector();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_create_test_string() {
        let s = create_test_string();
        assert_eq!(s, "Hello, Rust!");
    }

    #[test]
    fn test_create_test_hashmap() {
        let map = create_test_hashmap();
        let mut expected = HashMap::new();
        expected.insert("one".to_string(), 1);
        expected.insert("two".to_string(), 2);
        expected.insert("three".to_string(), 3);
        assert_eq!(map, expected);
    }

    #[test]
    fn test_create_test_hashset() {
        let set = create_test_hashset();
        let expected: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
        assert_eq!(set, expected);
    }

    #[test]
    fn test_create_test_strings() {
        let strings = create_test_strings();
        assert_eq!(strings, vec!["hello", "world", "rust", "programming"]);
    }

    #[test]
    fn test_create_test_numbers() {
        let numbers = create_test_numbers();
        assert_eq!(numbers, vec![1, 3, 5, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_create_test_points() {
        let points = create_test_points();
        assert_eq!(points, vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]);
    }

    #[test]
    fn test_create_test_errors() {
        let errors = create_test_errors();
        assert_eq!(errors.len(), 5);
        assert_eq!(errors[0], Ok(1));
        assert!(matches!(errors[1], Err(_)));
        assert_eq!(errors[2], Ok(2));
        assert!(matches!(errors[3], Err(_)));
        assert_eq!(errors[4], Ok(3));
    }

    #[test]
    fn test_create_test_options() {
        let options = create_test_options();
        assert_eq!(options.len(), 5);
        assert_eq!(options[0], Some(1));
        assert_eq!(options[1], None);
        assert_eq!(options[2], Some(2));
        assert_eq!(options[3], None);
        assert_eq!(options[4], Some(3));
    }

    #[test]
    fn test_create_test_async_tasks() {
        let tasks = create_test_async_tasks();
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0], tokio::time::Duration::from_millis(100));
        assert_eq!(tasks[1], tokio::time::Duration::from_millis(200));
        assert_eq!(tasks[2], tokio::time::Duration::from_millis(300));
    }
}
