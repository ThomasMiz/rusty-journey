// Rust provides an implementation of channels:
use std::{sync::mpsc, thread, time::Duration}; // Multiple producer single consumer FIFO queue

fn main() {
    // Create a channel. We get the two ends of the channel, the transmit and receive ends.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi!");
        tx.send(val).unwrap();
        // Note: ownership of `val` is transferred to the channel!
    });

    let received = rx.recv().unwrap();
    println!("Received message from other thread: {}", received);
    // Received message from other thread: Hi!

    // We can also use the `rx.recv_timeout()` and `rx.try_recv()` methods.

    // There is a slight problem if we want to have multiple threads producing values, as
    // `tx` is being _moved_ into the closure of the first thread we spawn. This can be
    // easily solved by `.clone()`ing it:

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = ["Hola", "Como", "Estas", "Pedro"].into_iter().map(String::from);

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = ["Rindo", "Un", "Examen", "Hoy"].into_iter().map(String::from);

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(60));
        }
    });

    // Also note: we can use iterators over the receiving end! Iteration ends once
    // the channel gets closed.
    for received in rx {
        println!("Received: {received}");
    }
    // Received: Rindo
    // Received: Hola
    // Received: Un
    // Received: Como
    // Received: Examen
    // Received: Hoy
    // Received: Estas
    // Received: Pedro
}
