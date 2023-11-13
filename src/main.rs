use std::mem;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    // test1();
    test2();
}


fn expensive_function() {
    let num_assignments = 1000000000;

    // Create an integer value and wrap it in a Box and an Rc
    let value: i32 = 0;
    let mut boxed_value = Box::new(value);

    for _ in 0..num_assignments {
        *boxed_value = 42;
    }
}
fn expensive_function2(){
    let value: i32 = 0;

    let num_assignments = 1000000000;
    let rc_value = Rc::new(value);

    for _ in 0..num_assignments {
        let _cloned_rc = rc_value.clone();
    }
}

fn expensive_function3() {
    let num_assignments = 1000000000;

    // Create a mutable integer variable and a mutable pointer to it
    let mut value: i32 = 0;
    let mut pointer: *mut i32 = &mut value as *mut i32;

    // Perform the pointer assignments
    for _ in 0..num_assignments {
        unsafe {
            *pointer = 42;
        }
    }
}

fn expensive_function4() {
    let num_assignments = 1000000000;

    // Create a mutable integer variable and a mutable pointer to it
    let mut value: i32 = 0;

    // Perform the pointer assignments
    for _ in 0..num_assignments {
            value = 42;
    }
}

fn test1(){
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let start = Instant::now();
    expensive_function2();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function2() is: {:?}", duration);

    let start = Instant::now();
    expensive_function3();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function3() is: {:?}", duration);

    let start = Instant::now();
    expensive_function4();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function4() is: {:?}", duration);
}

fn test2(){
    let start = Instant::now();

    // Number of objects to create
    let num_objects = 1000000000;

    // Create and discard objects (young generation)
    for _ in 0..num_objects {
        let obj = Box::new(0); // Box represents a heap-allocated object
        // Simulate some usage or operations with the object
    }

    // Trigger the collection of objects (young generation)
    // Sleep to mimic the garbage collection process (for demonstration purposes)
    let mut vec = Vec::new();
    vec.push(1);
    drop(vec); // This will free objects created in the previous loop

    std::thread::sleep(std::time::Duration::from_secs(100));

    // Create more objects to promote some to the old generation
    for _ in 0..num_objects {
        let obj = Box::new(0);
        // Simulate some usage or operations with the object
    }

    let mut vec = Vec::new();
    vec.push(1);
    // Trigger the collection of objects (young generation)
    drop(vec);

    std::thread::sleep(std::time::Duration::from_secs(100));

    // Continue with your monitoring and analysis
    // Rust's ownership system and smart pointers manage memory efficiently

    let duration = start.elapsed();

    println!("Time elapsed for creation of objects is: {:?}", duration);
}