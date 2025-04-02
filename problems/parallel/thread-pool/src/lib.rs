#![forbid(unsafe_code)]

use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::deque::{Injector, Steal, Stealer, Worker};

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    thread,
};

////////////////////////////////////////////////////////////////////////////////

pub struct ThreadPool<T> {
    common_queue: Arc<Mutex<Injector<T>>>,
    // workers_queues: Arc<Vec<Arc<Worker<T>>>>,
    join_handles: Vec<thread::JoinHandle<()>>,
}

impl<T: Send + Sync + 'static> ThreadPool<T> {
    pub fn new(thread_count: usize) -> Self {
        let common_queue = Arc::new(Mutex::new(Injector::new()));
        let workers_queues = Arc::new(vec![Worker::<T>::new_fifo()]);

        let common_queue_clone = Arc::clone(&common_queue);
        let workers_queues_clone = Arc::clone(&workers_queues);

        let t1 = thread::spawn(move || loop {
            match workers_queues[0].pop() {
                Some(_v) => println!("Worker 1 got value "),
                None => {
                    let common_queue = common_queue_clone.lock().unwrap();
                    match common_queue.steal() {
                        Steal::Empty => break,
                        Steal::Success(_v) => println!("Worker 1 got value"),
                        Steal::Retry => continue,
                    }
                }
            }
        });

        // let common_queue = Arc::new(Mutex::new(Injector::new()));
        // let workers_queues = vec![Arc::new(Worker::new_fifo())];

        // let common_queue_clone = common_queue.clone();

        // let t1 = thread::spawn(move || loop {
        //     match workers_queues[0].pop() {
        //         Some(_v) => println!("Worker 1 got value "),
        //         None => {
        //             let common_queue = common_queue_clone.lock().unwrap();
        //             match common_queue.steal() {
        //                 Steal::Empty => break,
        //                 Steal::Success(_v) => println!("Worker 1 got value"),
        //                 Steal::Retry => continue,
        //             }
        //         }
        //     }
        // });

        // let join_handles = workers_queues
        //     .iter()
        //     .zip((0..thread_count).into_iter())
        //     .map(|(w, id)| {
        //         thread::spawn(move || loop {
        //             match w.pop() {
        //                 Some(_) => println!("Worker {} got value", id),
        //                 None => match common_queue.steal() {
        //                     Steal::Empty => break,
        //                     Steal::Success(_) => println!("Worker {} got value ", id),
        //                     Steal::Retry => continue,
        //                 },
        //             }
        //         })
        //     })
        //     .collect::<Vec<_>>();
        Self {
            common_queue,
            // workers_queues,
            join_handles: vec![t1],
        }
    }

    // pub fn spawn(&self, task: ...) -> JoinHandle<...> {}

    pub fn shutdown(self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

// pub struct JoinHandle<T> {
//     // TODO: your code goes here.
// }

// #[derive(Debug)]
// pub struct JoinError {}

// impl<T> JoinHandle<T> {
//     pub fn join(self) -> Result<T, JoinError> {
//         // TODO: your code goes here.
//         unimplemented!()
//     }
// }
