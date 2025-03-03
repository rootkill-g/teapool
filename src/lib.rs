use std::{
    net::TcpStream,
    sync::mpsc,
    thread::{self, JoinHandle},
};

type Job = (fn(TcpStream), TcpStream);
type Sender = mpsc::Sender<Job>;
type Receiver = mpsc::Receiver<Job>;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Receiver) -> Self {
        let thread = Some(thread::spawn(|| Self::work(receiver)));

        Self { id, thread }
    }

    fn work(receiver: Receiver) {
        loop {
            let message = receiver.recv();

            match message {
                Ok((handler, stream)) => handler(stream),
                Err(_) => break,
            }
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    senders: Vec<Sender>,
    next_sender: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let mut senders = Vec::with_capacity(size);

        for id in 0..size {
            let (sender, receiver) = mpsc::channel();

            senders.push(sender);
            workers.push(Worker::new(id, receiver))
        }

        ThreadPool {
            workers,
            senders,
            next_sender: 0,
        }
    }

    pub fn execute(&mut self, handler: fn(TcpStream), stream: TcpStream) {
        let job = (handler, stream);

        self.senders[self.next_sender].send(job).unwrap();
        self.next_sender += 1;

        if self.next_sender == self.senders.len() {
            self.next_sender = 0;
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.senders.clear();

        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
