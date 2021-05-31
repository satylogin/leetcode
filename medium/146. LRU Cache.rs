use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: (i32, i32),
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(data: (i32, i32)) -> Self {
        Node {
            data,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl List {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, data: (i32, i32)) -> Option<Rc<RefCell<Node>>> {
        let node = Rc::new(RefCell::new(Node {
            data,
            prev: None,
            next: self.head.as_ref().map(|head| Rc::clone(head)),
        }));
        if let Some(head) = &self.head {
            head.borrow_mut().prev = Some(Rc::clone(&node));
        }
        if self.tail.is_none() {
            self.tail = Some(Rc::clone(&node));
        }
        self.head = Some(Rc::clone(&node));

        Some(Rc::clone(self.head.as_ref().unwrap()))
    }

    fn pop(&mut self) -> Option<Rc<RefCell<Node>>> {
        if self.head.is_none() {
            return None;
        }

        let safe_cur = Rc::clone(self.tail.as_ref().unwrap());

        if let Some(prev) = safe_cur.borrow().prev.as_ref() {
            prev.borrow_mut().next = None;
            self.tail = Some(Rc::clone(prev));
        } else {
            self.tail = None;
        }

        safe_cur.borrow_mut().prev = None;

        if self.tail.is_none() {
            self.head = None;
        }

        Some(Rc::clone(&safe_cur))
    }

    fn promote(&mut self, node: &Rc<RefCell<Node>>) {
        if self.head.as_ref().unwrap().as_ptr() == node.as_ptr() {
            return;
        }
        if self.tail.as_ref().unwrap().as_ptr() == node.as_ptr() {
            self.tail = node.borrow().prev.as_ref().map(|node| Rc::clone(node));
        }
        if let Some(prev) = node.borrow().prev.as_ref() {
            dbg!(prev.borrow().data);
            prev.borrow_mut().next = node.borrow().next.as_ref().map(|node| Rc::clone(node));
        }
        if let Some(next) = node.borrow().next.as_ref() {
            dbg!(next.borrow().data);
            next.borrow_mut().prev = node.borrow().prev.as_ref().map(|node| Rc::clone(node));
        }
        node.borrow_mut().next = Some(Rc::clone(self.head.as_ref().unwrap()));
        node.borrow_mut().prev = None;
        self.head.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(&node));
        self.head = Some(Rc::clone(&node))
    }
}

struct LRUCache {
    mem: HashMap<i32, Option<Rc<RefCell<Node>>>>,
    list: List,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            mem: HashMap::new(),
            list: List::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.mem.get(&key) {
            self.list.promote(node.as_ref().unwrap());
            return node.as_ref().unwrap().borrow().data.1;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.mem.get(&key) {
            node.as_ref().unwrap().borrow_mut().data = (key, value);
            self.list.promote(node.as_ref().unwrap());
            return;
        }
        if self.mem.len() == self.capacity {
            let popped = self.list.pop();
            self.mem.remove(&popped.as_ref().unwrap().borrow().data.0);
        }
        self.mem.insert(key, self.list.push((key, value)));
    }
}
