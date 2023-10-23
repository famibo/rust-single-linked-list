
struct ListItem<T> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

struct SinglyLinkedList<T> {
    head: ListItem<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None
        }
    }
    fn next(&self) -> Option<&Self> {
        if let Some(next) = &self.next {
            Some(next.as_ref())    // OR Some(&*next)
        } else {
            None
        }
    }
    fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() {
            self.next.as_mut().unwrap().mut_tail()
        } else {
            self
        }
    }
    fn data(&self) -> &T {
        self.data.as_ref()
    }
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T> SinglyLinkedList<T> {
    fn new(data: T) -> Self {
        SinglyLinkedList {
            head: ListItem::new(data),
        }
    }
    fn append(&mut self, data: T) {
        //let tail = self.head.mut_tail();
        let mut tail = self.head.as_mut();
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap()
        }
        tail.next = Some(Box::new(ListItem::new(data)));
    }
    fn head(&self) -> &ListItem<T> {
        &self.head
    }
}


fn main() {
    let mut list = SinglyLinkedList::new("head");
    list.append("middle");
    list.append("middle_after_1");
    list.append("middle_after_2");
    list.append("before_tail");
    list.append("tail");
    let mut item = list.head();
    loop {
        println!("item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item;
        } else {
            break;
        }
    }
}
