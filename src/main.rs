use chrono::{DateTime, Local, NaiveTime};

struct Schedule {
    time: NaiveTime,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
}

impl Schedule {
    fn get_next(&self) -> DateTime<Local> {
        todo!();
    }
}

fn main() {}
