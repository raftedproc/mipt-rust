#![forbid(unsafe_code)]
use crate::trie_key::ToKeyIter;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    hash::Hash,
    marker::PhantomData,
    ops::Index,
};

struct TrieNode<K, V>
where
    K: Hash + Eq,
    V: Clone + Default,
{
    pub is_terminal_: bool,
    pub jumps_: HashMap<K, TrieNode<K, V>>,
    value_: V,
}

impl<K, V> TrieNode<K, V>
where
    K: Hash + Eq,
    V: Clone + Default,
{
    pub fn new() -> Self {
        Self {
            is_terminal_: false,
            jumps_: HashMap::new(),
            value_: V::default(),
        }
    }

    pub fn set_value(&mut self, v: V) -> Option<V> {
        if self.is_terminal_ {
            // println!("set_value was terminal");
            let prev_val = self.value_.clone();
            self.value_ = v;
            // println!("set_value {}", self.is_terminal());
            Some(prev_val)
        } else {
            // println!("set_value was not terminal");
            self.is_terminal_ = true;
            self.value_ = v;
            // println!("set_value {}", self.is_terminal_);
            None
        }
    }

    pub fn get_next_jump<T: Hash + Eq + ?Sized>(&self, j: &T) -> Option<&TrieNode<K, V>>
    where
        K: Borrow<T>,
    {
        self.jumps_.get(j)
    }

    pub fn is_terminal(&self) -> bool {
        self.is_terminal_
    }
    pub fn get_value(&self) -> Option<&V> {
        if self.is_terminal() {
            // I presume there is a value if the node is terminal
            Some(&self.value_)
        } else {
            None
        }
    }
    pub fn get_mut_value(&mut self) -> Option<&mut V> {
        if self.is_terminal() {
            // I presume there is a value if the node is terminal
            Some(&mut self.value_)
        } else {
            None
        }
    }
    pub fn is_empty(&self) -> bool {
        self.jumps_.is_empty()
    }
    pub fn recursive_len(&self) -> usize {
        let mut len = if self.is_terminal() { 1 } else { 0 };
        for node in self.jumps_.values() {
            if !node.is_empty() {
                len += node.recursive_len();
            } else {
                len += if node.is_terminal() { 1 } else { 0 };
            }
        }
        len
    }
    pub fn has_at_least_one_terminal(&self) -> bool {
        if self.is_terminal_ {
            return true;
        } else {
        }
        let mut node = self;
        for key in self.jumps_.keys() {
            node = self.jumps_.get(key).unwrap();
            if !node.is_empty() {
                if node.has_at_least_one_terminal() {
                    return true;
                }
            } else {
                if node.is_terminal_ {
                    return true;
                }
            }
        }

        false
    }
}
// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

pub struct Trie<'a, K, V>
where
    K: ToKeyIter,
    V: Clone + Default,
    K::Item: Hash + Eq,
    Self: 'a,
    <K as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
    // TODO: your code goes here.
{
    node_: TrieNode<K::Item, V>,
    phantom: PhantomData<&'a V>,
}

