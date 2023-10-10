use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

#[derive(Clone)]
struct TransactionLog<T> {
    head: Link<T>,
    tail: Link<T>,
    pub length: u64,
}

pub struct ListIterator<T> {
    current: Link<T>,
}

impl<T> Node<T> {
    // a nice way of creating a new node
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None,
        }))
    }
}

impl<T> TransactionLog<T>{
    pub fn new_empty() -> TransactionLog<T> {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            },
            None => self.head = Some(new.clone()),
        };

        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }

    pub fn back_iter(self) -> ListIterator<T> { 
        ListIterator::new(self.tail)
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator::new(self.head.clone())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for TransactionLog<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransactionLog")
            .field("length", &self.length)
            .finish()
    }
}

impl<T: Clone> IntoIterator for TransactionLog<T> {
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head)
    }
}

impl<T> ListIterator<T> {
    fn new(start_at: Link<T>) -> ListIterator<T> {
        ListIterator {
            current: start_at,
        }
    }
}

impl<T: Clone> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },
            None => None
        };
        result
    }
}

impl<T: Clone> DoubleEndedIterator for ListIterator<T> {
    fn next_back(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            },
            None => None
        };
        result
    }
}

fn main() {
    let mut list = TransactionLog::new_empty();
    list.append("test".to_owned());
    list.append("1".to_owned());
    list.append("mint".to_owned());
    println!("{:?}", list);


    let mut iter = list.clone().into_iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    let mut iter = list.clone().back_iter();
    println!("{:?}", iter.next_back());
    println!("{:?}", iter.next_back());
    println!("{:?}", iter.next_back());

    match list.pop() {
        Some(value) => println!("{value}"),
        None => println!("empty"),
    };
    match list.pop() {
        Some(value) => println!("{value}"),
        None => println!("empty"),
    };
    match list.pop() {
        Some(value) => println!("{value}"),
        None => println!("empty"),
    };
    match list.pop() {
        Some(value) => println!("{value}"),
        None => println!("empty"),
    };
    println!("{:?}", list);
}
