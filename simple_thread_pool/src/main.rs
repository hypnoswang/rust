use simple_thread_pool::Job;
use simple_thread_pool::Pool;
use std::thread;
use std::time::Duration;

struct MyJob {}

impl Job for MyJob {
    fn run(&mut self) {
        println!("Homer is working");
    }
}

fn run() {
    let mut pool: Pool<MyJob> = Pool::new(10);
    for i in 0..100 {
        let job = MyJob {};
        match pool.dispatch(job) {
            Ok(()) => {
                println!("Successfully dispatch job {}", i);
            }
            Err(s) => {
                println!("Failed dispatch job {}, {}", i, s);
            }
        }
    }

    thread::sleep(Duration::from_secs(1));

    pool.info();

    pool.stop();
}

fn main() {
    run()
}
