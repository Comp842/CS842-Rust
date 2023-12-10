use crate::Timer::Timer;
use memory_stats::memory_stats;

// Number of assignments
const NUM_ASSIGNMENTS: usize = 1_000_000_000;

pub(crate) fn test1() {

    let mut value: usize = 0;

    // Start the timer
    let timer = Timer::start_timer();
    // for i in 0..200{
    //     if let Some(usage) = memory_stats() {
    //         println!("{}", usage.virtual_mem);
    //     } else {
    //         println!("Couldn't get the current memory usage :(");
    //     }

        // Perform the assignments
        for i in 0..NUM_ASSIGNMENTS {
            value = i;
        }

        // if let Some(usage) = memory_stats() {
        //     println!("{}", usage.virtual_mem);
        // } else {
        //     println!("Couldn't get the current memory usage :(");
        // }
    // }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} usize assignments: {} seconds",
        NUM_ASSIGNMENTS, elapsed_seconds
    );
}

pub(crate) fn test2() {
    let mut value2: Box<usize>;

    // Start the timer
    let timer = Timer::start_timer();

    // Perform the assignments
    for i in 0..NUM_ASSIGNMENTS {
        value2 = Box::new(i);
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} Box usize assignments using Box: {} seconds",
        NUM_ASSIGNMENTS, elapsed_seconds
    );
}

pub(crate) fn test3() {
    let mut value_arr = vec![0; NUM_ASSIGNMENTS];
    // Start the timer
    let timer = Timer::start_timer();

    for i in 0..NUM_ASSIGNMENTS {
        value_arr.push(i);
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} usize assignments stored in array: {} seconds",
        NUM_ASSIGNMENTS, elapsed_seconds
    );
}


pub(crate) fn test4() {
    let mut value: usize;
    let mut value_arr: Vec<Box<usize>> = vec![Box::new(0); NUM_ASSIGNMENTS / 4];

    // Start the timer
    let timer = Timer::start_timer();



    // Perform the assignments
    for i in 0..NUM_ASSIGNMENTS {
        value = i;
        value_arr[i % 4] = Box::new(value);
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} Box usize assignments stored in array: {} seconds",
        NUM_ASSIGNMENTS, elapsed_seconds
    );
}

pub(crate) fn test5() {

    let mut value: f64 = 0.0;

    // Start the timer
    let timer = Timer::start_timer();

    // Perform the assignments
    for i in 0..NUM_ASSIGNMENTS {
        value = i as f64;
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} f64 assignments: {} seconds",
        NUM_ASSIGNMENTS, elapsed_seconds
    );
}

pub(crate) fn test6() {
    let mut value2: Box<f64>;

    // Start the timer
    let timer = Timer::start_timer();

    // Perform the assignments
    for i in 0..NUM_ASSIGNMENTS {
        value2 = Box::new(i as f64);
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} Box of f64 assignments using Box: {} seconds",
        NUM_ASSIGNMENTS, elapsed_seconds
    );
}

pub(crate) fn test7() {
    let mut value_arr = vec![0f64; NUM_ASSIGNMENTS];
    // Start the timer
    let timer = Timer::start_timer();

    for i in 0..NUM_ASSIGNMENTS {
        value_arr.push(i as f64);
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} f64 assignments stored in array: {} seconds",
        NUM_ASSIGNMENTS, elapsed_seconds
    );
}


pub(crate) fn test8() {
    let mut value: usize;
    let mut value_arr: Vec<Box<f64>> = vec![Box::new(0.0); NUM_ASSIGNMENTS / 4];
    // Start the timer
    let timer = Timer::start_timer();



    // Perform the assignments
    for i in 0..NUM_ASSIGNMENTS {
        value = i;
        value_arr[i % 4] = Box::new(value as f64);
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} box f64 assignments stored in array: {} seconds",
        NUM_ASSIGNMENTS, elapsed_seconds
    );
}