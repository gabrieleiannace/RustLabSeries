use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{hours, minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        todo!("Add {minutes} minutes to existing Clock time");
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
