use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ConfigReader {
    pub contents: Vec<String>,
    file: File,
    line_content: Vec<String>,
}

impl ConfigReader {
    pub fn new(file: &str) -> Self {
        Self {
            contents: Vec::new(),
            file: File::open(file).unwrap(),
            line_content: Vec::new(),
        }
    }

    fn read(&mut self) {
        let reader = BufReader::new(&self.file);
        // Just set to null every time so we dont have to keep track
        self.contents = Vec::new();
        self.line_content = Vec::new(); 

        // Read by each line
        for (_i, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // ignore errors
            self.contents.push(line);
        }

        // Split each line of content by space
        for i in 0..self.contents.len() {
            println!("Line {}: {}\n", i + 1, self.contents[i]);
            for word in self.contents[i].split_whitespace() {
                self.line_content.push(word.to_string());
            }
        }
        // Print line elements by index
        for i in 0..self.line_content.len() {
            println!("Line element {}: {}", i + 1, self.line_content[i]);
        }
    }

    pub fn run(&mut self, config: &mut Config) {
        self.read();
        config.run(self);
    }
}

pub struct Config {
    pub days: Vec<String>,
    pub hours: Vec<u8>,
    pub minutes: Vec<u8>,
    pub meridians: Vec<String>,
    pub alarm_messages: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            days: Vec::new(),
            hours: Vec::new(),
            minutes: Vec::new(),
            meridians: Vec::new(),
            alarm_messages: Vec::new(),
        }
    }

    /* I seperated these into methods even 
       though I didn't need to just for neatness
       and to not confuse myself */

    fn store_days(&mut self, data: &ConfigReader) {
        // Set to null so we don't have to keep track
        self.days = Vec::new();

        for i in 0..data.contents.len() {
            for day in data.contents[i].split_whitespace() {
                self.days.push(day.to_string());
                break;
            }
        }

        for i in 0..self.days.len() {
            println!("Day {}: {}", i + 1, self.days[i]);
        }
    }

    fn store_time(&mut self, data: &ConfigReader) {
        // Set to null so we don't have to keep track
        self.hours = Vec::new();
        self.minutes = Vec::new();

        let mut time: Vec<String> = Vec::new();
        for i in 0..data.contents.len() {
            for (j, word) in data.contents[i].split_whitespace().enumerate() {
                if j == 1 { // 1 is the index of the time position
                    time.push(word.to_string());
                    break;
                }
            }
        }

        // Print the time
        for i in 0..time.len() {
            println!("Time {}: {}", i+1, time[i]);
        }

        // Store the hours
        for i in 0..time.len() {
            for hour in time[i].split(":") {
                let hour_converted: u8 = hour.to_string().parse().unwrap();
                self.hours.push(hour_converted);
                break;
            }
        }
        // Print the hours
        for i in 0..self.hours.len() {
            println!("Hour {}: {}", i+1, self.hours[i]);
        }

        //Store the minutes
         for i in 0..time.len() {
            for (j, minutes) in time[i].split(":").enumerate() {
                if j == 1 { // 1 is the position of the minutes
                    let minutes_converted: u8 = minutes.to_string().parse().unwrap();
                    self.minutes.push(minutes_converted);
                    break;
                }
            }
        }

        // Print the minutes
        for i in 0..self.minutes.len() {
            println!("Minutes {}: {}", i+1, self.minutes[i]);
        }
    }

    fn store_meridians(&mut self, data: &ConfigReader) {
        // Set to null so we don't have to keep track
        self.meridians = Vec::new();

        for i in 0..data.contents.len() {
            for (j, meridian) in data.contents[i].split_whitespace().enumerate() {
                if j == 2 { // 2 is the meridian index
                    self.meridians.push(meridian.to_string());
                    break;
                }
            }
        }

        // Print meridians
        for i in 0..self.meridians.len() {
            println!("Meridian {}: {}", i+1, self.meridians[i]);
        }
    }

    fn store_messages(&mut self, data: &ConfigReader) {
        // Set to null so we don't have to keep track
        self.alarm_messages = Vec::new();

        for i in 0..data.contents.len() {
            let mut the_message = String::new();
            for (j, message) in data.contents[i].chars().enumerate() {
                if j >= 12 { // the position
                    the_message.push_str(&message.to_string());
                }
            }
            self.alarm_messages.push(the_message.trim().to_string());
        }

        // Print messages
        for i in 0..self.alarm_messages.len() {
            println!("Message {}: {}", i+1, self.alarm_messages[i]);
        }
    }

    pub fn run(&mut self, data: &ConfigReader) {
        self.store_days(data);
        self.store_time(data);
        self.store_meridians(data);
        self.store_messages(data);
    }
}
