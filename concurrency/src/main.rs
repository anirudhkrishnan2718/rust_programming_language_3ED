use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3, 4];
    // needs the move keyword, since the main thread might finish running and drop v
    // before this thread gets a chance to finish
    let handle2 = thread::spawn(move || println!("Here is a vector {v:?}"));
    handle2.join().unwrap();

    // takes closure as argument, and works only as long as the main thread is active
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // spawned thread will be forced to finish before main thread begins
    // handle.join().unwrap();

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // calling join waits for the thread to finish, even though the main thread is
    // already done
    handle.join().unwrap();

    // define transmitter and receiver threads that will form a channel
    let (tx, rx) = mpsc::channel();

    // multiple produced single consumer
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("1 more"),
            String::from("1 messages"),
            String::from("1 are"),
            String::from("1 inside"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        // move ensures that the spawned thread owns tx now
        let vals = vec![
            String::from("2 Hi"),
            String::from("2 from"),
            String::from("2 the"),
            String::from("2 thread"),
        ];
        // transmitter is sending val from a spawned thread to the main thread
        // returns error if there is no main thread to send to
        for val in vals {
            tx.send(val).unwrap();
            // wait 1 second before sending each word through the channel
            thread::sleep(Duration::from_secs(1));
        }
        // borrowing rules throw error here, preventing concurrency bugs
        // println!("val is {val}");
    });

    for received in rx {
        // only prints words as they are received through the channel
        println!("Got: {received}");
    }
}
