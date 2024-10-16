use crossbeam::channel;
use std::thread;
use std::time::Duration;

// Challenge: Make two child threads and give them each a receiving end to a channel.  From the
// main thread loop through several values and print each out and then send it to the channel.
// On the child threads print out the values you receive. Close the sending side in the main
// thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
// the child threads.

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    let (tx, rx) = channel::unbounded();
    let (tx1, rx1) = (tx.clone(), rx.clone());
    let (tx2, rx2) = (tx.clone(), rx.clone());

    let handle1 = thread::spawn(move || {
        for msg in rx1 {
            println!("handel1 thread: Received {}", msg);
            thread::sleep(Duration::from_millis(500));
        }
    });

    let handle2 = thread::spawn(move || {
        for msg in rx2 {
            println!("handle2 thread: Received {}", msg);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for n in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: sent {}", n);
        tx.send(n).unwrap();
    }

    drop(tx);
    drop(tx1);
    drop(tx2);

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main thread: Exiting.")
}
