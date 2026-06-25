use std::{thread, vec};

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure {list:?}");
    let mut borrows_mutably = || list.push(7);

    // this line throws an error because the closure has a mutable reference
    // println!("Before calling closure {list:?}");
    borrows_mutably();
    println!("After calling closure {list:?}");

    let list2 = vec!['A', 'B', 'C'];
    println!("Before defining closure {list2:?}");

    // move keyword moves the variable into the closure, instead of just
    // borrowing immutably
    thread::spawn(move || println!("From thread: {list2:?}"))
        .join()
        .unwrap();
    // main thread does not have access to list2, throwing an error
    // println!("After moving into closure {list2:?}");

    let mut list3 = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // sort_by_key takes an FnMut closure, and calls it once for each element
    // of the list
    let mut num_sort_operations = 0;
    list3.sort_by_key(|r| {
        // closure only takes a mutable reference to num_sort_operations
        // and thus can be called more than once
        num_sort_operations += 1;
        r.width
    });
    println!("{list3:#?} was sorted in {num_sort_operations} operations");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
