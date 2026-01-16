// Example solutions (commented out):
/*
pub async fn run_exercises() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use tokio::time::{sleep, Duration};
    use rayon::prelude::*;

    print_section_header("Concurrency Exercises");

    // Exercise 1
    fn basic_threading() {
        let handles: Vec<_> = (0..3)
            .map(|i| {
                thread::spawn(move || {
                    println!("Thread {} running", i);
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
        print_test_result("Basic threading", true);
    }

    // Exercise 2
    fn message_passing() {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel();
        let producer = thread::spawn(move || {
            for i in 0..5 {
                tx.send(i).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
        let consumer = thread::spawn(move || {
            while let Ok(i) = rx.recv() {
                println!("Received: {}", i);
            }
        });
        producer.join().unwrap();
        consumer.join().unwrap();
        print_test_result("Message passing", true);
    }

    // Exercise 3
    fn shared_state() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..3 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Final count: {}", *counter.lock().unwrap());
        print_test_result("Shared state", true);
    }

    // Exercise 4
    async fn async_operations() {
        let tasks = create_test_async_tasks();
        let handles: Vec<_> = tasks.into_iter()
            .map(|duration| {
                tokio::spawn(async move {
                    sleep(duration).await;
                    println!("Task completed after {:?}", duration);
                })
            })
            .collect();
        for handle in handles {
            handle.await.unwrap();
        }
        print_test_result("Async operations", true);
    }

    // Exercise 5
    fn parallel_processing() {
        let numbers = create_test_vector();
        let sum: i32 = numbers.par_iter()
            .map(|&x| x * x)
            .sum();
        println!("Sum of squares: {}", sum);
        print_test_result("Parallel processing", true);
    }

    // Run all exercises
    basic_threading();
    message_passing();
    shared_state();
    async_operations().await;
    parallel_processing();
}
*/
