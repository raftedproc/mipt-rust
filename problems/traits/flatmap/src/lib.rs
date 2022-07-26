#![forbid(unsafe_code)]

use std::{borrow::Borrow, iter::FromIterator, ops::Index};

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FlatMap<K, V> {
    data_: Vec<(K, V)>,
}

impl<K: Ord, V> FlatMap<K, V> {
    pub fn new() -> Self {
        Self { data_: Vec::new() }
    }

    pub fn len(&self) -> usize {
        // TODO: your code goes here.
        self.data_.len()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: your code goes here.
        self.data_.is_empty()
    }

    pub fn capacity(&self) -> usize {
        // TODO: your code goes here.
        self.data_.capacity()
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        // TODO: your code goes here.
        &self.data_
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // TODO: your code goes here.
        let idx = self.data_.iter().enumerate().find(|&t| key == t.1 .0);
        let result = match idx {
            Some(x) => Some(self.data_.swap_remove(x.0).1),
            _ => None,
        };
        self.data_.push((key, value));
        // Will eat up all the perf
        self.data_.sort_by(|x, y| x.0.cmp(&y.0));
        result
    }
    // This version passes all functionality tests but it is utterly slow.
    // pub fn get<Q>(&self, key: &Q) -> Option<&V>
    // where
    //     K: Borrow<Q>,
    //     Q: Ord + ?Sized,
    // {
    //     self.data_
    //         .iter()
    //         .find(|&t| key == t.0.borrow())
    //         .map(|t| &t.1)
    // }
    // This version is comparable with tree and hashmap speed-wise doing lookups but
    // binary_search_by() delivers deterministic but nevertheless any of the equal keys so
    // this version doesn't pass the tests.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let idx = self.data_.binary_search_by(|t| t.0.borrow().cmp(key));
        match idx {
            Ok(x) => self.data_.get(x).map(|t| &t.1),
            _ => None,
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let idx = self.data_.iter().position(|t| t.0.borrow() == key);
        match idx {
            Some(x) => Some(self.data_.swap_remove(x).1),
            None => None,
        }
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let idx = self.data_.iter().position(|t| t.0.borrow() == key);
        match idx {
            Some(x) => Some(self.data_.swap_remove(x)),
            None => None,
        }
    }
}

impl<K: Ord, Q: ?Sized, V> Index<&Q> for FlatMap<K, V>
where
    K: Borrow<Q>,
    Q: Ord,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        // how to panic here
        let idx = self.data_.iter().find(|&t| index == t.0.borrow());
        match idx {
            Some(x) => &x.1,
            None => panic!("FlatMap<K,V>::index(): Can not find a key"),
        }
    }
}

impl<K: Ord, V> Extend<(K, V)> for FlatMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for i in iter {
            self.insert(i.0, i.1);
        }
    }
}

impl<K: Ord, V> From<Vec<(K, V)>> for FlatMap<K, V> {
    fn from(v: Vec<(K, V)>) -> Self {
        let mut res = Self::new();
        for (k, v) in v {
            res.insert(k, v);
        }
        res
    }
}

impl<K: Ord, V> From<FlatMap<K, V>> for Vec<(K, V)> {
    fn from(map: FlatMap<K, V>) -> Self {
        map.data_
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut res = FlatMap::new();
        for (k, v) in iter {
            res.insert(k, v);
        }
        res
    }
}
// impl IntoIterator for FlatMap { ... }
impl<K: Ord, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.data_.into_iter()
    }
}
