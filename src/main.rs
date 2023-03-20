use binaryclock::print_and_get_binaryclock;
use std::process::Command;

mod binaryclock;

fn main() {
    loop {
        print_and_get_binaryclock();
        println!("Rust är nice");
        println!("Bättre än Stagrims klocka");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        if cfg!(target_os = "windows") {
            Command::new("cls").status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        };
    }
}