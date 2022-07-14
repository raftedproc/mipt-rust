#![forbid(unsafe_code)]

use std::cmp::min;
use std::fmt::Debug;

#[derive(Default)]
pub struct MinQueue<T> {
    // TODO: your code goes here.
    data1: Vec<(T, T)>,
    data2: Vec<(T, T)>,
}

impl<T: Clone + Ord + Debug> MinQueue<T> {
    pub fn new() -> Self {
        // TODO: your code goes here.
        Self {
            data1: Vec::new(),
            data2: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        // TODO: your code goes here.
        let min_val: T = if self.data1.is_empty() {
            val.clone()
        } else {
            let data1_min = self.data1.last().unwrap().1.clone();
            min(val.clone(), data1_min)
        };
        let value: T = val.clone();
        self.data1.push((value, min_val));
    }

    pub fn pop(&mut self) -> Option<T> {
        // TODO: your code goes here.
        if self.data2.is_empty() {
            for (i, t) in self.data1.iter().rev().enumerate() {
                let val = (
                    t.0.clone(),
                    if i == 0 {
                        t.0.clone()
                    } else {
                        min(
                            t.0.clone(),
                            self.data2.last().map(|t1| t1.1.clone()).unwrap(),
                        )
                    },
                );
                self.data2.push(val);
            }
            // Don't know how to access the prev iter being in the next to correctly populate min value
            // self.data2 = self
            //     .data1
            //     .iter()
            //     .rev()
            //     .enumerate()
            //     .map(|t| {
            //         (
            //             t.1 .0.clone(),
            //             if t.0 == 0 {
            //                 t.1 .0.clone()
            //             } else {
            //                 min(
            //                     t.1 .0.clone(),
            //                     self.data2.last().map(|t1| t1.1.clone()).unwrap(),
            //                 )
            //                 // min(
            //                 //     t.0.clone(),
            //                 //     // self.data2.last().map(|t1| t1.1.clone()).unwrap(),
            //                 // )
            //             },
            //         )
            //     })
            //     .collect::<Vec<(T, T)>>();
            self.data1 = vec![]
        }
        self.data2.pop().map(|t| t.0)
    }

    pub fn front(&self) -> Option<&T> {
        // TODO: your code goes here.
        if self.data2.is_empty() {
            self.data1.first().map(|t| &t.0)
        } else {
            self.data2.last().map(|t| &t.0)
        }
    }

    pub fn min(&self) -> Option<&T> {
        // TODO: your code goes here.
        match (self.data1.is_empty(), self.data2.is_empty()) {
            (false, false) => min(
                self.data2.last().map(|t| &t.1),
                self.data1.last().map(|t| &t.1),
            ),
            (true, false) => self.data2.last().map(|t| &t.1),
            (false, true) => self.data1.last().map(|t| &t.1),
            (true, true) => None,
        }
    }

    pub fn len(&self) -> usize {
        // TODO: your code goes here.
        self.data1.len() + self.data2.len()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: your code goes here.
        return self.data1.is_empty() && self.data2.is_empty();
    }
}
