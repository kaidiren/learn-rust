use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

static NTHREADS: i32 = 100;

fn main() {
    println!("Hello, world!");
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs((id % 4) as u64));
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
    }

    // let mut ids = Vec::with_capacity(NTHREADS as usize);
    let mut ids = vec![];
    for _ in 0..NTHREADS {
        // ids.push(rx.recv());
        match rx.recv() {
            Ok(n) => {
                println!("{} received", n);
                ids.push(n);
            }
            Err(_) => {}
        };
    }
    println!("{:?}", ids);
}
