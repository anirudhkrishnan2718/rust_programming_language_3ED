use std::time::Duration;

fn main() {
    // this makes the top level function async, since main itself cannot be async
    // trpl::block_on(async {}) is boilerplate for async main
    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(250)).await;
            }
        };
        // awaiting each async block immediately after defining it disables concurrent behavior
        // fut1.await;
        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(250)).await;
            }
        };
        // fut2.await;

        // futures are lazy, which means that the first task only starts at this point, after
        // the second task has finished
        // fut1.await;

        // wait for both threads to end before finishing the program
        trpl::join(fut1, fut2).await;
    });

    trpl::block_on(async {
        // asynchronous channel receiver has to be mutable
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx1_fut = async move {
            let vals = vec![
                String::from("1 More"),
                String::from("1 messages"),
                String::from("1 for"),
                String::from("1 you"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        // to allow each word to be received immediately, the sender and receiver need to be
        // separate async blocks
        let tx_fut = async move {
            // the move is needed so that tx is moved into this block, and dropped after all 4
            // words are sent. This is needed for the channel to close and the program to finish
            let vals = vec![
                String::from("0 Hi"),
                String::from("0 from"),
                String::from("0 the"),
                String::from("0 future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            // while loop runs until the pattern matches Some(value), and captures the variable
            // in value for use within the loop
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // the sender and receiver async blocks are joined, and the fair runtime executes them
        // words are printed by the receiver one at a time

        // one producer one consumer
        // trpl::join(tx_fut, rx_fut).await;

        // multiple producers one consumer
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    })
}
