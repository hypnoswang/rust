use std::thread::{self, JoinHandle};

fn move_closure() -> JoinHandle<()> {
    let items = vec![1, 2, 3];

    // items在函数move_closure结束后就会被销毁,所以这里我们必须使用move来获取所有权
    thread::spawn(move || {
        println!("The itimes are: {:#?}", items);
    })
}

fn main() {
    let h = move_closure();
    h.join().unwrap();

    println!("Main thread exited......");
}
