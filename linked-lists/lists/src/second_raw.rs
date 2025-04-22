/// A Singly Linked List generic over type 'T'
/// This is a very basic singly linked list implementation.
/// It can perform the following operations on a stack:
/// - Push an element onto the stack
/// - Pop an element off the stack
/// - Peek at the top element of the stack
/// - Iterate over the elements of the stack
use std::ptr;

/// Public API for the List
pub struct List<T> {
    head: Link<T>,
}

/// Internal representation of the List.
/// Link is a pointer to the next node in the list.
type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::into_raw(Box::new(Node {
            elem: elem,
            next: self.head,
        }));
        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let node = unsafe { Box::from_raw(self.head) };
            self.head = node.next;
            Some(node.elem)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            let node = unsafe { &*self.head };
            Some(&node.elem)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.head.is_null() {
            None
        } else {
            let node = unsafe { &mut *self.head };
            Some(&mut node.elem)
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head;
        while !cur.is_null() {
            unsafe {
                let node = Box::from_raw(cur);
                cur = node.next;
            }
        }
    }
}

/// into_iter(): An owned iterator for the list
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    /// Return an owned iterator for the list
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/// iter(): An immutable reference iterator for the list
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    // Lifetimes are elided here, make that explicit using '_
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: unsafe { self.head.as_ref() },
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = unsafe { node.next.as_ref() };
            &node.elem
        })
    }
}

/// iter_mut(): A mutable reference iterator for the list
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: unsafe { self.head.as_mut() },
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // Use take to avoid copying the mutable reference (Option<&T>).
        // It is not allowed since only one mutable reference can exit at a time.
        self.next.take().map(|node| {
            self.next = unsafe { node.next.as_mut() };
            &mut node.elem
        })
    }
}

/// Implementing the trait IntoIterator helps using a list object
/// in for a loop without explicitly calling into_iter()
/// Since IntoIterator is implemented for List, the
/// 'impl<T> List<T>' implementation is redundant.
impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

/// Implementing IntoIterator for an immutable reference to List
impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Implementing IntoIterator for a mutable reference to List
impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
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

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn into_iter_for() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut result = Vec::new();
        // Call into_iter() is not needed if IntoIterator is implemented
        for i in list.into_iter() {
            result.push(i);
        }
        assert_eq!(result, vec![3, 2, 1]);
    }
    #[test]
    fn into_iter_for_implicit() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut result = Vec::new();
        // Not calling into_iter() explicitly since IntoIterator is implemented
        for i in list {
            result.push(i);
        }
        assert_eq!(result, vec![3, 2, 1]);
    }
    #[test]
    fn into_iter_for_ref() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut result = Vec::new();
        // Immutable reference to list
        for i in &list {
            result.push(i);
        }
        let expected = vec![&3, &2, &1];
        assert_eq!(result, expected);
    }
    #[test]
    fn into_iter_for_mut_ref() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut result = Vec::new();
        // Immutable reference to list
        for i in &mut list {
            result.push(i);
        }
        let expected = vec![&3, &2, &1];
        assert_eq!(result, expected);
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
