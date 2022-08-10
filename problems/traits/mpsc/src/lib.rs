#![forbid(unsafe_code)]

use std::{
    cell::{Cell, RefCell},
    collections::{vec_deque, VecDeque},
    error::Error,
    fmt::{Debug, Display},
    ops::Deref,
    rc::Rc,
};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

type SingleThrVecShRef<T> = Rc<RefCell<VecDeque<T>>>;
type SingleThrBooShRef = Rc<RefCell<bool>>;

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T> {
    pub value: T,
}

pub struct Sender<T> {
    // TODO: your code goes here.
    queue_: SingleThrVecShRef<T>,
    channel_is_closed_: SingleThrBooShRef,
}

impl<T> Sender<T> {
    pub fn new(queue: SingleThrVecShRef<T>, is_closed: SingleThrBooShRef) -> Self {
        Self {
            queue_: queue,
            channel_is_closed_: is_closed.clone(),
        }
    }
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        // TODO: your code goes here.
        if *self.channel_is_closed_.borrow() == true {
            return Err(SendError { value: value });
        }
        self.queue_.borrow_mut().push_back(value);
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        // TODO: your code goes here.
        *self.channel_is_closed_.borrow() == true
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        // TODO: your code goes here.
        Rc::ptr_eq(&self.channel_is_closed_, &other.channel_is_closed_)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        // TODO: your code goes here.
        Self {
            queue_: self.queue_.clone(),
            channel_is_closed_: self.channel_is_closed_.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // TODO: your code goes here.
        // Rc::strong_count == 2 b/c of the original Rc created
        // in the channel ctor f()
        if Rc::strong_count(&self.channel_is_closed_) <= 2 {
            *self.channel_is_closed_.borrow_mut() = true;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    // TODO: your code goes here.
    queue_: SingleThrVecShRef<T>,
    channel_is_closed_: SingleThrBooShRef,
}

impl<T> Receiver<T> {
    pub fn new(queue: SingleThrVecShRef<T>, is_closed: SingleThrBooShRef) -> Self {
        Self {
            queue_: queue,
            channel_is_closed_: is_closed.clone(),
        }
    }
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        // TODO: your code goes here.
        if !self.queue_.borrow().is_empty() {
            return Ok(self.queue_.borrow_mut().pop_front().unwrap());
        }
        if *self.channel_is_closed_.borrow() == true {
            return Err(ReceiveError::Closed);
        }
        if self.queue_.borrow().is_empty() == true {
            return Err(ReceiveError::Empty);
        }
        return Err(ReceiveError::Empty);
    }

    pub fn close(&mut self) {
        // TODO: your code goes here.
        *self.channel_is_closed_.borrow_mut() = true;
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // TODO: your code goes here.
        *self.channel_is_closed_.borrow_mut() = true;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    // TODO: your code goes here.
    let msg_queue: VecDeque<T> = VecDeque::new();
    let shared_queue: SingleThrVecShRef<T> = Rc::new(RefCell::new(msg_queue));
    let shared_queue_state: SingleThrBooShRef = Rc::new(RefCell::new(false));
    let s = Sender::new(shared_queue.clone(), shared_queue_state.clone());
    let r = Receiver::new(shared_queue.clone(), shared_queue_state.clone());
    (s, r)
}
