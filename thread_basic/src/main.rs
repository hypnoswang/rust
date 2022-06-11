use std::thread::{self, JoinHandle};

fn move_closure() -> JoinHandle<()> {
    let items = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("The itimes are: {:#?}", items);
    });

    handle
}

fn main() {
    let h = move_closure();
    h.join().unwrap();

    println!("Main thread exited......");
}
