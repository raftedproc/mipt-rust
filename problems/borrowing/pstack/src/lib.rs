#![forbid(unsafe_code)]
use std::rc::Rc;

pub struct PRef<T> {
    value_ref: Option<Rc<T>>,
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // TODO: your code goes here.
        self.value_ref.as_ref().unwrap()
    }
}

impl<T> Default for PRef<T> {
    fn default() -> Self {
        Self {
            value_ref: { None },
        }
    }
}

impl<T> PRef<T> {
    pub fn new(value: T) -> Self {
        // TODO: your code goes here.
        Self {
            value_ref: Some(Rc::new(value)),
        }
    }
}

impl<T> Clone for PRef<T> {
    fn clone(&self) -> Self {
        // TODO: your code goes here.
        Self {
            value_ref: self.value_ref.clone(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct PStack<T> {
    // TODO: your code goes here.
    prev_stack: Option<Rc<PStack<T>>>,
    value: PRef<T>,
    length: usize,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        // TODO: your code goes here.
        Self {
            prev_stack: None,
            value: <_>::default(),
            length: 0,
        }
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        // TODO: your code goes here.
        Self {
            prev_stack: self.prev_stack.clone(),
            value: self.value.clone(),
            length: self.length,
        }
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        // TODO: your code goes here.
        Self::default()
    }

    pub fn push(&self, value: T) -> Self {
        // TODO: your code goes here.
        Self {
            prev_stack: Some(Rc::new(self.clone())),
            value: PRef::new(value),
            length: self.length.checked_add(1).expect("Some error"),
        }
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        // TODO: your code goes here.
        if self.is_empty() {
            None
        } else {
            Some((
                self.value.clone(),
                self.prev_stack.as_ref().unwrap().as_ref().clone(),
            ))
        }

        // Rc::try_unwrap(self.prev_stack.clone().unwrap())
        //     .map_or(None, |st| Some((self.value.clone(), st)))
    }

    pub fn len(&self) -> usize {
        // TODO: your code goes here.
        self.length
    }

    pub fn is_empty(&self) -> bool {
        // TODO: your code goes here.
        self.length == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = PRef<T>> {
        PStackIter {
            current: Rc::new(self.clone()),
        }
    }
}

pub struct PStackIter<T> {
    current: Rc<PStack<T>>,
}

impl<T> Iterator for PStackIter<T> {
    type Item = PRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_empty() {
            None
        } else {
            let value = self.current.value.clone();
            self.current = self.current.prev_stack.clone().unwrap();
            Some(value)
        }
    }
}
