#[macro_use]
extern crate std;

use std::collections::HashMap;
use std::cell::RefCell;

macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

struct MyRc<T> {
    data: *mut T,
    count: *mut usize,
}

impl<T> MyRc<T> {
    fn new(data: T) -> Self {
        let b = Box::new(data);
        let count = Box::into_raw(Box::new(1));
        Self {
            data: Box::into_raw(b),
            count,
        }
    }
    fn clone(&self) -> Self {
        unsafe {
            *self.count += 1;
        }
        Self {
            data: self.data,
            count: self.count,
        }
    }
    fn strong_count(&self) -> usize {
        unsafe {
            *self.count
        }
    }
    fn weak_count(&self) -> usize { 0 }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.count -= 1;
            if *self.count == 0 {
                drop(Box::from_raw(self.data));
                drop(Box::from_raw(self.count));
            }
        }
    }
}

#[derive(Debug)]
struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    fn new() -> SimpleStack<T> {
        SimpleStack {
            stack: RefCell::new(Vec::new()),
        }
    }
    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }
    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

fn main() {
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("part1: {:?}", map);
    let my_rc = MyRc::new(27);
    let rc_clone = my_rc.clone();
    println!("part2:");
    println!("strong_count: {}", my_rc.strong_count());
    println!("weak_count: {}", my_rc.weak_count());
    let stack = SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("part3:");
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
    stack.push(4);
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
}