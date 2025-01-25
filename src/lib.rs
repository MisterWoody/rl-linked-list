#[derive(Debug)]
pub struct LinkedList {
    head: Link,
}

impl LinkedList {
    pub fn empty() -> LinkedList {
        Self { head: None }
    }

    pub fn push(&mut self, element: u32) {
        // We can't use match on the Some case, as this tries to move data out of the list
        // this isn't allowed as data is only borrowed not owned -
        // std::mem replace functions comes into play
        // let old_head = std::mem::replace(&mut self.head, None);
        // This is such a common pattern, that the following take function is exactly equivalent
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<u32> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.element
        })
    }

    pub fn peek(&self) -> Option<&u32> {
        self.head.as_ref().map(|n| &n.element)
    }
}

#[derive(Debug)]
struct Node {
    element: u32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::empty();
        list.push(1024);
        list.push(42);
        dbg!(list);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::empty();
        list.push(1042);
        let elem = list.pop();
        assert_eq!(elem, Some(1042));
    }

    #[test]
    fn test_peek() {
        let mut list = LinkedList::empty();
        list.push(43);
        assert_eq!(list.peek(), Some(&43));
    }
}

// See https://rust-unofficial.github.io/too-many-lists/first-layout.html
