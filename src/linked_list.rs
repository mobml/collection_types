use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}

struct Node<T> {
    info: T,
    next: Link<T>,
}

impl<T> Node<T> {
    /// This function create a new instance of Node into smart pointer
    pub fn new(info: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { info, next: None }))
    }
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn append(&mut self, info: T) {
        let node = Node::new(info);

        match self.tail.take() {
            Some(old_node) => old_node.borrow_mut().next = Some(node.clone()),
            None => self.head = Some(node.clone()),
        }

        self.length += 1;
        self.tail = Some(node);
    }

    pub fn push_front(&mut self, info: T) {
        let node = Node::new(info);

        match self.head.take() {
            Some(old_node) => node.borrow_mut().next = Some(old_node),
            None => self.tail = Some(node.clone()),
        }
        self.head = Some(node);
        self.length += 1;
    }

    pub fn seek_tail(&self) -> Option<T> {
        match self.tail.clone() {
            Some(value) => Some(value.borrow().info),
            None => None,
        }
    }

    pub fn seek_head(&self) -> Option<T> {
        match self.head.clone() {
            Some(value) => Some(value.borrow().info),
            None => None,
        }
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            current: self.head.clone(),
        }
    }

    /// this function remove o node
    pub fn remove_head(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.borrow_mut().next.take();
                self.length -= 1;
                Some(node.borrow().info)
            }
            None => None,
        }
    }
}

pub struct ListIterator<T: Copy> {
    current: Link<T>,
}

impl<T: Copy> Iterator for ListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take();

        if current.is_none() {
            return None;
        }

        let current = current.unwrap();
        let current = current.borrow();
        self.current = current.next.clone();
        Some(current.info)
    }
}

#[macro_export]
macro_rules! list {
    ($($x:expr), *) => {
        {
            let mut temp_list = LinkedList::new();
            $( temp_list.append($x); )* temp_list
        }
    };
}
