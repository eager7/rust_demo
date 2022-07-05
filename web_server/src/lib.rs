use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn exec<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            println!("start worker:{}", id);
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("job:{}", id);
                    job();
                }
                Message::Terminate => {
                    println!("quit:{}", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + 'static + Send>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("send terminate message");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("shutdown thread");
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                println!("shutdown worker:{}", worker.id);
                thread.join().unwrap();
            }
        }
    }
}
enum Message {
    NewJob(Job),
    Terminate,
}
