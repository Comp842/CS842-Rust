use std::collections::{BTreeSet, LinkedList};
use rand::Rng;
use crate::AdvancedLinkedList::AdvancedLinkedList;
use crate::TestObject::TestObject;
use crate::Timer::Timer;
use memory_stats::memory_stats;


pub(crate) fn test1() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 1;
    let num_loops = 1000000000;

    for _ in 0..num_loops {
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test2() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 10;
    let num_loops = 100000000;

    for _ in 0..num_loops {
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test3() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 100;
    let num_loops = 10000000;

    for _ in 0..num_loops {
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test4() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 1000;
    let num_loops = 1000000;

    for _ in 0..num_loops {
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test5() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 10000;
    let num_loops = 100000;

    for _ in 0..num_loops {
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test6() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 100000;
    let num_loops = 10000;

    for _ in 0..num_loops {
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test7() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 1000000;
    let num_loops = 1000;

    for _ in 0..num_loops {
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test8() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 10000000;
    let num_loops = 100;


    for _ in 0..num_loops {
        // if let Some(usage) = memory_stats() {
        //     println!("{}", usage.virtual_mem);
        // } else {
        //     println!("Couldn't get the current memory usage :(");
        // }
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
        // if let Some(usage) = memory_stats() {
        //     println!("{}", usage.virtual_mem);
        // } else {
        //     println!("Couldn't get the current memory usage :(");
        // }
    }


    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test9() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 100000000;
    let num_loops = 10;

    for _ in 0..num_loops {
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}

pub(crate) fn test10() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 1000000000;
    let num_loops = 1;

    for _ in 0..num_loops {
        let mut ts = BTreeSet::new();

        for i in 0..num_objects {
            ts.insert(i);
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} tree set to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}