mod utils;

use std::panic;
use std::time::SystemTime;
use chrono::{DateTime, Local};

use console_error_panic_hook;

use web_sys::{Document, Element};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rustclock2!");
}



pub fn get_binary_time() -> [u64; 6] {
    let local_time: DateTime<Local> = Local::now();
    let midnight = local_time.date().and_hms(0, 0, 0).with_timezone(&Local);
    let time_since_midnight = local_time.signed_duration_since(midnight).num_seconds() as u64;
    let hours = time_since_midnight / 3600;
    let minutes = (time_since_midnight % 3600) / 60;
    let seconds = time_since_midnight % 60;

    let mut binary_time = [0; 6];
    binary_time[0] = hours / 10;
    binary_time[1] = hours % 10;
    binary_time[2] = minutes / 10;
    binary_time[3] = minutes % 10;
    binary_time[4] = seconds / 10;
    binary_time[5] = seconds % 10;

    return binary_time;
}

pub fn output_binary_clock(binary_time: [u64; 6]) {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.get_element_by_id("container").unwrap();

    let width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap();
    let circle_size = (width / 10.0) as u32;
    let num_rows = 3;

    container.set_inner_html(""); // clear the container before adding new circles

    // center the container horizontally and align it to the top
    container.set_attribute("style", "display: flex; justify-content: center; align-items: flex-start; flex-wrap: wrap;").unwrap();

    for row in (0..num_rows) {
        let row_container = document.create_element("div").unwrap();
        row_container.set_attribute("style", "display: flex;").unwrap();

        let num_circles = if row == 0 { 5 } else { 6 };

        for col in 0..num_circles {
            let circle = document.create_element("div").unwrap();
            //Wrong, not working
            let color = if (binary_time[row*2+col/3] & (1 << (col%3))) != 0 { "#2e2e2e;" } else if (col % 2 == 0) {"#96c"} else { "#f280a1" };
            circle.set_attribute("style", &format!("background-color: {}; border-radius: 50%; width: {}px; height: {}px; display: inline-block; margin: {}px;", color, circle_size, circle_size, circle_size/5)).unwrap();
            row_container.append_child(&circle).unwrap();
        }

        container.append_child(&row_container).unwrap();
    }
}





#[wasm_bindgen]
pub fn get_and_output_binary_time() {
    let binary_time = get_binary_time();
    output_binary_clock(binary_time);
}