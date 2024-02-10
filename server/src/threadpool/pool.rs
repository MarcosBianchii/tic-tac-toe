use super::worker::{Job, Worker};
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(n: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        Self {
            workers: (0..n).map(|id| Worker::new(id, Arc::clone(&rx))).collect(),
            sender: Some(tx),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        self.workers
            .iter_mut()
            .inspect(|worker| println!("Shutting down worker {}", worker.id))
            .map(|worker| worker.thread.take())
            .flatten()
            .for_each(|handle| handle.join().unwrap());
    }
}
