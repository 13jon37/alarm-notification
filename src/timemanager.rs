use chrono::{Local, Timelike, Datelike};

pub struct TimeManager {
    pub time_day: String,
    pub time_hours: u32,
    pub time_minutes: u32,
    pub time_seconds: u32,
    pub time_meridian: String, // For twelve hour time aka AM and PM
}

impl TimeManager {
    pub fn new() -> Self {
        // Just null init I guess
        Self {
            time_day: String::from(""),
            time_hours: 0,
            time_minutes: 0,
            time_seconds: 0,
            time_meridian: String::from(""),
        }
    }

    pub fn get_time(&mut self) {
        let time = Local::now();
        self.time_day = time.weekday().to_string();
        self.time_hours = time.hour();

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

        self.time_minutes = time.minute();
        self.time_seconds = time.second();

        println!("Day: {}  {}:{}:{} {}\n", self.time_day,
                                         self.time_hours,
                                         self.time_minutes,
                                         self.time_seconds,
                                         self.time_meridian);
    }

    pub fn get_day(&self) -> &str {
        return &self.time_day;
    }

    pub fn get_hours(&self) -> &u32 {
        return &self.time_hours;
    }

    pub fn get_minutes(&self) -> &u32 {
        return &self.time_minutes;
    }

    pub fn get_seconds(&self) -> &u32 {
        return &self.time_seconds;
    }
}

pub fn get_current_seconds() -> u32 {
    let time = Local::now();
    return time.second();
}




