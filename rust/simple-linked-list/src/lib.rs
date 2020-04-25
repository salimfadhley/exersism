use std::iter::FromIterator;

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Default)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut length: usize = 0;
        let mut n = &self.head;

        while n.is_some() {
            length += 1;
            n = &n.as_ref().unwrap().next
        }

        length
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, _element: T) {
        let head = self.head.take();
        self.head.replace(Box::new(Node::new(_element, head)));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();

        match head {
            Some(x) => {
                self.head = x.next;
                Some(x.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {

        match &self.head {
            Some(x) => Some(&x.data),
            None => None
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut oldll: SimpleLinkedList<T> = self;
        let mut newll: SimpleLinkedList<T> = SimpleLinkedList::new();


        while oldll.head.is_some() {
            newll.push(oldll.pop().unwrap());
        }

        newll
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ll = SimpleLinkedList::new();
        _iter.into_iter().for_each(|x|ll.push(x));
        ll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut my:SimpleLinkedList<T> = self.rev();
        let mut v: Vec<T> = vec![];

        while my.head.is_some() {
            v.push(my.pop().unwrap());
        }
        v

    }
}
