use std::collections::LinkedList;
use crate::SimpleLinkedList::SimpleLinkedList;
use crate::Timer::Timer;

pub(crate) fn test1() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 1;
    let num_loops = 100000000;

    for _ in 0..num_loops {
        let mut first_elem = Box::new(SimpleLinkedList::new());
        let mut curr_elem = &mut first_elem;

        for _ in 0..num_objects {
            let next_elem = SimpleLinkedList::new();
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
        let mut first_elem = Box::new(SimpleLinkedList::new());
        let mut curr_elem = &mut first_elem;

        for _ in 0..num_objects {
            let next_elem = SimpleLinkedList::new();
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
        let mut first_elem = Box::new(SimpleLinkedList::new());
        let mut curr_elem = &mut first_elem;

        for _ in 0..num_objects {
            let next_elem = SimpleLinkedList::new();
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
        let mut first_elem = Box::new(SimpleLinkedList::new());
        let mut curr_elem = &mut first_elem;

        for _ in 0..num_objects {
            let next_elem = SimpleLinkedList::new();
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
            list.push_back(i);
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
            list.push_back(i);
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
            list.push_back(i);
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

        for i in 0..num_objects {
            list.push_back(i);
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
pub(crate) fn test9() {
    // Start the timer
    let timer = Timer::start_timer();

    let num_objects = 100000000;
    let num_loops = 1;


    for _ in 0..num_loops {
        let mut list = LinkedList::new();

        for i in 0..num_objects {
            list.push_back(i);
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