// Example solutions (commented out):
/*
pub fn run_exercises() {
    use std::fmt::Display;

    // Exercise 1
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

    // Exercise 2
    fn find_max<T: Ord>(v: &[T]) -> Option<&T> {
        v.iter().max()
    }
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Max number: {:?}", find_max(&numbers));

    // Exercise 3
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

    // Exercise 4
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

    // Exercise 5
    fn add_and_print<T: Add<Output = T> + Clone + Display>(a: T, b: T) -> T {
        let sum = a.clone() + b;
        println!("Sum: {}", sum);
        sum
    }
    let result = add_and_print(5, 3);
    println!("Final result: {}", result);
}
*/
