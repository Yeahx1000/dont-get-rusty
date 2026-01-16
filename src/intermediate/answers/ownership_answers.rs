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
