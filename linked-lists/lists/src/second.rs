/// A Minimal Singly Linked List
/// This is a very basic singly linked list implementation.
/// It can perform the following operations on a stack:
/// - Push an element onto the stack
/// - Pop an element off the stack
/// - Peek at the top element of the stack

/// Public API for the List
pub struct List<T> {
    head: Link<T>,
}

/// Internal representation of the List.
/// A Link is just a primitive version of Option.
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }
    /// Push a new element onto the list
    pub fn push(&mut self, elem: T) {
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
    pub fn pop(&mut self) -> Option<T> {
        // Map Some(node) to Some(node.elem) using a closure
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        // Map Some(node) to Some(&node.elem) using a closure
        // map takes self by value, so we use as_ref to avoid moving
        // the value out of self
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}
