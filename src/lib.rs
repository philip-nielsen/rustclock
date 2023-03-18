mod utils;

use chrono::{DateTime, Local};
use std::time::SystemTime;
use std::{collections::btree_map::Range, panic};

use console_error_panic_hook;

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

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
extern "C" {
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

    let width = web_sys::window()
        .unwrap()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap();
    let circle_size = (width / 10.0) as u32;
    let num_rows = 3;

    container.set_inner_html(""); // clear the container before adding new circles

    // center the container horizontally and align it to the top
    container
        .set_attribute(
            "style",
            "display: flex; justify-content: center; align-items: flex-start; flex-wrap: wrap;",
        )
        .unwrap();
    let normal_range = 0..2;

    for row in (0..num_rows) {
        let row_container = document.create_element("div").unwrap();
        row_container
            .set_attribute("style", "display: flex;")
            .unwrap();

        let new_range = normal_range.clone().map(|x| x + row as u64);
        let num_circles = if row == 0 { 5 } else { 6 };
        let bitRange = if row == 0 {
            0..num_circles - 3
        } else {
            0..num_circles - 2
        };

        for i in new_range.clone() {
            for j in bitRange.clone() {
                let bit = (binary_time[i as usize] >> (3 - j)) & 1;

                let circle = document.create_element("div").unwrap();
                //Wrong, not working
                let color = if bit != 0 {
                    "#2e2e2e;"
                } else if (j % 2 == 0) {
                    "#96c"
                } else {
                    "#f280a1"
                };
                circle.set_attribute("style", &format!("background-color: {}; border-radius: 50%; width: {}px; height: {}px; display: inline-block; margin: {}px;", color, circle_size, circle_size, circle_size/5)).unwrap();
                row_container.append_child(&circle).unwrap();
            }
        }

        container.append_child(&row_container).unwrap();
    }
}

#[wasm_bindgen]
pub fn get_and_output_binary_time() {
    let binary_time = get_binary_time();
    output_binary_clock(binary_time);
}
