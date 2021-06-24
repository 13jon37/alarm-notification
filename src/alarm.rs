extern crate msgbox;

//mod timemanager;
//mod config;

use crate::timemanager::TimeManager;
use crate::config::ConfigReader;
use msgbox::IconType;

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

    fn message_box(contents: &str) {
        // Message box and borrow string contents
        match msgbox::create("Alarm", contents, IconType::Info) {
            Ok(()) => (),
            Err(msgbox::MsgBoxError::Create(_)) => {
                println!("Failed to create messagebox!");
            }
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
            thu  => Alarm::message_box(time.get_day()),
        }

        // Have match statements here
    }
}
