use std::collections::LinkedList;
use rand::Rng;
use crate::AdvancedLinkedList::AdvancedLinkedList;
use crate::TestObject::TestObject;
use crate::Timer::Timer;
use memory_stats::memory_stats;

pub(crate) fn test1() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 1;
    let num_loops = 100000000;

    for _ in 0..num_loops {
        let mut first_elem = Box::new(AdvancedLinkedList::new());
        let mut curr_elem = &mut first_elem;

        for _ in 0..num_objects {
            let next_elem = AdvancedLinkedList::new();
            curr_elem.set_next(Some(Box::new(next_elem)));
            curr_elem = &mut *curr_elem.next.as_mut().unwrap();
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} linked lists to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}pub(crate) fn test2() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 10;
    let num_loops = 10000000;

    for _ in 0..num_loops {
        let mut first_elem = Box::new(AdvancedLinkedList::new());
        let mut curr_elem = &mut first_elem;

        for _ in 0..num_objects {
            let next_elem = AdvancedLinkedList::new();
            curr_elem.set_next(Some(Box::new(next_elem)));
            curr_elem = &mut *curr_elem.next.as_mut().unwrap();
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} linked lists to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}pub(crate) fn test3() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 100;
    let num_loops = 1000000;

    for _ in 0..num_loops {
        let mut first_elem = Box::new(AdvancedLinkedList::new());
        let mut curr_elem = &mut first_elem;

        for _ in 0..num_objects {
            let next_elem = AdvancedLinkedList::new();
            curr_elem.set_next(Some(Box::new(next_elem)));
            curr_elem = &mut *curr_elem.next.as_mut().unwrap();
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} linked lists to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}pub(crate) fn test4() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 1000;
    let num_loops = 100000;

    for _ in 0..num_loops {
        let mut first_elem = Box::new(AdvancedLinkedList::new());
        let mut curr_elem = &mut first_elem;

        for _ in 0..num_objects {
            let next_elem = AdvancedLinkedList::new();
            curr_elem.set_next(Some(Box::new(next_elem)));
            curr_elem = &mut *curr_elem.next.as_mut().unwrap();
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} linked lists to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}
pub(crate) fn test5() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 1;
    let num_loops = 100000000;


    for _ in 0..num_loops {
        let mut list = LinkedList::new();

        for i in 0..num_objects {
            let mut rng = rand::thread_rng();
            let num_of_objects = rng.gen_range(1..=5);
            for _ in 0..num_of_objects {
                list.push_back(TestObject::new());
            }
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} linked lists to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}
pub(crate) fn test6() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 100;
    let num_loops = 1000000;


    for _ in 0..num_loops {
        let mut list = LinkedList::new();

        for i in 0..num_objects {
            let mut rng = rand::thread_rng();
            let num_of_objects = rng.gen_range(1..=5);
            for _ in 0..num_of_objects {
                list.push_back(TestObject::new());
            }
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} linked lists to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}
pub(crate) fn test7() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 10000;
    let num_loops = 10000;


    for _ in 0..num_loops {
        let mut list = LinkedList::new();

        for i in 0..num_objects {
            let mut rng = rand::thread_rng();
            let num_of_objects = rng.gen_range(1..=5);
            for _ in 0..num_of_objects {
                list.push_back(TestObject::new());
            }
        }
    }

    // Stop the timer and calculate the elapsed time
    let elapsed_seconds = timer.get_time_seconds();

    // Print the results
    println!(
        "Time taken for {} linked lists to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}
pub(crate) fn test8() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 1000000;
    let num_loops = 100;


    for _ in 0..num_loops {
        let mut list = LinkedList::new();
        // if let Some(usage) = memory_stats() {
        //     println!("{}", usage.virtual_mem);
        // } else {
        //     println!("Couldn't get the current memory usage :(");
        // }
        for i in 0..num_objects {
            let mut rng = rand::thread_rng();
            let num_of_objects = rng.gen_range(1..=5);
            for _ in 0..num_of_objects {
                list.push_back(TestObject::new());
            }
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
        "Time taken for {} linked lists to be created {} times: {} seconds",
        num_objects, num_loops, elapsed_seconds
    );
}