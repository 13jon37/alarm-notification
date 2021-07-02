mod timemanager;
mod config;
mod alarm;

use std::{thread, time};

fn main() {
    let mut time   = timemanager::TimeManager::new();
    let mut config_reader = config::ConfigReader::new("alarm.conf");
    let mut config = config::Config::new();

    // Check the current time seconds and wait so we can get in sync
    if timemanager::get_current_seconds() > 0 {
        thread::sleep(time::Duration::from_secs(60 - timemanager::get_current_seconds() as u64));
    }

    loop {
        // Start time manager and read the config
        time.get_time();
        config_reader.run(&mut config);

        // Alarm system
        alarm::alarm_system(&config, &config_reader, &time);

        thread::sleep(time::Duration::from_secs(60));
    }
}
