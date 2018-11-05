use std::sync::*;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

use std::thread;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let msg = Message::NewJob(Box::new(f));
        self.sender.send(msg).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        self.workers
            .iter_mut()
            .flat_map(|worker| worker.thread.take())
            .for_each(|thread| thread.join().unwrap());
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = match rx.lock().unwrap().recv().unwrap() {
                Message::NewJob(job) => job,
                Message::Terminate =>{
                    println!("#{}: shutting down", id);
                    break;
                }
            };

            println!("#{}: got job", id);
            job.call_box();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
