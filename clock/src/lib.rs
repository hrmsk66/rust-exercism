use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    m: i32,
}

impl Clock {
    fn normalize(&self) -> Self {
        Clock { m: self.m.rem_euclid(1440) }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { m: hours * 60 + minutes }.normalize()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { m: self.m + minutes }.normalize()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.m / 60, self.m % 60)
    }
}