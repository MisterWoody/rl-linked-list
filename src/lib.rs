struct LinkedList {
    head: Link,
}

struct Node {
    element: u32,
    next: Link,
}

enum Link {
    Empty,
    NonEmpty(Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // This is bad as we're calling into the memory allocator just to get an empty node representation
        let list = Link::NonEmpty(Box::new(Node {
            element: 1024,
            next: Link::Empty,
        }));
    }
}

// See https://rust-unofficial.github.io/too-many-lists/first-layout.html
