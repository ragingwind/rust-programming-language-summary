use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
  NewJob(Job),
  Terminate,
}

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

// Rust doesn’t allow us to move a value out of a Box<T> because 
// Rust doesn’t know how big the value inside the Box<T>
// Rust explicitly that in this case we can take ownership of 
// the value inside the Box<T> using self: Box<Self>
type Job = Box<FnBox + Send + 'static>;

struct Worker {
  id: usize,
  // if Worker holds an Option<thread::JoinHandle<()>> instead, 
  // we can call the take method on the Option to move the value 
  // out of the Some variant and leave a None variant in its place
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let message = receiver.lock().unwrap().recv().unwrap();

        match message {
          Message::NewJob(job) => {
            println!("Worker {} got a job; executing.", id);
            job.call_box();
          },
          Message::Terminate => {
            println!("Worker {} was told to terminate.", id);
            break;
          },
        }
      }
    });

    Worker {
      id,
      thread: Some(thread)
    }
  }
}

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
    
    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers,
      sender,
    }
  }

  pub fn execute<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    println!("Shutting down all worker.");

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}