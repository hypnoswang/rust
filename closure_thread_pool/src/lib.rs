use std::marker::Send;
use std::ops::Drop;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Cmd {
    Exec(Job),
    Stop,
}

struct Worker {
    idx: usize,
    handle: Option<JoinHandle<()>>,

    counter: Arc<Mutex<u32>>,
}

impl Worker {
    fn new(idx: usize, rx: Arc<Mutex<Receiver<Cmd>>>) -> Worker {
        let c: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

        let cc = c.clone();
        let h = thread::spawn(move || loop {
            let c = rx.lock().unwrap().recv().unwrap_or_else(|e| {
                eprintln!("Got errors when waiting for jobs: {}", e.to_string());
                Cmd::Stop
            });

            match c {
                Cmd::Stop => {
                    eprintln!("Received stop cmd, thread-{} exiting....", idx);
                    break;
                }
                Cmd::Exec(job) => {
                    println!("thread-{} begin to work", idx);
                    job();
                    println!("thread-{} complete work", idx);

                    let mut cnt = cc.lock().unwrap(); // 锁会一直持有到本分支结束
                    *cnt += 1;
                }
            }
        });

        Worker {
            idx,
            handle: Some(h),
            counter: c,
        }
    }

    fn info(&self) {
        let cnt = *self.counter.lock().unwrap();
        println!("thread-{} handled total {} requests", self.idx, cnt);
    }

    fn wait(&mut self) {
        self.handle.take().unwrap().join().unwrap();
    }
}

pub struct Pool {
    sender: Sender<Cmd>,
    workers: Vec<Worker>,
}

impl Pool {
    pub fn new(max: usize) -> Pool {
        let (tx, rx) = mpsc::channel();
        let rxp = Arc::new(Mutex::new(rx));

        let mut pool = Pool {
            sender: tx,
            workers: vec![],
        };

        for i in 0..max {
            pool.workers.push(Worker::new(i, Arc::clone(&rxp)));
        }

        pool
    }

    pub fn dispatch(&mut self, job: Job) -> Result<(), &'static str> {
        let res = self.sender.send(Cmd::Exec(job));
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Dispatch Job failed: {}", e.to_string());
                Err("Dispatch job failed")
            }
        }
    }

    pub fn info(&self) {
        self.workers.iter().for_each(|w| w.info());
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Cmd::Stop).unwrap();
        }

        for worker in &mut self.workers {
            worker.wait();
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let mut p = Pool::new(4);

        for i in 0..100 {
            p.dispatch(Box::new(move || {
                println!("Starting job {}", i);
                thread::sleep(Duration::from_millis(100));
                println!("Finished job {}", i);
            }))
            .unwrap();
        }

        thread::sleep(Duration::from_secs(5));
        p.info();
    }
}
