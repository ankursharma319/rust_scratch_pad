use std::sync::mpsc::Receiver;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker
    {
        let handle = thread::spawn(move || {
            loop {
                println!("inside worker #{}, waiting for lock and message on receiver!", id);
                // NOTE: dividing the receiver locking and msg receiving up into two different
                // parts is counter productive because it makes it that the lock is held until the
                // end of the scope (i.e. after job()) rather than immediately unlocking after the
                // let statement
                // let receiver_raw: &Receiver<Job> = &*receiver.lock().expect("failed to acquire lock");
                // let job = receiver_raw.recv().expect("didnt receive message successfully in thread via the channel");
                let job = receiver.lock().expect("failed to acquire lock").recv().expect("didnt receive message successfully in thread via the channel");
                println!("executing job on worker#{}", id);
                job();
            }
        });
        Worker {id, handle}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    // add code here
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver_protected = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for i in 0..workers.capacity() {
            workers.push(Worker::new(i, Arc::clone(&receiver_protected)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce(),
            F: Send + 'static,
    {
        println!("executing function in thread pool");
        let func = Box::new(f);
        self.sender.send(func).expect("Couldnt send job successfully to a worker thread");
    }
}
