use std::fmt;

#[derive(Debug,PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        hours += minutes / 60;
        hours %= 24;
        minutes %= 60;

        let (hours, minutes) = Clock::normalize(hours, minutes);
        Clock { hours, minutes }
    }

    fn normalize(mut h: i32, m: i32) -> (i32, i32) {
        if m < 0 {
            h -= 1;
        }

        ((h + 24) % 24, (m + 60) % 60)
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        self.hours += self.minutes / 60;
        self.hours %= 24;
        self.minutes %= 60;

        let (hours, minutes) = Clock::normalize(self.hours, self.minutes);
        Clock { hours, minutes }
    }
}
