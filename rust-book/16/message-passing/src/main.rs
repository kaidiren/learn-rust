use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let sender2 = sender.clone();
    thread::spawn(move || {
        let mut index = 0;
        loop {
            index += 1;
            let val = format!("from sender with index: {}", index);
            sender.send(val.clone()).unwrap();
            sender2.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
            if index > 100 {
                break;
            }
        }
    });

    for received in receiver {
        println!("Got: {}", received);
    }
}
