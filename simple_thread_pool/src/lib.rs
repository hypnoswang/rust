use std::marker::Send;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

pub trait Job {
    fn run(&mut self);
}

enum Cmd<T: Job + Send> {
    Exec(T),
    Stop,
}

struct Worker<T: Job + Send> {
    handle: Option<JoinHandle<()>>,
    sender: Sender<Cmd<T>>,
}

impl<T> Worker<T>
where
    T: Job + Send + 'static,
{
    fn new(idx: usize) -> Worker<T> {
        let (tx, rx) = mpsc::channel();
        let h = thread::spawn(move || loop {
            let c: Cmd<T> = rx.recv().unwrap_or_else(|e| {
                eprintln!("Got errors when waiting for jobs: {}", e.to_string());
                Cmd::Stop
            });

            match c {
                Cmd::Stop => {
                    eprintln!("Received stop cmd, exiting....");
                    break;
                }
                Cmd::Exec(mut job) => {
                    println!("thread-{} begin to work", idx);
                    job.run();
                    println!("thread-{} complete work", idx);
                }
            }
        });

        Worker {
            handle: Some(h),
            sender: tx,
        }
    }

    fn dispatch(&self, job: T) -> Result<(), &'static str> {
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

pub struct Pool<T: Job + Send> {
    last_worker: usize,
    workers: Vec<Worker<T>>,
}

impl<T> Pool<T>
where
    T: Job + Send + 'static,
{
    pub fn new(max: usize) -> Pool<T> {
        let mut pool = Pool {
            last_worker: 0,
            workers: vec![],
        };

        for i in 0..max {
            pool.workers.push(Worker::new(i));
        }

        pool
    }

    pub fn dispatch(&mut self, job: T) -> Result<(), &'static str> {
        self.last_worker = (self.last_worker + 1) % self.workers.len();
        let worker = self.workers.get(self.last_worker).unwrap();
        worker.dispatch(job)
    }

    pub fn stop(self) {
        for worker in self.workers {
            worker.stop()
        }
    }
}
