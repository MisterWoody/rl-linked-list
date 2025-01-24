enum Node {
    Empty,
    NonEmpty(u32, Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // This is bad as we're calling into the memory allocator just to get an empty node representation
        let list = Node::NonEmpty(1091, Box::new(Node::Empty));
    }
}
