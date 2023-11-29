use std::mem;
use std::ptr::replace;
use crate::first::Link::More;

pub struct List {
    head : Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    elem : i32,
    next : Link
}

impl List {
   fn new()-> Self  {
       List {head: Link::Empty}
   }
    fn push(&mut self, val : i32) {
        let node = Box::new(Node {
            elem : val,
            next : mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None
            }
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}

mod test {
    use crate::first::{Link, List};

    #[test]
    fn basics() {
        let mut list = List::new();

       // Check empty list behaves right
        assert_eq!(list.pop(),None);


        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);


        // Check normal removal
        assert_eq!(list.pop(),Some(3));
        assert_eq!(list.pop(),Some(2));
        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(),Some(5));
        assert_eq!(list.pop(),Some(4));

        // Check exhaustion
        assert_eq!(list.pop(),Some(1));
        assert_eq!(list.pop(),None);
    }
}