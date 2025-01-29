use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

fn channels() {
    const N: i32 = 10;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let handles = (0..N).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            // don't use the result
            let _ = _tx.send(i).unwrap();
        })
    });
    // close all threads
    for h in handles {
        h.join().unwrap();
    }
    // receive N times
    let numbers: Vec<i32> = (0..N).map(|_|
        rx.recv().unwrap()
    ).collect();

    println!("{:?}", numbers);

}

fn main() {
    channels();
}
