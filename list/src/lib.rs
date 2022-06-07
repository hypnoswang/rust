//! Box<T>的使用练习,实现一个链表

pub struct Node {
    pub v: i32,
    pub next: Option<Box<Node>>,
}

pub struct List {
    pub head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> List {
        List { head: None }
    }

    pub fn push(&mut self, n: Node) {
        if let None = self.head {
            self.head = Some(Box::new(n));
            return;
        }

        let mut pn = &mut self.head;
        while let Some(node) = pn.as_mut() {
            if let None = node.next {
                node.next = Some(Box::new(n));
                break;
            } else {
                pn = &mut node.next;
            }
        }
    }

    pub fn display(&self) {
        let mut pn = &self.head;
        while let Some(n) = pn {
            print!("{} ", n.v);
            pn = &n.next;
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use super::Node;

    #[test]
    fn create_and_display() {
        let mut l = List::new();

        for i in 1..10 {
            let n = Node { v: i, next: None };
            l.push(n);
        }

        l.display();
    }
}
