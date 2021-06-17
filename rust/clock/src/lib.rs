use std::fmt;


const HOUR_LENGTH: i32 = 60;
const HOURS_PER_DAY: i32 = 24;
const DAY_LENGTH: i32 = HOURS_PER_DAY * HOUR_LENGTH;

fn limit_to_day(time: i32) -> i32 {
    time % DAY_LENGTH
}

#[derive(PartialEq, Debug)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time = (hours * HOUR_LENGTH) + minutes;

        if time < 0 {
            return Clock(DAY_LENGTH - limit_to_day(time).abs());
        }

        Clock(limit_to_day(time))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if minutes < 0 {
            let negative_min = limit_to_day(minutes).abs();
            let new_time = limit_to_day(self.0 - negative_min);

            if new_time < 0 {
                return Clock(DAY_LENGTH + new_time);
            }

            return Clock(new_time);
        }

        Clock(limit_to_day(self.0 + minutes))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = (self.0 / HOUR_LENGTH) % HOURS_PER_DAY;
        let minutes = (self.0 - (hours * HOUR_LENGTH)) % HOUR_LENGTH;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
