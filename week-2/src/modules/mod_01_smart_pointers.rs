use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

// list_sum
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn list_sum(list: &List) -> i32 {
    match list {
        List::Cons(value, next) => value + list_sum(next),
        List::Nil => 0,
    }
}

// Implement Deref for Wrapper<T>
struct Wrapper<T>(T);

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn double_len(s: &str) -> usize {
    s.len() * 2
}

// Rc - refeence counting
pub fn count_owners(n: usize) -> usize {
    let owner = Rc::new(2);
    let mut clones = Vec::new();
    for _ in 0..n {
        clones.push(owner.clone());
    }
    Rc::strong_count(&owner)
}

// counter RefCell
struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }

    fn increment(&self) {
        *self.value.borrow_mut() += 1
    }

    fn get(&self) -> i32 {
        *self.value.borrow()
    }
}

pub fn count_to(n: i32) -> i32 {
    let counter = Counter::new();

    for _ in 0..n {
        counter.increment();
    }
    counter.get()
}
