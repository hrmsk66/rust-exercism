use std::collections::BTreeMap;

#[derive(Default)]
pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades
            .entry(grade)
            .or_insert(vec![])
            .push(String::from(student))
    }
    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect::<Vec<u32>>()
    }
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).cloned().map(|mut names| {
            names.sort();
            names
        })
    }
}