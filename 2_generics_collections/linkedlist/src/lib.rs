struct Node<T> {
    element: T,
    next: Link<Node<T>>,
}

pub struct LinkedList<T> {
    head: Link<Node<T>>,
}

type Link<T> = Option<Box<T>>;

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: T) {
        let head = self.head.take();
        let new_node = Node {
            element,
            next: head,
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|value| {
            self.head = value.next;
            value.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|value| &value.element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        let list = LinkedList::<i32>::new();
        assert!(list.head.is_none());
    }

    #[test]
    fn push() {
        let mut list = LinkedList::new();
        list.push(1);
        assert!(list.head.is_some());
        assert_eq!(list.head.unwrap().element, 1);
    }

    #[test]
    fn pop() {
        let mut list = LinkedList::new();
        list.push("1");
        list.push("2");
        assert_eq!(list.pop(), Some("2"));
        assert_eq!(list.pop(), Some("1"));
        assert!(list.head.is_none());
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert!(list.peek().is_none());
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek(), Some(&1));
    }
}
