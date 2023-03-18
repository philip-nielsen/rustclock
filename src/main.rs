use binaryclock::get_binary_time;
use chrono::{DateTime, Datelike, Timelike, Local, Duration};
use std::time::{SystemTime};

mod binaryclock;

fn main() {
    loop {
        let current_time = SystemTime::now();
        let binary_time = get_binary_time(current_time);
        
        //print_binary_clock(binary_time);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}