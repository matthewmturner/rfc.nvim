use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::error::{RFSeeError, RFSeeResult};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    _id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });
        Self {
            _id: id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F) -> RFSeeResult<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        match self.sender.as_ref() {
            Some(sender) => sender
                .send(job)
                .map_err(|e| RFSeeError::RuntimeError(e.to_string())),
            None => Err(RFSeeError::RuntimeError("No sender available".to_string())),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::ThreadPool;

    #[test]
    fn test_single_thread_completes_work() {
        let pool = ThreadPool::new(1);
        let completed = Arc::new(Mutex::new(false));

        let cloned = Arc::clone(&completed);
        let job = move || {
            let mut guard = cloned.lock().unwrap();
            *guard = true;
        };

        pool.execute(job).unwrap();
        drop(pool); // Let the job finish and bring ref count on `cloned` to 1
        assert!(Arc::into_inner(completed).unwrap().into_inner().unwrap());
    }

    #[test]
    fn test_multiple_threads_completes_work() {
        let pool = ThreadPool::new(4);
        let completed = Arc::new(Mutex::new(0));

        let cloned = Arc::clone(&completed);
        let job = move || {
            let mut guard = cloned.lock().unwrap();
            *guard += 1;
        };

        pool.execute(job.clone()).unwrap();
        pool.execute(job.clone()).unwrap();
        pool.execute(job.clone()).unwrap();
        pool.execute(job).unwrap();
        drop(pool); // Let the job finish and bring ref count on `cloned` to 1
        assert_eq!(Arc::into_inner(completed).unwrap().into_inner().unwrap(), 4);
    }
}
