extern crate msgbox;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};
use msgbox::IconType;
use chrono::{Local, Timelike, Datelike};

pub struct TimeManager {
    time_day: String,
    time_hours: u32,
    time_minutes: u32,
    time_seconds: u32,
    time_meridian: String, // For twelve hour time aka AM and PM
}

impl TimeManager {
    pub fn new() -> Self {
        // Just null init I guess
        Self {
            time_day: "".to_string(),
            time_hours: 0,
            time_minutes: 0,
            time_seconds: 0,
            time_meridian: "".to_string(),
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

        println!("Day: {}  {}:{}:{} {}", self.time_day,
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

pub struct ConfigReader {
    contents: Vec<String>,
    file: File,
}

impl ConfigReader {
    pub fn new(file: &str) -> Self {
        Self {
            contents: Vec::new(),
            file: File::open(file).unwrap(),
        }
    }

    pub fn read(&mut self) {
        let reader = BufReader::new(&self.file);
        // Read by each line
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // ignore errors
            println!("{}. {}", index + 1, line);
        }
    }
}

fn message_box(contents: &str) {
    // Message box and borrow string contents
    match msgbox::create("Alarm", contents, IconType::Info) {
        Ok(()) => (),
        Err(msgbox::MsgBoxError::Create(_)) => {
            println!("Failed to create messagebox!");
        }
    }
}

fn main() {
    let mut time = TimeManager::new();
    let mut config = ConfigReader::new("alarm.conf");

    // Check the current time seconds and wait so we can get in sync
    if get_current_seconds() > 0 {
        println!("Waiting to sync: {}", 60 - get_current_seconds());
        thread::sleep(time::Duration::from_secs(60 - get_current_seconds() as u64));
    }

    'running: loop {
        time.get_time();
        config.read();

        thread::sleep(time::Duration::from_secs(60));
    }
}
