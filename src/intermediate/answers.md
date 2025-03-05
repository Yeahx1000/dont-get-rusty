# Intermediate Level Answers

This guide contains solutions for all intermediate-level exercises. Try to solve the exercises yourself first, and only look at these answers when you're stuck or want to verify your solution.

## Ownership and Borrowing

### Exercise 1: Basic Ownership
```rust
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
```

### Exercise 2: Mutable References
```rust
fn append_world(s: &mut String) {
    s.push_str(" world");
}
```

### Exercise 3: Multiple References
```rust
fn get_first_last(v: &[i32]) -> Option<(&i32, &i32)> {
    if v.is_empty() {
        None
    } else {
        Some((&v[0], &v[v.len() - 1]))
    }
}
```

### Exercise 4: Lifetimes
```rust
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
```

### Exercise 5: Ownership and Move Semantics
```rust
fn remove_evens(mut v: Vec<i32>) -> Vec<i32> {
    v.retain(|&x| x % 2 != 0);
    v
}
```

## Collections

### Exercise 1: Vector Operations
```rust
fn vector_stats(v: &[i32]) -> Vec<f64> {
    let sum: i32 = v.iter().sum();
    let product: i32 = v.iter().product();
    let avg = sum as f64 / v.len() as f64;
    vec![sum as f64, product as f64, avg]
}
```

### Exercise 2: HashMap Basics
```rust
fn get_value(map: &HashMap<String, i32>, key: &str) -> Option<&i32> {
    map.get(key)
}
```

### Exercise 3: HashSet Operations
```rust
fn set_operations(v1: &[i32], v2: &[i32]) -> (HashSet<i32>, HashSet<i32>) {
    let set1: HashSet<_> = v1.iter().cloned().collect();
    let set2: HashSet<_> = v2.iter().cloned().collect();
    (set1.intersection(&set2).cloned().collect(),
     set1.union(&set2).cloned().collect())
}
```

### Exercise 4: Iterator Chains
```rust
fn process_strings(strings: Vec<String>) -> Vec<String> {
    strings.into_iter()
        .filter(|s| s.len() > 5)
        .map(|s| s.to_uppercase())
        .collect::<Vec<_>>()
}
```

### Exercise 5: Custom Collection
```rust
struct NumberSet {
    numbers: HashSet<i32>,
}

impl NumberSet {
    fn new() -> Self {
        Self {
            numbers: HashSet::new(),
        }
    }
    
    fn add(&mut self, n: i32) {
        self.numbers.insert(n);
    }
    
    fn remove(&mut self, n: i32) {
        self.numbers.remove(&n);
    }
    
    fn contains(&self, n: i32) -> bool {
        self.numbers.contains(&n)
    }
    
    fn sum(&self) -> i32 {
        self.numbers.iter().sum()
    }
}
```

## Error Handling

### Exercise 1: Basic Result
```rust
fn process_string(s: String) -> Result<String, String> {
    if s.len() > 5 {
        Ok(s.to_uppercase())
    } else {
        Err("String too short".to_string())
    }
}
```

### Exercise 2: Option Type
```rust
fn find_first_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x % 2 == 0).copied()
}
```

### Exercise 3: Custom Error Type
```rust
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
```

### Exercise 4: Error Propagation
```rust
fn read_json_file(path: &str) -> Result<Value, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let value: Value = serde_json::from_str(&contents)?;
    Ok(value)
}
```

### Exercise 5: Error Handling Patterns
```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("Failed to parse number: {}", e))
}
```

## Traits and Generics

### Exercise 1: Basic Trait
```rust
trait Printable {
    fn print(&self);
    fn print_with_prefix(&self, prefix: &str) {
        println!("{}: {}", prefix, self);
    }
}

impl Printable for i32 {
    fn print(&self) {
        println!("Number: {}", self);
    }
}

impl Printable for String {
    fn print(&self) {
        println!("String: {}", self);
    }
}
```

### Exercise 2: Generic Functions
```rust
fn find_max<T: Ord>(v: &[T]) -> Option<&T> {
    v.iter().max()
}
```

### Exercise 3: Trait Bounds
```rust
struct ValueHolder<T: Display> {
    value: T,
}

impl<T: Display> ValueHolder<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
    
    fn print(&self) {
        println!("Value: {}", self.value);
    }
    
    fn update(&mut self, new_value: T) {
        self.value = new_value;
    }
}
```

### Exercise 4: Associated Types
```rust
trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn remove(&mut self) -> Option<Self::Item>;
    fn is_empty(&self) -> bool;
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Container for Stack<T> {
    type Item = T;
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn remove(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
```

### Exercise 5: Multiple Trait Bounds
```rust
fn add_and_print<T: Add<Output = T> + Clone + Display>(a: T, b: T) -> T {
    let sum = a.clone() + b;
    println!("Sum: {}", sum);
    sum
}
``` 