#![forbid(unsafe_code)]

use std::hash::Hash;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct LRUCache<K, V> {
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // TODO: your code goes here.
        unimplemented!()
    }

}
