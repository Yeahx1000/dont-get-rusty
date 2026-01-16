// Example solutions (commented out):
/*
pub fn run_exercises() {
    // Exercise 1
    macro_rules! sum {
        ($($x:expr),*) => {{
            let mut sum = 0;
            $(sum += $x;)*
            sum
        }};
    }
    println!("Sum: {}", sum!(1, 2, 3, 4, 5));

    // Exercise 2
    macro_rules! match_type {
        (number $x:expr) => { format!("Number: {}", $x) };
        (text $x:expr) => { format!("Text: {}", $x) };
        ($x:expr) => { format!("Unknown: {}", $x) };
    }
    println!("{}", match_type!(number 42));
    println!("{}", match_type!(text "hello"));

    // Exercise 3
    macro_rules! vec_of {
        ($($x:expr),*) => {{
            let mut v = Vec::new();
            $(v.push($x);)*
            v
        }};
    }
    let v = vec_of!(1, "two", 3.0);
    println!("Vector: {:?}", v);

    // Exercise 4
    // In a separate proc-macro crate:
    /*
    #[proc_macro_derive(CustomDebug)]
    pub fn derive_custom_debug(input: TokenStream) -> TokenStream {
        // Implementation here
    }
    */

    // Exercise 5
    // In a separate proc-macro crate:
    /*
    #[proc_macro_attribute]
    pub fn log_function(attr: TokenStream, item: TokenStream) -> TokenStream {
        // Implementation here
    }
    */
}
*/
