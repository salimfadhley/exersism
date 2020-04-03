use std::fmt;
use std::fmt::Debug;

pub fn minutes_to_hours_and_minutes(mm:i32) -> (i32,i32) {
    let hours: i32 = mm.div_euclid(mm / 60);
    let minutes: i32 =
}

pub struct Clock {
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        normal_time(self.hours, self.minutes) == normal_time(other.hours, other.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nt: (i32, i32) = normal_time(self.hours, self.minutes);

        write!(f, "{:02}:{:02}", nt.0, nt.1)
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes:(hours * 60) + minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_time =

        Clock {
            minutes: self.minutes + minutes;
        }
    }
}
