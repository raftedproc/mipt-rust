#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    a_: Vec<V>,
    hm_: HashMap<K, (usize, usize)>,
    epoch_set_: BTreeMap<usize, K>,
    epoch_: usize,
}

impl<K: Clone + Hash + Ord, V /* + std::fmt::Debug */> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        // TODO: your code goes here.
        println!("new");
        if capacity == 0 {
            panic!("Can not create LRUCache w capacity = 0.");
        }
        Self {
            a_: Vec::with_capacity(capacity + 1),
            hm_: HashMap::with_capacity(capacity),
            epoch_set_: BTreeMap::new(),
            epoch_: 1,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        // TODO: your code goes here.
        println!("get ");
        let idx_epoch_tupl = self.hm_.get(key).map(|t| t.clone());
        if idx_epoch_tupl == None {
            return None;
        }
        let (idx, epoch) = idx_epoch_tupl.unwrap();
        self.epoch_set_.remove(&epoch);
        self.epoch_set_.insert(self.epoch_, key.clone());
        self.hm_.remove(&key.clone());
        self.hm_.insert(key.clone(), (idx, self.epoch_));

        self.epoch_ += 1;
        self.a_.get(idx)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // TODO: your code goes here.
        let arr_len = self.a_.len();
        if arr_len == self.a_.capacity() - 1 {
            let idx_epoch = self.hm_.get(&key);
            if let None = idx_epoch {
                // No key in the hashmap so replacing the oldest LRU element
                // get LRU key to replace
                let (_epoch, key_2_replace) = self
                    .epoch_set_
                    .iter()
                    .next()
                    .map(|t| (t.0.clone(), t.1.clone()))
                    .unwrap();
                // Invariant. There must be a key so unwrap and panic if needed.
                let (idx, epoch_to_remove) =
                    self.hm_.get(&key_2_replace).map(|t| t.clone()).unwrap();
                self.epoch_set_.remove(&epoch_to_remove);
                self.epoch_set_.insert(self.epoch_, key.clone());
                self.a_[idx] = value;
                self.hm_.insert(key, (idx, self.epoch_));
                self.epoch_ += 1;
                self.hm_.remove(&key_2_replace);
                return None;
            } else {
                // found the entry so return Some(x)
                let (idx, epoch) = (idx_epoch.map(|t| (t.0.clone(), t.1.clone()))).unwrap();
                self.a_.push(value);
                self.epoch_set_.remove(&epoch);
                self.epoch_set_.insert(self.epoch_, key.clone());
                self.hm_.remove(&key);
                self.hm_.insert(key, (idx, self.epoch_));
                self.epoch_ += 1;
                Some(self.a_.swap_remove(idx))
            }
        } else {
            // The cache has some space to insert a value
            let idx_epoch = self.hm_.get(&key);
            if let None = idx_epoch {
                // The key is not in the cache
                self.a_.push(value);
                self.epoch_set_.insert(self.epoch_, key.clone());
                self.hm_.insert(key, (arr_len, self.epoch_));
                self.epoch_ += 1;
                return None; // inserted into the cache w/o eviction so return None
            }
            // The key is in the cache
            let (idx, epoch) = (idx_epoch.map(|t| (t.0.clone(), t.1.clone()))).unwrap();
            self.a_.push(value);
            self.epoch_set_.remove(&epoch);
            self.epoch_set_.insert(self.epoch_, key.clone());
            self.hm_.remove(&key);
            self.hm_.insert(key, (idx, self.epoch_));
            self.epoch_ += 1;
            Some(self.a_.swap_remove(idx))
        }
    }
}
