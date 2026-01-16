// Example solutions (commented out):
/*
pub fn run_exercises() {
    // Exercise 1
    unsafe fn deref_raw(ptr: *const i32) -> Option<i32> {
        if ptr.is_null() {
            None
        } else {
            Some(*ptr)
        }
    }
    let x = 42;
    let ptr = &x as *const i32;
    unsafe {
        match deref_raw(ptr) {
            Some(val) => println!("Value: {}", val),
            None => println!("Null pointer"),
        }
    }

    // Exercise 2
    fn modify_through_ptr(value: &mut i32) {
        let ptr = value as *mut i32;
        unsafe {
            *ptr = 100;
        }
    }
    let mut x = 42;
    modify_through_ptr(&mut x);
    println!("Modified value: {}", x);

    // Exercise 3
    unsafe trait UnsafeCounter {
        unsafe fn increment(&mut self);
        unsafe fn decrement(&mut self);
    }
    struct Counter(i32);
    unsafe impl UnsafeCounter for Counter {
        unsafe fn increment(&mut self) {
            self.0 += 1;
        }
        unsafe fn decrement(&mut self) {
            self.0 -= 1;
        }
    }

    // Exercise 4
    unsafe fn allocate_and_init<T>(size: usize) -> *mut T {
        let ptr = std::alloc::alloc(std::alloc::Layout::array::<T>(size).unwrap()) as *mut T;
        if !ptr.is_null() {
            for i in 0..size {
                std::ptr::write(ptr.add(i), std::mem::zeroed());
            }
        }
        ptr
    }
    unsafe fn deallocate<T>(ptr: *mut T, size: usize) {
        if !ptr.is_null() {
            std::alloc::dealloc(
                ptr as *mut u8,
                std::alloc::Layout::array::<T>(size).unwrap(),
            );
        }
    }

    // Exercise 5
    struct SafePtr<T> {
        ptr: *mut T,
    }
    impl<T> SafePtr<T> {
        fn new(value: T) -> Self {
            let ptr = Box::into_raw(Box::new(value));
            Self { ptr }
        }
        unsafe fn get(&self) -> &T {
            &*self.ptr
        }
        unsafe fn get_mut(&mut self) -> &mut T {
            &mut *self.ptr
        }
    }
    impl<T> Drop for SafePtr<T> {
        fn drop(&mut self) {
            unsafe {
                if !self.ptr.is_null() {
                    let _ = Box::from_raw(self.ptr);
                }
            }
        }
    }
}
*/
