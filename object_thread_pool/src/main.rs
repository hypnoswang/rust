use object_thread_pool::Job;
use object_thread_pool::Pool;
use std::thread;
use std::time::Duration;

struct Cleaning {}

impl Job for Cleaning {
    fn run(&mut self) {
        println!("Homer is cleaning");
    }
}

struct Digging {}

impl Job for Digging {
    fn run(&mut self) {
        println!("Homer is digging");
    }
}

fn run() {
    let mut pool = Pool::new(10);
    for i in 0..100 {
        let job: Box<dyn Job + Send + 'static>;
        if i % 2 == 0 {
            job = Box::new(Cleaning {});
        } else {
            job = Box::new(Digging {});
        }

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
