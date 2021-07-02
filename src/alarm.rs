extern crate msgbox;

use crate::timemanager::TimeManager;
use crate::config::ConfigReader;
use crate::config::Config;
use msgbox::IconType;

fn message_box(contents: &str) {
    // Message box and borrow string contents
    match msgbox::create("Alarm", contents, IconType::Info) {
        Ok(()) => (),
        Err(msgbox::MsgBoxError::Create(_)) => {
            println!("Failed to create messagebox!");
        }
    }
}

pub fn alarm_system(config: &Config, data: &ConfigReader, time: &TimeManager) {
    for i in 0..data.contents.len() {
        if &config.days[i] == time.get_day() &&
           &config.hours[i] == time.get_hours() &&
           &config.minutes[i] == time.get_minutes() &&
           &config.meridians[i] == time.get_meridian() {
                message_box(&config.alarm_messages[i]);
            }
    }
}
