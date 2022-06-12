use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let h1 = thread::spawn(move || {
        for i in 0..11 {
            if i % 5 == 1 {
                println!("thread-1 print {}", i);
            } else if i % 5 == 0 {
                tx1.send(i).unwrap();
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = tx.clone();
    let h2 = thread::spawn(move || {
        for i in 0..11 {
            if i % 5 == 2 {
                println!("thread-2 print {}", i);
            } else if i % 5 > 2 {
                tx2.send(i).unwrap();
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let h3 = thread::spawn(move || loop {
        let v = rx
            .recv_timeout(Duration::from_secs(10))
            .unwrap_or_else(|e| {
                println!("Received err: {}", e.to_string());
                -1
            });

        if v == -1 {
            println!("Got value -1, thread-3 exiting ...");
            break;
        }

        println!("thread-3 print {}", v);
    });

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
}
