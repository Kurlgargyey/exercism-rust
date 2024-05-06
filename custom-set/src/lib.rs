use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Eq)]
pub struct CustomSet<T: Clone> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    data: Vec<T>,
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter().all(|e| other.contains(e)) && other.data.iter().all(|e| self.contains(e))
    }
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut data = Vec::<T>::from(input);
        data.dedup();

        CustomSet { data }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.iter().any(|e| e == element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element)
        };
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|e| other.contains(e))
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.iter().all(|e| !other.contains(e))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet {
            data: self
                .data
                .clone()
                .into_iter()
                .filter(|e| other.contains(e))
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        CustomSet {
            data: self
                .data
                .clone()
                .into_iter()
                .filter(|e| !other.contains(e))
                .collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut union = self.data.clone();
        union.append(&mut other.data.clone());
        CustomSet { data: union }
    }
}
