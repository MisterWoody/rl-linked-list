struct Node {
    element: u32,
    next: List,
}

enum List {
    Empty,
    Link(Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // This is bad as we're calling into the memory allocator just to get an empty node representation
        let list = List::Link(Box::new(Node {
            element: 1024,
            next: List::Empty,
        }));
    }
}

// See https://rust-unofficial.github.io/too-many-lists/first-layout.html
