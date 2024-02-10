use std::sync::{mpsc::Receiver, Arc, Mutex};
use std::thread::{self, JoinHandle};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    pub thread: Option<JoinHandle<()>>,
    pub id: usize,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let Ok(job) = receiver.lock().unwrap().recv() else {
                println!("Worker {id} shutting down");
                break;
            };

            println!("Worker {id} running job");
            job();
        });

        println!("Worker {id} spawned");

        Self {
            thread: Some(thread),
            id,
        }
    }
}
