type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            len: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let value = element;
        let next = self.head.take();
        self.head = Some(Box::new(Node { value, next }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            if let Some(next) = head.next {
                self.head = Some(next);
            }
            self.len -= 1;
            return Some(head.value);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(ref head) = self.head { Some(&head.value) } else { None }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let members = Vec::from(self);
        SimpleLinkedList::from_iter(members.into_iter().rev())
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(SimpleLinkedList::new(), |mut acc, item| {
            acc.push(item);
            acc
        })
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut members = Vec::<T>::new();
        let mut curr = linked_list.head;
        loop {
            if let Some(head) = curr {
                members.push(head.value);
                if let Some(next) = head.next {
                    curr = Some(next);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        members.into_iter().rev().collect()
    }
}
