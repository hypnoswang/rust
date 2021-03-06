use std::marker::Send;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub trait Job {
    fn run(&mut self);
}

enum Cmd {
    Exec(Box<dyn Job + Send>),
    Stop,
}

struct Worker {
    idx: usize,
    handle: Option<JoinHandle<()>>,
    sender: Sender<Cmd>,

    counter: Arc<Mutex<u32>>,
}

impl Worker {
    fn new(idx: usize) -> Worker {
        let c: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

        let cc = c.clone();
        let (tx, rx) = mpsc::channel();
        let h = thread::spawn(move || loop {
            let c: Cmd = rx.recv().unwrap_or_else(|e| {
                eprintln!("Got errors when waiting for jobs: {}", e.to_string());
                Cmd::Stop
            });

            match c {
                Cmd::Stop => {
                    eprintln!("Received stop cmd, thread-{} exiting....", idx);
                    break;
                }
                Cmd::Exec(mut job) => {
                    println!("thread-{} begin to work", idx);
                    job.run();
                    println!("thread-{} complete work", idx);

                    let mut cnt = cc.lock().unwrap(); // 锁会一直持有到本分支结束
                    *cnt += 1;
                }
            }
        });

        Worker {
            idx,
            handle: Some(h),
            sender: tx,
            counter: c,
        }
    }

    fn info(&self) {
        let cnt = *self.counter.lock().unwrap();
        println!("thread-{} handled total {} requests", self.idx, cnt);
    }

    fn dispatch(&self, job: Box<dyn Job + Send>) -> Result<(), &'static str> {
        if let Err(e) = self.sender.send(Cmd::Exec(job)) {
            println!("dispatch job failed, {:?}", e);
            return Err("dispatch failed");
        }

        Ok(())
    }

    fn stop(mut self) {
        self.sender.send(Cmd::Stop).unwrap();
        if self.handle.is_some() {
            self.handle.take().unwrap().join().unwrap();
        }
    }
}

pub struct Pool {
    last_worker: usize,
    workers: Vec<Worker>,
}

impl Pool {
    pub fn new(max: usize) -> Pool {
        let mut pool = Pool {
            last_worker: 0,
            workers: vec![],
        };

        for i in 0..max {
            pool.workers.push(Worker::new(i));
        }

        pool
    }

    pub fn dispatch(&mut self, job: Box<dyn Job + Send>) -> Result<(), &'static str> {
        self.last_worker = (self.last_worker + 1) % self.workers.len();
        let worker = self.workers.get(self.last_worker).unwrap();
        worker.dispatch(job)
    }

    pub fn info(&self) {
        self.workers.iter().for_each(|w| w.info());
    }

    pub fn stop(self) {
        for worker in self.workers {
            worker.stop()
        }
    }
}
