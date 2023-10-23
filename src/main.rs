
mod linked_list;
use linked_list::SinglyLinkedList;

fn main() {
    let mut list = SinglyLinkedList::new("head");
    list.append("middle");
    list.append("middle_after_1");
    list.append("middle_after_2");
    list.append("before_tail");
    list.append("tail");
    println!("{}", list);
    let mut item = list.head();
    loop {
        println!("item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item;
        } else {
            break;
        }
    }
    let tail = list.tail();
    println!("tail: {}", tail.data());
}
