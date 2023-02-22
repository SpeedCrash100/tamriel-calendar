use chrono::Datelike;

const MONTH_MAPPING: [&str; 12] = [
    "Morning Star",
    "Sun's Dawn",
    "First Seed",
    "Rain's Hand",
    "Second Seed",
    "Mid Year",
    "Sun's Height",
    "Last Seed",
    "Hearthfire",
    "Frostfall",
    "Sun's Dusk",
    "Evening Star",
];

pub fn convert_year<D: Datelike>(date: D) -> String {
    let year = date.year();
    let epoch = year / 1000;
    let sub_year = year % 1000;

    format!("{}E {}", epoch, sub_year)
}

pub fn convert_month<D: Datelike>(date: D) -> String {
    let month = date.month0() as usize;
    String::from(MONTH_MAPPING[month])
}
