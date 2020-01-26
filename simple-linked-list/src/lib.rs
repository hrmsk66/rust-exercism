use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize
}
impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            len: 0
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take()
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            None => None,
            _ => {
                let head = *self.head.take().unwrap();
                let resp = head.data;
                self.head = head.next;
                self.len -= 1;

                Some(resp)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(v) => {
                Some(&v.data)
            }
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();

        let mut node = self.head;

        while let Some(v) = node {
            list.push(v.data);
            node =v.next;
        }

        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for el in _iter {
            list.push(el);
        }
        list
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
        let mut resp = vec![];

        let mut node = self.head;

        while let Some(v) = node {
            resp.push(v.data);
            node = v.next;
        }

        resp.reverse();
        resp
    }
}