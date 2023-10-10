use std::cell::RefCell;
use std::rc::Rc;

type SingleLink<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: SingleLink<T>,
}

#[derive(Debug)]
struct TransactionLog<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    pub length: u64,
}

impl<T> Node<T> {
    // a nice way of creating a new node
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
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
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };

        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
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
}

fn main() {
    let mut list = TransactionLog::new_empty();
    list.append("test");
    list.append("1");
    list.append("mint");
    println!("{:?}", list);
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
