use std::{
    sync::{
        mpsc::{Receiver, SyncSender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<SyncSender<Job>>,
}

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size if zero.
    pub fn new(size: usize) -> ThreadPool {
        if size == 0 {
            panic!("you want HOW MANY THREADS??");
        }

        let (sender, receiver) = std::sync::mpsc::sync_channel(64);

        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&mut self, action: F)
    where
        F: FnOnce() + Send + 'static,
    {
        match &self.sender {
            Some(sender) => sender.send(Box::new(action)).unwrap(),
            None => panic!("This ThreadPool is closed!!"),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down workers...");

        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Waiting for worker {}", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        // Note: thread::spawn will panic if the OS doesn't have enough resources to create our
        // thread. To avoid this, prefer using `std::thread::Builder` instead.
        let handle = thread::spawn(move || {
            println!("Worker {id} lives!");

            // This one works as intended
            loop {
                let maybe_job = receiver.lock().unwrap().recv();
                // Note: With `let` statements, any temporary values used on the right hand side are
                // dropped immediately after the `let` statement ends. This means the mutex is unlocked
                // right after this line and not held. This doesn't happen with the `if let`, or the
                // `while let` statements, which would hold the lock until the end of the block!

                match maybe_job {
                    Ok(job) => job(),
                    Err(_) => break,
                }
            }

            println!("Worker {id} signing off!");
        });

        Worker {
            id,
            handle: Some(handle),
        }
    }
}
