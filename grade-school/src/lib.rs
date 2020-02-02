use std::collections::HashMap;

pub struct School { students: HashMap<u32, Vec<String>> }

impl School {
    pub fn new() -> School {
        School { students: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.students.contains_key(&grade) {
            self.students.get_mut(&grade).unwrap().push(student.to_string());
        } else {
            self.students.insert(grade, vec![student.to_string()]);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut r = self.students.keys().map(|x| x.clone()).collect::<Vec<u32>>();
        r.sort();
        r
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if let Some(g) = self.students.get(&grade) {
            let mut r = g.clone();
            r.sort();
            Some(r)
        } else {
            None
        }
    }
}
