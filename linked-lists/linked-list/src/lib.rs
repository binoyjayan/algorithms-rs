#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    /// Create a new empty linked list
    pub fn new() -> Self {
        LinkedList(None)
    }

    /// Push an element to the front of the list
    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    /// Push an element to the back of the list
    pub fn push_back(&mut self, data: T) {
        match self.0 {
            None => self.push_front(data),
            Some((ref mut _head, ref mut tail)) => tail.push_back(data),
        }
    }

    /// Pop an element from the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        match self.0.take() {
            None => None,
            Some((data, tail)) => {
                self.0 = tail.0;
                Some(data)
            }
        }
    }

    /// Pop an element from the back of the list
    pub fn pop_back(&mut self) -> Option<T> {
        self.0.take().map(|(data, mut tail)| match tail.0 {
            None => data,
            Some(_) => {
                let result = tail.pop_back();
                self.0 = Some((data, tail));
                result.unwrap()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front_pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
    }

    #[test]
    fn test_push_back_pop_front() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn test_push_back_pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
    }

    #[test]
    fn test_push_front_pop_back() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
    }
}
