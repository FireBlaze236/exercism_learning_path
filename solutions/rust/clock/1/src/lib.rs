pub struct Clock {
    display_mins: i32,
    display_hours: i32,
}
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");

        let mm = minutes % 60;
        let hh = (hours + (minutes / 60)) % 24;

        Clock {
            display_mins: mm,
            display_hours: hh,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");

        let new_mins = self.display_mins + minutes;
        let new_hours = (self.display_hours + new_mins / 60) % 24;

        Clock {
            display_mins: (new_mins) % 60,
            display_hours: new_hours,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.display_hours, self.display_mins)
    }
}

impl std::fmt::Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.display_hours, self.display_mins)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.display_mins == other.display_mins && self.display_hours == other.display_hours
    }
}
