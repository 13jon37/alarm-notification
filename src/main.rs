mod timemanager;
mod config;
mod alarm;

use std::{thread, time};
use chrono::{Local, Timelike};


pub fn get_current_seconds() -> u32 {
    let time = Local::now();
    return time.second();
}

fn main() {
    // Init everything
    let mut time = timemanager::TimeManager::new();
    let mut config = config::ConfigReader::new("alarm.conf");
    let alarm = alarm::Alarm::new();

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
