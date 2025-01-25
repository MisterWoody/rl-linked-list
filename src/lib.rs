pub struct LinkedList {
    head: Link,
}

impl LinkedList {
    pub fn empty() -> LinkedList {
        LinkedList { head: None }
    }
}

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
        let list = LinkedList::empty();
    }
}

// See https://rust-unofficial.github.io/too-many-lists/first-layout.html
