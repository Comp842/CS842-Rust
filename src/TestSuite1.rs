use crate::TestObject::TestObject;
use crate::Timer::Timer;
use memory_stats::memory_stats;


pub(crate) fn test1() {
    // Start the timer
    let timer = Timer::start_timer();

    // Number of objects to create
    let num_objects = 1;
    let num_loops = 1000000000;

    // Run loop num_loops times
    for _ in 0..num_loops {
        let mut arr_of_test_obj = Vec::with_capacity(num_objects);

        // Create and discard short-lived objects (young generation)
        for _ in 0..num_objects {
            arr_of_test_obj.push(TestObject::new());
        }

        // Ensure that the vector is dropped, effectively discarding the objects
        arr_of_test_obj.clear();
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} objects to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test2() {
    // Start the timer
    let timer = Timer::start_timer();

    // Number of objects to create
    let num_objects = 100;
    let num_loops = 10000000;

    // Run loop num_loops times
    for _ in 0..num_loops {
        let mut arr_of_test_obj = Vec::with_capacity(num_objects);

        // Create and discard short-lived objects (young generation)
        for _ in 0..num_objects {
            arr_of_test_obj.push(TestObject::new());
        }

        // Ensure that the vector is dropped, effectively discarding the objects
        arr_of_test_obj.clear();
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} objects to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test3() {
    // Start the timer
    let timer = Timer::start_timer();

    // Number of objects to create
    let num_objects = 10000;
    let num_loops = 100000;

    // Run loop num_loops times
    for _ in 0..num_loops {
        let mut arr_of_test_obj = Vec::with_capacity(num_objects);

        // Create and discard short-lived objects (young generation)
        for _ in 0..num_objects {
            arr_of_test_obj.push(TestObject::new());
        }

        // Ensure that the vector is dropped, effectively discarding the objects
        arr_of_test_obj.clear();
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} objects to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test4() {
    // Start the timer
    let timer = Timer::start_timer();

    // Number of objects to create
    let num_objects = 100000;
    let num_loops = 10000;

    // Run loop num_loops times
    for _ in 0..num_loops {
        let mut arr_of_test_obj = Vec::with_capacity(num_objects);

        // Create and discard short-lived objects (young generation)
        for _ in 0..num_objects {
            arr_of_test_obj.push(TestObject::new());
        }

        // Ensure that the vector is dropped, effectively discarding the objects
        arr_of_test_obj.clear();
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} objects to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test5() {
    // Start the timer
    let timer = Timer::start_timer();

    // Number of objects to create
    let num_objects = 1000000;
    let num_loops = 1000;

    // Run loop num_loops times
    for _ in 0..num_loops {
        let mut arr_of_test_obj = Vec::with_capacity(num_objects);

        // Create and discard short-lived objects (young generation)
        for _ in 0..num_objects {
            arr_of_test_obj.push(TestObject::new());
        }

        // Ensure that the vector is dropped, effectively discarding the objects
        arr_of_test_obj.clear();
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} objects to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test6() {
    // Start the timer
    let timer = Timer::start_timer();

    // Number of objects to create
    let num_objects = 10000000;
    let num_loops = 100;

    // Run loop num_loops times
    for _ in 0..num_loops {
        // if let Some(usage) = memory_stats() {
        //     println!("{}", usage.virtual_mem);
        // } else {
        //     println!("Couldn't get the current memory usage :(");
        // }
        let mut arr_of_test_obj = Vec::with_capacity(num_objects);

        // Create and discard short-lived objects (young generation)
        for _ in 0..num_objects {
            arr_of_test_obj.push(TestObject::new());
        }
        // if let Some(usage) = memory_stats() {
        //     println!("{}", usage.virtual_mem);
        // } else {
        //     println!("Couldn't get the current memory usage :(");
        // }

        // Ensure that the vector is dropped, effectively discarding the objects
        arr_of_test_obj.clear();
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} objects to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test7() {
    // Start the timer
    let timer = Timer::start_timer();

    // Number of objects to create
    let num_objects = 1000000000;
    let num_loops = 1;

    // Run loop num_loops times
    for _ in 0..num_loops {
        let mut arr_of_test_obj = Vec::with_capacity(num_objects/100);

        // Create and discard short-lived objects (young generation)
        for i in 0..num_objects {
            arr_of_test_obj.push(TestObject::new());
            if (i%10000000 == 1 ){
                arr_of_test_obj.clear();
            }
        }

        // Ensure that the vector is dropped, effectively discarding the objects
        arr_of_test_obj.clear();
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} objects to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}