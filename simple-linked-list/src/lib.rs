#[derive(Clone, Debug)]
pub struct SimpleLinkedList<T> {
    value: Option<T>,
    next: Option<Box<SimpleLinkedList<T>>>,
}

impl<T> SimpleLinkedList<T> where T: Clone {
    pub fn new() -> Self {
        SimpleLinkedList {
            value: None,
            next: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut curr = Some(self);
        loop {
            if let Some(&SimpleLinkedList { value: Some(_), next: _ }) = curr {
                len += 1;
            } else {
                break;
            }
            if let Some(&SimpleLinkedList { value: _, next: Some(ref next) }) = curr {
                curr = Some(next);
            } else {
                break;
            }
        }
        len
    }

    pub fn push(&mut self, element: T) {
        let value = Some(element);
        let curr_value = self.value.clone();
        let curr_next = self.next.clone();
        let new_next = SimpleLinkedList { value: curr_value, next: curr_next };
        self.value = value;
        self.next = Some(Box::new(new_next));
    }

    pub fn pop(&mut self) -> Option<T> {
        let curr_value = self.value.clone();
        if let Some(curr_next) = self.next.clone() {
            *self = *curr_next;
        } else {
            self.value = None;
        }

        curr_value
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(value) = &self.value { Some(value) } else { None }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let members = Vec::from(self);
        SimpleLinkedList::from_iter(members.into_iter().rev())
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> where T: Clone {
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

impl<T> From<SimpleLinkedList<T>> for Vec<T> where T: Clone {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut members = Vec::<T>::new();
        let mut curr = Some(&linked_list);
        loop {
            if let Some(&SimpleLinkedList { value: Some(ref value), next: _ }) = curr {
                members.push(value.clone());
            } else {
                break;
            }
            if let Some(&SimpleLinkedList { value: _, next: Some(ref next) }) = curr {
                curr = Some(next);
            } else {
                break;
            }
        }
        members.into_iter().rev().collect()
    }
}
