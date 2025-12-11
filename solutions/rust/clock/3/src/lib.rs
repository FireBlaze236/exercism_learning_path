pub struct Clock {
    display_mins: i32,
    display_hours: i32,
}
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

impl Clock {
    fn get_seconds(hours: i32, minutes: i32) -> i32 {
        let hs = hours * 3600;
        let ms = minutes * 60;

        let t = hs + ms;
        Clock::wrap_seconds(t)
    }

    fn wrap_seconds(seconds: i32) -> i32 {
        const SECONDS_IN_DAY: i32 = 24 * 3600;
        match seconds.cmp(&0) {
            Ordering::Less => (SECONDS_IN_DAY) + (seconds % SECONDS_IN_DAY),
            Ordering::Equal => 0,
            Ordering::Greater => (seconds) % (SECONDS_IN_DAY),
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");

        let seconds = Clock::get_seconds(hours, minutes);

        let hh = seconds / 3600;
        let mm = (seconds % 3600) / 60;

        Clock {
            display_mins: mm,
            display_hours: hh,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");

        let cur_seconds = Clock::get_seconds(self.display_hours, self.display_mins);
        let add_seconds = Clock::get_seconds(0, minutes);

        let seconds = Clock::wrap_seconds(cur_seconds + add_seconds);

        let hh = seconds / 3600;
        let mm = (seconds % 3600) / 60;

        Clock {
            display_mins: mm,
            display_hours: hh,
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
