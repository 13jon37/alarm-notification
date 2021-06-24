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

pub struct ConfigReader {
    contents: Vec<String>,
    file: File,
    num_of_lines: u32,
}

impl ConfigReader {
    pub fn new(file: &str) -> Self {
        Self {
            contents: Vec::new(),
            file: File::open(file).unwrap(),
            num_of_lines: 0,
        }
    }

    pub fn read(&mut self) {
        let reader = BufReader::new(&self.file);
        // Read by each line
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // ignore errors
            self.contents.push(line);
            self.num_of_lines = (index as u32) + 1;
        }
        
        let mut count = 0;
        for i in 0..self.num_of_lines {
            println!("Content {}: {}", i + 1, self.contents[count]);
            count += 1;
        }
    }
}

pub struct Alarm {
    day: String,
    hours: u32,
    minutes: u32,
    meridian: String,
}

impl Alarm {
    pub fn new() -> Self {
        Self {
            day: String::from(""),
            hours: 0,
            minutes: 0,
            meridian: String::from(""),
        }
    }

    pub fn alarm_system(&self, time: &TimeManager, config: &ConfigReader) {
        let son = String::from("Son");
        let mon = String::from("Mon");
        let tue = String::from("Tue");
        let wed = String::from("Wed");
        let thu = String::from("Thu");
        let fri = String::from("Fri");
        let sat = String::from("Sat");

        match &time.time_day {
           thu  => message_box(time.get_day()),
        }

        // Have match statements here
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
    // Init everything
    let mut time = TimeManager::new();
    let mut config = ConfigReader::new("alarm.conf");
    let alarm = Alarm::new();

    // Check the current time seconds and wait so we can get in sync
   /* if get_current_seconds() > 0 {
        println!("Waiting to sync: {}", 60 - get_current_seconds());
        thread::sleep(time::Duration::from_secs(60 - get_current_seconds() as u64));
    }*/

    loop {
        // Start time manager and read the config
        time.get_time();
        config.read();

        // Alarm system
        alarm.alarm_system(&time, &config);

        thread::sleep(time::Duration::from_secs(60));
    }
}
