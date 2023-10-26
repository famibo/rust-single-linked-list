use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

type ItemRef<T> = Rc<RefCell<ListItem<T>>>;
#[derive(Debug)]
pub struct ListItem<T> {
    data: Box<T>,
    next: Option<ItemRef<T>>,
}

#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: ItemRef<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None
        }
    }
    pub fn next(&self) -> Option<ItemRef<T>> {
        if let Some(next) = &self.next {
            Some(Rc::clone(next))
        } else {
            None
        }
    }

    pub fn data(&self) -> &T {
        self.data.as_ref()
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new(data: T) -> Self {
        SinglyLinkedList {
            head: Rc::new(RefCell::new(ListItem::new(data))),
        }
    }
    pub fn append(&mut self, data: T) {
        let tail = Self::find_tail(Rc::clone(&self.head));
        let new_item = Rc::new(RefCell::new(ListItem::new(data)));
        tail.borrow_mut().next = Some(new_item);
    }
    pub fn head(&self) -> ItemRef<T> {
        Rc::clone(&self.head)
    }

    pub fn tail(&self) -> ItemRef<T> {
        Self::find_tail(self.head())
    }
    fn find_tail(item: ItemRef<T>) -> ItemRef<T> {
        if let Some(next) = &item.borrow().next {
            Self::find_tail(Rc::clone(next))
        } else {
            Rc::clone(&item)
        }
    }
}

impl<T: std::fmt::Debug> fmt::Display for SinglyLinkedList<T>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}