impl<'a, K, V> Trie<'a, K, V>
where
    K: ToKeyIter,
    V: Clone + Default,
    K::Item: Hash + Eq,
    Self: 'a,
    <K as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
    // TODO: your code goes here.
{
    pub fn new() -> Self {
        // TODO: your code goes here.
        Self {
            node_: TrieNode::new(),
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        // TODO: your code goes here.
        // DFS search for all terminal
        if self.node_.is_empty() {
            return 0;
        }
        self.node_.recursive_len()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: your code goes here.
        self.node_.is_empty()
    }

    pub fn insert<'b, Q: ToKeyIter + ?Sized>(&mut self, key: &'b Q, value: V) -> Option<V>
    where
        K: Borrow<Q>,
        K::Item: Hash + Eq,
        <Q as ToKeyIter>::KeyIter<'b>: Iterator<Item = <K as ToKeyIter>::Item>,
    {
        let mut node = &mut self.node_;
        for c in key.key_iter() {
            // println!("iter in insert");
            node = node.jumps_.entry(c.clone()).or_insert(TrieNode::new());
        }
        node.set_value(value)
    }

    pub fn get<Q: ToKeyIter + ?Sized>(&self, key: &'a Q) -> Option<&V>
    where
        K: Borrow<Q>,
        K::Item: Hash + Eq,
        <Q as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
    {
        let mut node = &self.node_;
        for c in key.key_iter() {
            node = node.get_next_jump(&c)?
        }
        node.get_value()
    }

    pub fn get_mut<'b, Q: ToKeyIter + ?Sized>(&mut self, key: &'b Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        K::Item: Hash + Eq,
        <Q as ToKeyIter>::KeyIter<'b>: Iterator<Item = <K as ToKeyIter>::Item>,
    {
        let mut node = &mut self.node_;
        for c in key.key_iter() {
            node = node.jumps_.get_mut(&c)?
        }
        node.get_mut_value()
    }
    pub fn prefix_is_valid<Q: ToKeyIter + ?Sized>(&self, key: &'a Q, check_for_term: bool) -> bool
    where
        K: Borrow<Q>,
        K::Item: Hash + Eq,
        <Q as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
    {
        let mut node = &self.node_;
        for c in key.key_iter() {
            let node_opt = node.jumps_.get(&c);
            if node_opt.is_none() {
                return false;
            }
            node = node_opt.unwrap();
        }
        if check_for_term {
            node.is_terminal()
        } else {
            node.has_at_least_one_terminal()
        }
    }

    pub fn contains<Q: ToKeyIter + ?Sized>(&self, key: &'a Q) -> bool
    where
        K: Borrow<Q>,
        K::Item: Hash + Eq,
        <Q as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
    {
        self.prefix_is_valid(key, true)
    }

    pub fn starts_with<Q: ToKeyIter + ?Sized>(&self, key: &'a Q) -> bool
    where
        K: Borrow<Q>,
        K::Item: Hash + Eq,
        <Q as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
    {
        self.prefix_is_valid(key, false)
    }

    pub fn remove<'b, Q: ToKeyIter + ?Sized>(&mut self, key: &'b Q) -> Option<V>
    where
        K: Borrow<Q>,
        K::Item: Hash + Eq,
        <Q as ToKeyIter>::KeyIter<'b>: Iterator<Item = <K as ToKeyIter>::Item>,
    {
        let mut node = &mut self.node_;
        for c in key.key_iter() {
            node = node.jumps_.get_mut(&c)?
        }
        if node.is_terminal() {
            node.is_terminal_ = false;
            return Some(node.value_.clone());
        }
        None
    }
}

////////////////////////////////////////////////////////////////////////////////

// impl<'a, K, V> Index<K> for Trie<'a, K, V>
// where
//     K: ToKeyIter,
//     V: Clone + Default,
//     K::Item: Hash + Eq,
//     <K as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
// {
//     type Output = V;
//     fn index(&self, key: K) -> &Self::Output
//     where
//         K::Item: Hash + Eq,
//         <K as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
//     {
//         let key_ref = &key;
//         let option_v = self.get(key_ref);
//         match option_v {
//             Some(so) => &option_v.unwrap(),
//             None => panic!("some"),
//         }
//     }
// }
// TODO: your code goes here.
// impl<'a, V> Index<String> for Trie<'a, String, V>
// where
//     V: Clone + Default,
//     // K::Item: Hash + Eq,
//     // <K as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
// {
//     type Output = V;
//     fn index(&self, key: String) -> &Self::Output
// where
//         // K::Item: Hash + Eq,
//         // <K as ToKeyIter>::KeyIter<'a>: Iterator<Item = <K as ToKeyIter>::Item>,
//     {
//         // let key_ref = &key;
//         // let option_v = self.get(key_ref);
//         // match option_v {
//         //     Some(so) => &option_v.unwrap(),
//         //     None => panic!("some"),
//         // }
//         if let Some(a) = self.get(&key) {
//             a
//         } else {
//             panic!("some")
//         }
//     }
// }
