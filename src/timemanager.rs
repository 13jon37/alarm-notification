use chrono::{Local, Timelike, Datelike};

pub struct TimeManager {
    pub time_day: String,
    pub time_hours: u8,
    pub time_minutes: u8,
    pub time_seconds: u8,
    pub time_meridian: String, // For twelve hour time aka AM and PM
}

impl TimeManager {
    pub fn new() -> Self {
        // Just null init I guess
        Self {
            time_day: String::new(),
            time_hours: 0,
            time_minutes: 0,
            time_seconds: 0,
            time_meridian: String::new(),
        }
    }

    pub fn get_time(&mut self) {
        let time = Local::now();
        self.time_day = time.weekday().to_string();
        self.time_hours = time.hour() as u8;

        if self.time_hours > 12 {
            // Convert to 12 hour time
            self.time_hours -= 12;
        }

        // Figure out if its AM or PM
        if time.hour() < 12 {
            self.time_meridian = "AM".to_string();
        }
        if time.hour() > 12 {
            self.time_meridian = "PM".to_string();
        } 

        self.time_minutes = time.minute() as u8;
        self.time_seconds = time.second() as u8;
    }

    pub fn get_day(&self) -> &str {
        return &self.time_day;
    }

    pub fn get_hours(&self) -> &u8 {
        return &self.time_hours;
    }

    pub fn get_minutes(&self) -> &u8 {
        return &self.time_minutes;
    }

    pub fn get_meridian(&self) -> &str {
        return &self.time_meridian;
    }
}

pub fn get_current_seconds() -> u8 {
    let time = Local::now();
    return time.second() as u8;
}
