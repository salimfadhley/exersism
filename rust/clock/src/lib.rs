use std::fmt;
use std::fmt::Debug;

pub fn modulo(lhs: i32, rhs: i32) -> i32 {
    let r = lhs % rhs;
    match r {
        x if x == 0 => 0,
        x if x >= 0 => r,
        x => x + rhs,
    }
}

pub fn extract_remainder(n: i32, b: i32) -> (i32, i32) {
    let d: f64 = (n as f64) / (b as f64);
    let r: i32 = d.floor() as i32;
    let nn: i32 = modulo(n, b);
    (r, nn)
}

pub fn normal_time(h: i32, m: i32) -> (i32, i32) {
    let t0 = extract_remainder(m, 60);
    let t1 = extract_remainder(h + t0.0, 24);
    (t1.1, t0.1)
}

pub struct Clock {
    hours: i32,
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
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_time = normal_time(self.hours, self.minutes + minutes);

        Clock {
            hours: new_time.0,
            minutes: new_time.1,
        }
    }
}
