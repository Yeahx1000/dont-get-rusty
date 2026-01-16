// Example solutions (commented out):
/*
pub fn run_exercises() {
    use std::ffi::{CString, CStr};

    // Exercise 1
    extern "C" {
        fn strlen(s: *const std::os::raw::c_char) -> usize;
    }
    fn safe_strlen(s: &str) -> usize {
        let c_string = CString::new(s).unwrap();
        unsafe { strlen(c_string.as_ptr()) }
    }
    println!("Length: {}", safe_strlen("hello"));

    // Exercise 2
    fn rust_to_c(s: &str) -> CString {
        CString::new(s).unwrap()
    }
    unsafe fn c_to_rust(s: *const std::os::raw::c_char) -> String {
        CStr::from_ptr(s).to_string_lossy().into_owned()
    }

    // Exercise 3
    #[repr(C)]
    struct CPoint {
        x: f64,
        y: f64,
    }
    struct Point {
        x: f64,
        y: f64,
    }
    impl Point {
        fn from_c(point: CPoint) -> Self {
            Self {
                x: point.x,
                y: point.y,
            }
        }
        fn to_c(&self) -> CPoint {
            CPoint {
                x: self.x,
                y: self.y,
            }
        }
    }

    // Exercise 4
    type Callback = extern "C" fn(x: i32) -> i32;
    extern "C" {
        fn register_callback(cb: Callback);
    }
    extern "C" fn rust_callback(x: i32) -> i32 {
        x * 2
    }
    unsafe {
        register_callback(rust_callback);
    }

    // Exercise 5
    #[derive(Debug)]
    enum FFIError {
        InvalidInput,
        OperationFailed,
        Unknown,
    }
    fn convert_error(code: i32) -> Result<(), FFIError> {
        match code {
            0 => Ok(()),
            1 => Err(FFIError::InvalidInput),
            2 => Err(FFIError::OperationFailed),
            _ => Err(FFIError::Unknown),
        }
    }
}
*/
