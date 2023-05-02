use std::sync::mpsc::Receiver;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
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
                let job = receiver.lock().expect("failed to acquire lock").recv();
                match job {
                    Ok(j) => {
                        println!("executing job on worker#{}", id);
                        j();
                    },
                    Err(_) => {
                        println!("worker#{} disconnected, shutting down thread", id);
                        return;
                    }
                }
            }
        });
        Worker {id, handle: Option::Some(handle)}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
        ThreadPool { workers, sender: Option::Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce(),
            F: Send + 'static,
    {
        println!("executing function in thread pool");
        let func = Box::new(f);

        if let Some(sender) = &self.sender {
            sender.send(func).expect("Couldnt send job successfully to a worker thread");
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // dropping the sender channel will make all future .recv calls inside threads
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}
