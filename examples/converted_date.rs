use chrono::{Datelike, Local};

use tamriel_calendar::*;

pub fn main() {
    let current_time = Local::now();

    let tamriel_month = convert_month(current_time);
    let tamriel_year = convert_year(current_time);

    println!("{} {} {}", current_time.day(), tamriel_month, tamriel_year);
}
