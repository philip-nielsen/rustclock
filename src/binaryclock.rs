use chrono::{DateTime, Local};

fn get_binary_time() -> [u64; 6] {
    let local_time: DateTime<Local> = Local::now();
    let midnight = local_time.date().and_hms(0, 0, 0).with_timezone(&Local);
    let time_since_midnight = local_time.signed_duration_since(midnight).num_seconds() as u64;
    let hours = time_since_midnight / 3600;
    let minutes = (time_since_midnight % 3600) / 60;
    let seconds = time_since_midnight % 60;

    let mut binary_time = [0; 6];
    binary_time[0] = hours / 16;
    binary_time[1] = hours % 16;
    binary_time[2] = minutes / 16;
    binary_time[3] = minutes % 16;
    binary_time[4] = seconds / 16;
    binary_time[5] = seconds % 16;

    return binary_time;
}

fn print_binary_clock(binary_time: [u64; 6]) {
    let color = "\x1b[35m";

    for i in 0..2 {
        for j in 0..4 {
            let bit = (binary_time[i] >> (3 - j)) & 1;
            print!("{}{} \x1b[0m", color, if bit == 1 { "██" } else { "  " });
        }
        if i == 1 { println!("\n") };
    }

    for i in 2..4 {
        for j in 0..4 {
            let bit = (binary_time[i] >> (3 - j)) & 1;
            print!("{}{} \x1b[0m", color, if bit == 1 { "██" } else { "  " });
        }
        if i == 3 { println!("\n") };
    }

    for i in 4..6 {
        for j in 0..4 {
            let bit = (binary_time[i] >> (3 - j)) & 1;
            print!("{}{} \x1b[0m", color, if bit == 1 { "██" } else { "  " });
        }
    }

    println!();
}

pub fn print_and_get_binaryclock() {
    let binary_time = get_binary_time();
    print_binary_clock(binary_time);
}