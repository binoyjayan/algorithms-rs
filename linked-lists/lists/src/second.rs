use std::mem;

/// Public API for the List
pub struct List {
    head: Link,
}

/// Internal representation of the List.
/// A Link is just a primitive version of Option.
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::None }
    }
}

impl List {
    /// Push a new element onto the list
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // steal a value out of a borrow to replace it with a empty link
            // head is now Empty and the new node points to the old head
            next: self.head.take(),
        });
        // Make head point to the new node
        self.head = Link::Some(new_node);
    }

    /// Pop an element off the list
    pub fn pop(&mut self) -> Option<i32> {
        // Map Some(node) to Some(node.elem) using a closure
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
