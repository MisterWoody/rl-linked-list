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
        let old_head = std::mem::replace(&mut self.head, None);
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);

        // match self.head {
        //     None => {
        //         self.head = Some(Box::new(Node {
        //             element,
        //             next: None,
        //         }))
        //     }
        //     Some(n) => {
        //         let new_head = Some(Box::new(Node {
        //             element,
        //             next: Some(n),
        //         }));
        //         self.head = new_head;
        //     }
        // }
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
}

// See https://rust-unofficial.github.io/too-many-lists/first-layout.html
