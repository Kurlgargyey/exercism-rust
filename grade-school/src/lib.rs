use std::collections::{ BTreeMap, BTreeSet };

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> School {
        School(BTreeMap::<u32, BTreeSet<String>>::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let success = self.0
            .entry(grade)
            .or_insert(BTreeSet::from([student.to_string()]))
            .insert(student.to_string());
        if !success {
            println!(
                "You attempted to add {:?} twice, but they are already on the roster.",
                student
            )
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = Vec::<u32>::new();
        for key in self.0.keys() {
            result.push(*key);
        }
        result
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.0
            .get(&grade)
            .unwrap_or(&BTreeSet::<String>::new())
            .into_iter()
            .map(|student| student.clone())
            .collect()
    }
}
