use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours:i32 = hours;
        let mut minutes:i32 = minutes;

        let mut rip = minutes/60;
        if minutes%60 < 0 {
            rip += -1;
            minutes = 60 + minutes%60;
        }

        hours = (hours+rip)%24;
        if hours%24 < 0 {
            hours = 24 + hours%24;
        }

        Clock{hours, minutes: minutes%60}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hours= String::new();
        if self.hours < 10 { hours.push_str("0");}
        hours.push_str(self.hours.to_string().as_str());

        let mut minutes: String = String::new();
        if self.minutes < 10 { minutes.push_str("0");}
        minutes.push_str(self.minutes.to_string().as_str());

        write!(f, "{}:{}", hours, minutes)
    }
}
