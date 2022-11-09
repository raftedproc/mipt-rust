#![forbid(unsafe_code)]
use std::str::Chars;

pub trait ToKeyIter {
    type Item: Clone;
    type KeyIter<'a>
    where
        Self: 'a;

    fn key_iter<'a>(&'a self) -> Self::KeyIter<'a>;
}

impl ToKeyIter for str {
    type Item = char;
    type KeyIter<'a> = std::str::Chars<'a>;

    fn key_iter<'a>(&'a self) -> Self::KeyIter<'a> {
        return self.chars();
    }
}

impl ToKeyIter for String {
    type Item = char;
    type KeyIter<'a> = std::str::Chars<'a>;

    fn key_iter<'a>(&'a self) -> Self::KeyIter<'a> {
        return self.as_str().chars();
    }
}

////////////////////////////////////////////////////////////////////////////////

// Bonus

// pub trait FromKeyIter {
//     fn to_key(self) -> ???;
// }

// impl FromKeyIter for ???
// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

// Bonus

// pub trait TrieKey
// TODO: your code goes here.
