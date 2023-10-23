use std::fmt;
#[derive(Debug)]
pub struct ListItem<T> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: ListItem<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None
        }
    }
    pub fn next(&self) -> Option<&Self> {
        if let Some(next) = &self.next {
            Some(next.as_ref())    // OR Some(&*next)
        } else {
            None
        }
    }
    pub fn data(&self) -> &T {
        self.data.as_ref()
    }
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new(data: T) -> Self {
        SinglyLinkedList {
            head: ListItem::new(data),
        }
    }
    pub fn append(&mut self, data: T) {
        let mut tail = self.head.as_mut();
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap()
        }
        tail.next = Some(Box::new(ListItem::new(data)));
    }
    pub fn head(&self) -> &ListItem<T> {
        &self.head
    }

    pub fn tail(&self) -> &ListItem<T> {
        let mut tail = &self.head;
        while tail.next.is_some() {
            tail = tail.next.as_ref().unwrap()
        }
        tail
    }
}

impl<T: std::fmt::Debug> fmt::Display for SinglyLinkedList<T>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}