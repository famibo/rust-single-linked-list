
mod linked_list;
mod linked_list_rc;
use linked_list_rc::SinglyLinkedList;
use std::rc::Rc;
use crate::linked_list_rc::ItemRefExt;

fn main() {
    let mut list = SinglyLinkedList::new("head");
    list.append("middle");
    list.append("middle_after_1");
    list.append("middle_after_2");
    list.append("before_tail");
    list.append("tail");
    println!("{}", list);

    let mut item = list.head();
    println!("item inner_element: {}", item.inner_element().data());

    loop {
        println!("item: {}", item.borrow().data());
        if let Some(next_item) = Rc::clone(&item).borrow().next() {
            item = next_item;
        } else {
            break;
        }
    }

    let tail = list.tail();
    println!("tail: {}", tail.borrow().data());
}
