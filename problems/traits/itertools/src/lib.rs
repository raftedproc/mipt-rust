#![forbid(unsafe_code)]
// use std::iter::Cycle;
// #![feature(once_cell)]
// use std::lazy::Lazy;
use std::{cell::RefCell, collections::VecDeque, fmt::Debug, iter::Fuse, rc::Rc};

// pub enum IterSource {
//     Itr,
//     St,
//     St2,
// }

// impl PartialEq for IterSource {
//     fn eq(&self, y: &IterSource) -> bool {
//         match self {
//             IterSource::Itr => *y == IterSource::Itr,
//             IterSource::St => *y == IterSource::St,
//             IterSource::St2 => *y == IterSource::St2,
//         }
//     }
// }
// I don't like this state machine transitions using bools. Matches + enum should be more elegant
// but the amount of code will be the same.
pub struct LazyCycle<I: Iterator> {
    // TODO: your code goes here.
    iter_: I,
    store1_: VecDeque<I::Item>,
    store2_: VecDeque<I::Item>,
    iter_exchausted_: bool,
    iter_in_store1_: bool,
}

impl<I: Iterator> LazyCycle<I>
where
    I::Item: Clone,
{
    fn new(iter: I) -> Self {
        println!("LazyCycle ctor");
        Self {
            iter_: iter,
            store1_: VecDeque::new(),
            store2_: VecDeque::new(),
            iter_exchausted_: false,
            iter_in_store1_: false,
        }
    }
}

impl<I: Iterator> Iterator for LazyCycle<I>
where
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.iter_exchausted_ {
            let res = self.iter_.next();
            match res {
                Some(x) => {
                    self.store1_.push_back(x.clone());
                    return Some(x);
                }
                None => {
                    if self.store2_.is_empty() && self.store1_.is_empty() {
                        return None;
                    }
                    self.iter_exchausted_ = true;
                    self.iter_in_store1_ = true;
                }
            }
        }
        let res = if self.iter_in_store1_ {
            self.store1_.pop_front()
        } else {
            None
        };
        match res {
            Some(x) => {
                println!("some from store1_");
                self.store2_.push_back(x.clone());
                Some(x)
            }
            None => {
                println!("None from store1_");
                self.iter_in_store1_ = false;
                let res = self.store2_.pop_front(); // doesn't take None into account\
                match res {
                    Some(x) => {
                        println!("Some from store2_");
                        self.store1_.push_back(x.clone());
                        Some(x)
                    }
                    None => {
                        self.iter_in_store1_ = true;
                        self.next()
                    }
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    // TODO: your code goes here.
    iter_: I,
    skip_: VecDeque<I::Item>,
}

impl<I: Iterator> Extract<I> {
    fn new(iter: I, skip: VecDeque<I::Item>) -> Self {
        Self {
            iter_: iter,
            skip_: skip,
        }
    }
}

impl<I: Iterator> Iterator for Extract<I>
where
    I: Sized,
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.skip_.is_empty() {
            return self.skip_.pop_front();
        }
        self.iter_.next()
    }
}
// ////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct TeeBuffer<A, I> {
    backlog: VecDeque<A>,
    iter: I,
    /// The owner field indicates which id should read from the backlog
    owner: bool,
}
pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    // TODO: your code goes here.
    rcbuffer: Rc<RefCell<TeeBuffer<I::Item, I>>>,
    id: bool,
}
impl<I: Iterator> Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(iter: I) -> (Fuse<Tee<I>>, Fuse<Tee<I>>)
    where
        I: Iterator,
        I::Item: Clone,
    {
        let buffer = TeeBuffer {
            backlog: VecDeque::new(),
            iter,
            owner: false,
        };
        let t1 = Tee {
            rcbuffer: Rc::new(RefCell::new(buffer)),
            id: true,
        };
        let t2 = Tee {
            rcbuffer: t1.rcbuffer.clone(),
            id: false,
        };
        (t1.fuse(), t2.fuse())
    }
}

impl<I> Iterator for Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        // .borrow_mut may fail here -- but only if the user has tied some kind of weird
        // knot where the iterator refers back to itself.
        let mut buffer = self.rcbuffer.borrow_mut();
        if buffer.owner == self.id {
            match buffer.backlog.pop_front() {
                None => {}
                some_elt => return some_elt,
            }
        }
        match buffer.iter.next() {
            None => None,
            Some(elt) => {
                buffer.backlog.push_back(elt.clone());
                buffer.owner = !self.id;
                Some(elt)
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////////////

pub struct GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    // TODO: your code goes here.
    iter_: I,
    predicate_: F,
    data_: Vec<I::Item>,
}

impl<I, F, V> GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    fn new(iter: I, f: F) -> GroupBy<I, F, V>
    where
        I: Iterator,
        F: FnMut(&I::Item) -> V,
        V: Eq,
    {
        Self {
            iter_: iter,
            predicate_: f,
            data_: vec![],
        }
    }
}

impl<I, F, V> Iterator for GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
    I::Item: Debug,
{
    type Item = (V, Vec<I::Item>);
    fn next(&mut self) -> Option<Self::Item> {
        let mut data: Vec<I::Item> = vec![];
        let val_opt = self.data_.pop();
        match val_opt {
            Some(x) => data.push(x),
            None => {
                let next_iter_val = self.iter_.next();
                match next_iter_val {
                    Some(y) => data.push(y),
                    None => return None,
                }
            }
        }
        while let Some(x) = self.iter_.next() {
            if (self.predicate_)(&x) == (self.predicate_)(&data.last().unwrap()) {
                data.push(x);
            } else {
                self.data_.push(x);
                break;
            }
        }
        Some(((self.predicate_)(&data.last().unwrap()), data))
    }
}
////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        // TODO: your code goes here.
        LazyCycle::new(self)
    }

    // fn next(&mut self) -> Option<Self::Item>
    // where
    //     Self: Iterator,
    //     Self: Sized,
    //     Self::Item: Clone;

    fn extract(mut self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
        Self: Iterator,
        // Self::Item: Clone,
    {
        // TODO: your code goes here.
        let mut skip: VecDeque<<Self as Iterator>::Item> = VecDeque::new();
        let mut counter = 0;
        while counter <= index {
            match self.next() {
                Some(x) => skip.push_back(x),
                None => return (None, Extract::new(self, skip)),
            }
            // skip.push_back(self.next().unwrap());
            counter += 1;
        }
        // Can't use this b/c enumerate exhause self and I am not allowed
        // to exhaust it.
        // let mut skip = self
        //     .enumerate()
        //     .take_while(|t| t.0 < index)
        //     .map(|v| v.1)
        //     .collect::<VecDeque<_>>();
        (skip.pop_back(), Extract::new(self, skip))
    }

    fn tee(self) -> (Fuse<Tee<Self>>, Fuse<Tee<Self>>)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        // TODO: your code goes here.
        let (it1, it2) = Tee::new(self);
        (it1, it2)
    }

    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq,
    {
        // TODO: your code goes here.
        GroupBy::new(self, func)
    }
}

impl<I: Iterator> ExtendedIterator for I {}
